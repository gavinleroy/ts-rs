//! <h1 align="center" style="padding-top: 0; margin-top: 0;">
//! <img width="150px" src="https://raw.githubusercontent.com/Aleph-Alpha/ts-rs/main/logo.png" alt="logo">
//! <br/>
//! ts-rs
//! </h1>
//! <p align="center">
//! generate typescript interface/type declarations from rust types
//! </p>
//!
//! <div align="center">
//! <!-- Github Actions -->
//! <img src="https://img.shields.io/github/actions/workflow/status/Aleph-Alpha/ts-rs/test.yml?branch=main" alt="actions status" />
//! <a href="https://crates.io/crates/ts-rs">
//! <img src="https://img.shields.io/crates/v/ts-rs.svg?style=flat-square"
//! alt="Crates.io version" />
//! </a>
//! <a href="https://docs.rs/ts-rs">
//! <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//! alt="docs.rs docs" />
//! </a>
//! <a href="https://crates.io/crates/ts-rs">
//! <img src="https://img.shields.io/crates/d/ts-rs.svg?style=flat-square"
//! alt="Download" />
//! </a>
//! </div>
//!
//! ## why?
//! When building a web application in rust, data structures have to be shared between backend and frontend.
//! Using this library, you can easily generate TypeScript bindings to your rust structs & enums so that you can keep your
//! types in one place.
//!
//! ts-rs might also come in handy when working with webassembly.
//!
//! ## how?
//! ts-rs exposes a single trait, `TS`. Using a derive macro, you can implement this interface for your types.
//! Then, you can use this trait to obtain the TypeScript bindings.
//! We recommend doing this in your tests.
//! [See the example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs) and [the docs](https://docs.rs/ts-rs/latest/ts_rs/).
//!
//! ## get started
//! ```toml
//! [dependencies]
//! ts-rs = "7.1"
//! ```
//!
//! ```rust
//! use ts_rs::TS;
//!
//! #[derive(TS)]
//! #[ts(export)]
//! struct User {
//!     user_id: i32,
//!     first_name: String,
//!     last_name: String,
//! }
//! ```
//! When running `cargo test`, the TypeScript bindings will be exported to the file `bindings/User.ts`.
//!
//! ## features
//! - generate interface declarations from rust structs
//! - generate union declarations from rust enums
//! - inline types
//! - flatten structs/interfaces
//! - generate necessary imports when exporting to multiple files
//! - serde compatibility
//! - generic types
//! - support for ESM imports
//!
//! ## limitations
//! - generic fields cannot be inlined or flattened (#56)
//! - type aliases must not alias generic types (#70)
//!
//! ## cargo features
//! - `serde-compat` (default)  
//!
//!   Enable serde compatibility. See below for more info.  
//! - `format`
//!
//!   When enabled, the generated typescript will be formatted.
//!   Currently, this sadly adds quite a bit of dependencies.
//! - `chrono-impl`  
//!
//!   Implement `TS` for types from chrono  
//! - `bigdecimal-impl`  
//!
//!   Implement `TS` for types from bigdecimal  
//! - `url-impl`  
//!
//!   Implement `TS` for types from url
//! - `uuid-impl`  
//!
//!   Implement `TS` for types from uuid
//! - `bson-uuid-impl`
//!
//!   Implement `TS` for types from bson
//! - `bytes-impl`
//!
//!   Implement `TS` for types from bytes    
//! - `indexmap-impl`  
//!
//!   Implement `TS` for `IndexMap` and `IndexSet` from indexmap
//! - `index_vec-impl`
//!
//!   Implement `TS` for `IndexVec` from index_vec
//! - `ordered-float-impl`
//!
//!   Implement `TS` for `OrderedFloat` from ordered_float
//!
//! - `heapless-impl`  
//!
//!   Implement `TS` for `Vec` from heapless
//!
//! - `semver-impl`  
//!   Implement `TS` for `Version` from semver
//!
//! - `no-serde-warnings`
//!
//!   When `serde-compat` is enabled, warnings are printed during build if unsupported serde
//!   attributes are encountered. Enabling this feature silences these warnings.
//!
//! - `import-esm`
//!
//!   `import` statements in the generated file will have the `.js` extension in the end of
//!   the path to conform to the ES Modules spec. (e.g.: `import { MyStruct } from "./my_struct.js"`)
//!
//! If there's a type you're dealing with which doesn't implement `TS`, use `#[ts(type = "..")]` or open a PR.
//!
//! ## serde compatability
//! With the `serde-compat` feature (enabled by default), serde attributes can be parsed for enums and structs.
//! Supported serde attributes:
//! - `rename`
//! - `rename-all`
//! - `rename-all-fields`
//! - `tag`
//! - `content`
//! - `untagged`
//! - `skip`
//! - `flatten`
//! - `default`
//!
//! Note: `skip_serializing` and `skip_deserializing` are ignored. If you wish to exclude a field
//! from the generated type, but cannot use `#[serde(skip)]`, use `#[ts(skip)]` instead.
//!
//! When ts-rs encounters an unsupported serde attribute, a warning is emitted, unless the feature `no-serde-warnings` is enabled.
//!
//! ## contributing
//! Contributions are always welcome!
//! Feel free to open an issue, discuss using GitHub discussions or open a PR.
//! [See CONTRIBUTING.md](https://github.com/Aleph-Alpha/ts-rs/blob/main/CONTRIBUTING.md)
//!
//! ## todo
//! - [x] serde compatibility layer
//! - [x] documentation
//! - [x] use typescript types across files
//! - [x] more enum representations
//! - [x] generics
//! - [x] don't require `'static`

