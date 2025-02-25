use std::convert::TryFrom;

use proc_macro2::Ident;
use syn::{spanned::Spanned, Attribute, Error, Expr, ExprLit, Lit, Meta, Result};

macro_rules! syn_err {
    ($l:literal $(, $a:expr)*) => {
        syn_err!(proc_macro2::Span::call_site(); $l $(, $a)*)
    };
    ($s:expr; $l:literal $(, $a:expr)*) => {
        return Err(syn::Error::new($s, format!($l $(, $a)*)))
    };
}

macro_rules! impl_parse {
    ($i:ident ($input:ident, $out:ident) { $($k:pat => $e:expr),* $(,)? }) => {
        impl std::convert::TryFrom<&syn::Attribute> for $i {
            type Error = syn::Error;

            fn try_from(attr: &syn::Attribute) -> syn::Result<Self> { attr.parse_args() }
        }

        impl syn::parse::Parse for $i {
            fn parse($input: syn::parse::ParseStream) -> syn::Result<Self> {
                let mut $out = $i::default();
                loop {
                    let key: Ident = $input.call(syn::ext::IdentExt::parse_any)?;
                    match &*key.to_string() {
                        $($k => $e,)*
                        #[allow(unreachable_patterns)]
                        _ => syn_err!($input.span(); "unexpected attribute")
                    }

                    match $input.is_empty() {
                        true => break,
                        false => {
                            $input.parse::<syn::Token![,]>()?;
                        }
                    }
                }

                Ok($out)
            }
        }
    };
}

/// Converts a rust identifier to a typescript identifier.
pub fn to_ts_ident(ident: &Ident) -> String {
    let ident = ident.to_string();
    if ident.starts_with("r#") {
        ident.trim_start_matches("r#").to_owned()
    } else {
        ident
    }
}

/// Convert an arbitrary name to a valid Typescript field name.
///
/// If the name contains special characters it will be wrapped in quotes.
pub fn raw_name_to_ts_field(value: String) -> String {
    let valid = value
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
        && value
            .chars()
            .next()
            .map(|first| !first.is_numeric())
            .unwrap_or(true);
    if !valid {
        format!(r#""{value}""#)
    } else {
        value
    }
}

/// Parse all `#[ts(..)]` attributes from the given slice.
pub fn parse_attrs<'a, A>(attrs: &'a [Attribute]) -> Result<impl Iterator<Item = A>>
where
    A: TryFrom<&'a Attribute, Error = Error>,
{
    Ok(attrs
        .iter()
        .filter(|a| a.path().is_ident("ts"))
        .map(A::try_from)
        .collect::<Result<Vec<A>>>()?
        .into_iter())
}

/// Parse all `#[serde(..)]` attributes from the given slice.
#[cfg(feature = "serde-compat")]
#[allow(unused)]
pub fn parse_serde_attrs<'a, A: TryFrom<&'a Attribute, Error = Error>>(
    attrs: &'a [Attribute],
) -> impl Iterator<Item = A> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("serde"))
        .flat_map(|attr| match A::try_from(attr) {
            Ok(attr) => Some(attr),
            Err(_) => {
                #[cfg(not(feature = "no-serde-warnings"))]
                use quote::ToTokens;

                #[cfg(not(feature = "no-serde-warnings"))]
                warning::print_warning(
                    "failed to parse serde attribute",
                    format!("{}", attr.to_token_stream()),
                    "ts-rs failed to parse this attribute. It will be ignored.",
                )
                .unwrap();
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
}

/// Return doc comments parsed and formatted as JSDoc.
pub fn parse_docs(attrs: &[Attribute]) -> Result<String> {
    let lines = attrs
        .iter()
        .filter_map(|a| match a.meta {
            Meta::NameValue(ref x) if x.path.is_ident("doc") => Some(x),
            _ => None,
        })
        .map(|attr| match attr.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(ref str),
                ..
            }) => Ok(str.value()),
            _ => syn_err!(attr.span(); "doc attribute with non literal expression found"),
        })
        .map(|attr| {
            attr.map(|line| match line.trim() {
                "" => " *".to_owned(),
                _ => format!(" *{}", line.trim_end())
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(match lines.is_empty() {
        true => "".to_owned(),
        false => format!("/**\n{}\n */\n", lines.join("\n")),
    })
}

#[cfg(feature = "serde-compat")]
mod warning {
    use std::{fmt::Display, io::Write};

    use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

    // Sadly, it is impossible to raise a warning in a proc macro.
    // This function prints a message which looks like a compiler warning.
    #[allow(unused)]
    pub fn print_warning(
        title: impl Display,
        content: impl Display,
        note: impl Display,
    ) -> std::io::Result<()> {
        let make_color = |color: Color, bold: bool| {
            let mut spec = ColorSpec::new();
            spec.set_fg(Some(color)).set_bold(bold).set_intense(true);
            spec
        };

        let yellow_bold = make_color(Color::Yellow, true);
        let white_bold = make_color(Color::White, true);
        let white = make_color(Color::White, false);
        let blue = make_color(Color::Blue, true);

        let writer = BufferWriter::stderr(ColorChoice::Auto);
        let mut buffer = writer.buffer();

        buffer.set_color(&yellow_bold)?;
        write!(&mut buffer, "warning")?;
        buffer.set_color(&white_bold)?;
        writeln!(&mut buffer, ": {}", title)?;

        buffer.set_color(&blue)?;
        writeln!(&mut buffer, "  | ")?;

        write!(&mut buffer, "  | ")?;
        buffer.set_color(&white)?;
        writeln!(&mut buffer, "{}", content)?;

        buffer.set_color(&blue)?;
        writeln!(&mut buffer, "  | ")?;

        write!(&mut buffer, "  = ")?;
        buffer.set_color(&white_bold)?;
        write!(&mut buffer, "note: ")?;
        buffer.set_color(&white)?;
        writeln!(&mut buffer, "{}", note)?;

        writer.print(&buffer)
    }
}
