use quote::quote;
use syn::{FieldsUnnamed, Generics, Result, Type};

use crate::{
    attr::{FieldAttr, StructAttr},
    deps::Dependencies,
    types::generics::{format_generics, format_type},
    DerivedTS,
};

pub(crate) fn newtype(
    attr: &StructAttr,
    name: &str,
    fields: &FieldsUnnamed,
    generics: &Generics,
) -> Result<DerivedTS> {
    if attr.rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to newtype structs");
    }
    if attr.tag.is_some() {
        syn_err!("`tag` is not applicable to newtype structs");
    }
    let inner = fields.unnamed.first().unwrap();
    let FieldAttr {
        type_as,
        type_override,
        rename: rename_inner,
        inline,
        skip,
        optional,
        flatten,
        docs: _,
    } = FieldAttr::from_attrs(&inner.attrs)?;

    match (&rename_inner, skip, optional.optional, flatten) {
        (Some(_), ..) => syn_err!("`rename` is not applicable to newtype fields"),
        (_, true, ..) => return super::unit::null(attr, name),
        (_, _, true, ..) => syn_err!("`optional` is not applicable to newtype fields"),
        (_, _, _, true) => syn_err!("`flatten` is not applicable to newtype fields"),
        _ => {}
    };

    if type_as.is_some() && type_override.is_some() {
        syn_err!("`type` is not compatible with `as`")
    }

    let inner_ty = if let Some(ref type_as) = type_as {
        syn::parse_str::<Type>(type_as)?
    } else {
        inner.ty.clone()
    };

    let mut dependencies = Dependencies::default();

    match (type_override.is_none(), inline) {
        (false, _) => (),
        (true, true) => dependencies.append_from(&inner_ty),
        (true, false) => dependencies.push_or_append_from(&inner_ty),
    };

    let inline_def = match type_override {
        Some(ref o) => quote!(#o.to_owned()),
        None if inline => quote!(<#inner_ty as ts_rs::TS>::inline()),
        None => format_type(&inner_ty, &mut dependencies, generics),
    };

    let generic_args = format_generics(&mut dependencies, generics);
    Ok(DerivedTS {
        decl: quote!(format!("type {}{} = {};", #name, #generic_args, #inline_def)),
        inline: inline_def,
        inline_flattened: None,
        name: name.to_owned(),
        docs: attr.docs.clone(),
        dependencies,
        export: attr.export,
        export_to: attr.export_to.clone(),
    })
}