use std::{
    any::TypeId,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Range, RangeInclusive},
    path::{Path, PathBuf},
};

pub use ts_rs_macros::TS;

pub use crate::export::ExportError;
use crate::typelist::TypeList;

// Used in generated code. Not public API
#[doc(hidden)]
pub use crate::export::__private;

#[cfg(feature = "chrono-impl")]
mod chrono;
mod export;
pub mod typelist;

/// A type which can be represented in TypeScript.  
/// Most of the time, you'd want to derive this trait instead of implementing it manually.  
/// ts-rs comes with implementations for all primitives, most collections, tuples,
/// arrays and containers.
///
/// ### exporting
/// Because Rusts procedural macros are evaluated before other compilation steps, TypeScript
/// bindings cannot be exported during compile time.
/// Bindings can be exported within a test, which ts-rs generates for you by adding `#[ts(export)]`
/// to a type you wish to export to a file.
/// If, for some reason, you need to do this during runtime, you can call [`TS::export`] yourself.
///
/// ### serde compatibility
/// By default, the feature `serde-compat` is enabled.
/// ts-rs then parses serde attributes and adjusts the generated typescript bindings accordingly.
/// Not all serde attributes are supported yet - if you use an unsupported attribute, you'll see a
/// warning.
///
/// ### container attributes
/// attributes applicable for both structs and enums
///
/// - `#[ts(export)]`:  
///   Generates a test which will export the type, by default to `bindings/<name>.ts` when running
///   `cargo test`. The default base directory can be overridden with the `TS_RS_EXPORT_DIR` environment variable.
///   Adding the variable to a project's [config.toml](https://doc.rust-lang.org/cargo/reference/config.html#env) can
///   make it easier to manage.
/// ```toml
/// # <project-root>/.cargo/config.toml
/// [env]
/// TS_RS_EXPORT_DIR = { value = "<OVERRIDE_DIR>", relative = true }
/// ```
///
/// - `#[ts(export_to = "..")]`:  
///   Specifies where the type should be exported to. Defaults to `bindings/<name>.ts`.  
///   The `export_to` attribute will also override the `TS_RS_EXPORT_DIR` environment variable.  
///   If the provided path ends in a trailing `/`, it is interpreted as a directory.   
///   Note that you need to add the `export` attribute as well, in order to generate a test which exports the type.
///
/// - `#[ts(rename = "..")]`:  
///   Sets the typescript name of the generated type
///
/// - `#[ts(rename_all = "..")]`:  
///   Rename all fields/variants of the type.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case"
///
///
/// ### struct field attributes
///
/// - `#[ts(type = "..")]`:  
///   Overrides the type used in TypeScript.  
///   This is useful when there's a type for which you cannot derive `TS`.  
///
/// - `#[ts(rename = "..")]`:  
///   Renames this field  
///
/// - `#[ts(inline)]`:  
///   Inlines the type of this field  
///
/// - `#[ts(skip)]`:  
///   Skip this field  
///
/// - `#[ts(optional)]`:  
///   May be applied on a struct field of type `Option<T>`.
///   By default, such a field would turn into `t: T | null`.
///   If `#[ts(optional)]` is present, `t?: T` is generated instead.
///   If `#[ts(optional = nullable)]` is present, `t?: T | null` is generated.
///
/// - `#[ts(flatten)]`:  
///   Flatten this field
///   
/// ### enum attributes
///
/// - `#[ts(tag = "..")]`:  
///   Changes the representation of the enum to store its tag in a separate field.
///   See [the serde docs](https://serde.rs/enum-representations.html).
///
/// - `#[ts(content = "..")]`:  
///   Changes the representation of the enum to store its content in a separate field.
///   See [the serde docs](https://serde.rs/enum-representations.html).
///
/// - `#[ts(untagged)]`:  
///   Changes the representation of the enum to not include its tag.
///   See [the serde docs](https://serde.rs/enum-representations.html).
///
/// - `#[ts(rename_all = "..")]`:  
///   Rename all variants of this enum.  
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case"
///
/// - `#[ts(rename_all_fieds = "..")]`
///   Renames the fields of all the struct variants of this enum.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case"
///  
/// ### enum variant attributes
///
/// - `#[ts(rename = "..")]`:  
///   Renames this variant  
///
/// - `#[ts(skip)]`:  
///   Skip this variant  
pub trait TS {
    const EXPORT_TO: Option<&'static str> = None;
    const DOCS: Option<&'static str> = None;

    fn get_export_to() -> Option<String> {
        Self::EXPORT_TO.map(ToString::to_string)
    }

    /// Declaration of this type, e.g. `interface User { user_id: number, ... }`.
    /// This function will panic if the type has no declaration.
    fn decl() -> String {
        panic!("{} cannot be declared", Self::name());
    }

    /// Name of this type in TypeScript.
    fn name() -> String;

    /// Name of this type in TypeScript, with type arguments.
    fn name_with_type_args(args: Vec<String>) -> String {
        format!("{}<{}>", Self::name(), args.join(", "))
    }

    /// Formats this types definition in TypeScript, e.g `{ user_id: number }`.
    /// This function will panic if the type cannot be inlined.
    fn inline() -> String {
        panic!("{} cannot be inlined", Self::name());
    }

    /// Flatten an type declaration.  
    /// This function will panic if the type cannot be flattened.
    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }

    fn dependency_types() -> impl TypeList
    where
        Self: 'static,
    {
    }

    fn dependencies() -> Vec<Dependency>
    where
        Self: 'static,
    {
        use crate::typelist::TypeVisitor;

        let mut deps: Vec<Dependency> = vec![];
        struct Visit<'a>(&'a mut Vec<Dependency>);
        impl<'a> TypeVisitor for Visit<'a> {
            fn visit<T: TS + 'static + ?Sized>(&mut self) {
                if let Some(dep) = Dependency::from_ty::<T>() {
                    self.0.push(dep);
                }
            }
        }
        Self::dependency_types().for_each(&mut Visit(&mut deps));

        deps
    }

    /// `true` if this is a transparent type, e.g tuples or a list.
    /// This is used for resolving imports when using the `export!` macro.
    fn transparent() -> bool;

    /// Manually export this type to a file.
    /// The output file can be specified by annotating the type with `#[ts(export_to = ".."]`.
    /// By default, the filename will be derived from the types name.
    ///
    /// When a type is annotated with `#[ts(export)]`, it is exported automatically within a test.
    /// This function is only usefull if you need to export the type outside of the context of a
    /// test.
    fn export() -> Result<(), ExportError>
    where
        Self: 'static,
    {
        export::export_type_with_dependencies::<Self>()
    }

    /// Manually export this type to a file with a file with the specified path. This
    /// function will ignore the `#[ts(export_to = "..)]` attribute.
    fn export_to(path: impl AsRef<Path>) -> Result<(), ExportError>
    where
        Self: 'static,
    {
        export::export_type_to::<Self, _>(path)
    }

    /// Manually generate bindings for this type, returning a [`String`].  
    /// This function does not format the output, even if the `format` feature is enabled.
    fn export_to_string() -> Result<String, ExportError>
    where
        Self: 'static,
    {
        export::export_type_to_string::<Self>()
    }
}

/// A typescript type which is depended upon by other types.
/// This information is required for generating the correct import statements.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dependency {
    /// Type ID of the rust type
    pub type_id: TypeId,
    /// Name of the type in TypeScript
    pub ts_name: String,
    /// Path to where the type would be exported. By default a filename is derived from the types
    /// name, which can be customized with `#[ts(export_to = "..")]`.
    pub exported_to: String,
}

impl Dependency {
    /// Constructs a [`Dependency`] from the given type `T`.
    /// If `T` is not exportable (meaning `T::EXPORT_TO` is `None`), this function will return
    /// `None`
    pub fn from_ty<T: TS + 'static + ?Sized>() -> Option<Self> {
        let exported_to = T::get_export_to()?;
        Some(Dependency {
            type_id: TypeId::of::<T>(),
            ts_name: T::name(),
            exported_to,
        })
    }
}

// generate impls for primitive types
macro_rules! impl_primitives {
    ($($($ty:ty),* => $l:literal),*) => { $($(
        impl TS for $ty {
            fn name() -> String { $l.to_owned() }
            fn name_with_type_args(args: Vec<String>) -> String {
                assert!(args.is_empty(), "called name_with_type_args on primitive");
                $l.to_owned()
            }
            fn inline() -> String { $l.to_owned() }
            fn transparent() -> bool { false }
        }
    )*)* };
}
// generate impls for tuples
macro_rules! impl_tuples {
    ( impl $($i:ident),* ) => {
        impl<$($i: TS),*> TS for ($($i,)*) {
            fn name() -> String {
                format!("[{}]", [$($i::name()),*].join(", "))
            }
            fn inline() -> String {
                format!("[{}]", [$($i::inline()),*].join(", "))
            }
            fn dependency_types() -> impl TypeList
            where
                Self: 'static
            {
                ()$(.push::<$i>())*
            }
            fn transparent() -> bool { true }
        }
    };
    ( $i2:ident $(, $i:ident)* ) => {
        impl_tuples!(impl $i2 $(, $i)* );
        impl_tuples!($($i),*);
    };
    () => {};
}

// generate impls for wrapper types
macro_rules! impl_wrapper {
    ($($t:tt)*) => {
        $($t)* {
            fn name() -> String { T::name() }
            fn name_with_type_args(mut args: Vec<String>) -> String {
                assert_eq!(args.len(), 1);
                args.remove(0)
            }
            fn inline() -> String { T::inline() }
            fn inline_flattened() -> String { T::inline_flattened() }
            fn dependency_types() -> impl TypeList
            where
                Self: 'static
            {
                T::dependency_types()
            }
            fn transparent() -> bool { T::transparent() }
        }
    };
}

// implement TS for the $shadow, deferring to the impl $s
macro_rules! impl_shadow {
    (as $s:ty: $($impl:tt)*) => {
        $($impl)* {
            fn name() -> String { <$s>::name() }
            fn name_with_type_args(args: Vec<String>) -> String { <$s>::name_with_type_args(args) }
            fn inline() -> String { <$s>::inline() }
            fn inline_flattened() -> String { <$s>::inline_flattened() }
            fn dependency_types() -> impl $crate::typelist::TypeList
            where
                Self: 'static
            {
                <$s>::dependency_types()
            }
            fn transparent() -> bool { <$s>::transparent() }
        }
    };
}

impl<T: TS> TS for Option<T> {
    fn name() -> String {
        unreachable!();
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            1,
            "called Option::name_with_type_args with {} args",
            args.len()
        );
        format!("{} | null", args[0])
    }

    fn inline() -> String {
        format!("{} | null", T::inline())
    }

    fn dependency_types() -> impl TypeList
    where
        Self: 'static,
    {
        ().push::<T>()
    }

    fn transparent() -> bool {
        true
    }
}

impl<T: TS, E: TS> TS for Result<T, E> {
    fn name() -> String {
        unreachable!();
    }
    fn inline() -> String {
        format!("{{ Ok : {} }} | {{ Err : {} }}", T::inline(), E::inline())
    }
    fn dependency_types() -> impl TypeList
    where
        Self: 'static,
    {
        ().push::<T>().push::<E>()
    }
    fn transparent() -> bool {
        true
    }
}

impl<T: TS> TS for Vec<T> {
    fn name() -> String {
        "Array".to_owned()
    }

    fn inline() -> String {
        format!("Array<{}>", T::inline())
    }

    fn dependency_types() -> impl TypeList
    where
        Self: 'static,
    {
        ().push::<T>()
    }
    fn transparent() -> bool {
        true
    }
}

// Arrays longer than this limit will be emitted as Array<T>
const ARRAY_TUPLE_LIMIT: usize = 64;
impl<T: TS, const N: usize> TS for [T; N] {
    fn name() -> String {
        if N > ARRAY_TUPLE_LIMIT {
            return Vec::<T>::name();
        }

        "[]".to_owned()
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        if N > ARRAY_TUPLE_LIMIT {
            return Vec::<T>::name_with_type_args(args);
        }

        assert_eq!(
            args.len(),
            1,
            "called [T; N]::name_with_type_args with {} args",
            args.len()
        );

        format!(
            "[{}]",
            (0..N)
                .map(|_| args[0].clone())
                .collect::<Box<[_]>>()
                .join(", ")
        )
    }

    fn inline() -> String {
        if N > ARRAY_TUPLE_LIMIT {
            return Vec::<T>::inline();
        }

        format!(
            "[{}]",
            (0..N).map(|_| T::inline()).collect::<Box<[_]>>().join(", ")
        )
    }

    fn dependency_types() -> impl TypeList
    where
        Self: 'static,
    {
        ().push::<T>()
    }

    fn transparent() -> bool {
        true
    }
}

impl<K: TS, V: TS, H> TS for HashMap<K, V, H> {
    fn name() -> String {
        "Record".to_owned()
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            2,
            "called HashMap::name_with_type_args with {} args",
            args.len()
        );
        format!("Record<{}, {}>", args[0], args[1])
    }

    fn inline() -> String {
        format!("Record<{}, {}>", K::inline(), V::inline())
    }

    fn dependency_types() -> impl TypeList
    where
        Self: 'static,
    {
        ().push::<K>().push::<V>()
    }

    fn transparent() -> bool {
        true
    }
}

impl<I: TS> TS for Range<I> {
    fn name() -> String {
        panic!("called Range::name - Did you use a type alias?")
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            1,
            "called Range::name_with_type_args with {} args",
            args.len()
        );
        format!("{{ start: {}, end: {}, }}", &args[0], &args[0])
    }

    fn dependency_types() -> impl TypeList
    where
        Self: 'static,
    {
        ().push::<I>()
    }

    fn transparent() -> bool {
        true
    }
}

impl<I: TS> TS for RangeInclusive<I> {
    fn name() -> String {
        panic!("called RangeInclusive::name - Did you use a type alias?")
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            1,
            "called RangeInclusive::name_with_type_args with {} args",
            args.len()
        );
        format!("{{ start: {}, end: {}, }}", &args[0], &args[0])
    }

    fn dependency_types() -> impl TypeList
    where
        Self: 'static,
    {
        ().push::<I>()
    }

    fn transparent() -> bool {
        true
    }
}

impl_shadow!(as T: impl<T: TS + ?Sized> TS for &T);
impl_shadow!(as Vec<T>: impl<T: TS, H> TS for HashSet<T, H>);
impl_shadow!(as Vec<T>: impl<T: TS> TS for BTreeSet<T>);
impl_shadow!(as HashMap<K, V>: impl<K: TS, V: TS> TS for BTreeMap<K, V>);
impl_shadow!(as Vec<T>: impl<T: TS> TS for [T]);

impl_wrapper!(impl<T: TS + ?Sized> TS for Box<T>);
impl_wrapper!(impl<T: TS + ?Sized> TS for std::sync::Arc<T>);
impl_wrapper!(impl<T: TS + ?Sized> TS for std::rc::Rc<T>);
impl_wrapper!(impl<'a, T: TS + ToOwned + ?Sized> TS for std::borrow::Cow<'a, T>);
impl_wrapper!(impl<T: TS> TS for std::cell::Cell<T>);
impl_wrapper!(impl<T: TS> TS for std::cell::RefCell<T>);
impl_wrapper!(impl<T: TS> TS for std::sync::Mutex<T>);
impl_wrapper!(impl<T: TS + ?Sized> TS for std::sync::Weak<T>);
impl_wrapper!(impl<T: TS> TS for std::marker::PhantomData<T>);

impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

#[cfg(feature = "bigdecimal-impl")]
impl_primitives! { bigdecimal::BigDecimal => "string" }

#[cfg(feature = "uuid-impl")]
impl_primitives! { uuid::Uuid => "string" }

#[cfg(feature = "url-impl")]
impl_primitives! { url::Url => "string" }

#[cfg(feature = "ordered-float-impl")]
impl_primitives! { ordered_float::OrderedFloat<f32> => "number" }

#[cfg(feature = "ordered-float-impl")]
impl_primitives! { ordered_float::OrderedFloat<f64> => "number" }

#[cfg(feature = "bson-uuid-impl")]
impl_primitives! { bson::Uuid => "string" }

#[cfg(feature = "indexmap-impl")]
impl_shadow!(as Vec<T>: impl<T: TS> TS for indexmap::IndexSet<T>);

#[cfg(feature = "indexmap-impl")]
impl_shadow!(as HashMap<K, V>: impl<K: TS, V: TS> TS for indexmap::IndexMap<K, V>);

#[cfg(feature = "heapless-impl")]
impl_shadow!(as Vec<T>: impl<T: TS, const N: usize> TS for heapless::Vec<T, N>);

#[cfg(feature = "index_vec-impl")]
impl_shadow!(as Vec<T>: impl<K: index_vec::Idx, T: TS> TS for index_vec::IndexVec<K, T>);

#[cfg(feature = "semver-impl")]
impl_primitives! { semver::Version => "string" }

#[cfg(feature = "bytes-impl")]
mod bytes {
    use super::TS;

    impl_shadow!(as Vec<u8>: impl TS for bytes::Bytes);
    impl_shadow!(as Vec<u8>: impl TS for bytes::BytesMut);
}

impl_primitives! {
    u8, i8, NonZeroU8, NonZeroI8,
    u16, i16, NonZeroU16, NonZeroI16,
    u32, i32, NonZeroU32, NonZeroI32,
    usize, isize, NonZeroUsize, NonZeroIsize, f32, f64 => "number",
    u64, i64, NonZeroU64, NonZeroI64,
    u128, i128, NonZeroU128, NonZeroI128 => "bigint",
    bool => "boolean",
    char, Path, PathBuf, String, str,
    Ipv4Addr, Ipv6Addr, IpAddr, SocketAddrV4, SocketAddrV6, SocketAddr => "string",
    () => "null"
}
#[rustfmt::skip]
pub(crate) use impl_primitives;
