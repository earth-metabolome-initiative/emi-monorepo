//! Submodule providing methods populating the [`ExternalType`] struct with
//! several numeric types from the `core` crate.

use crate::structs::ExternalType;

/// Returns a vector containing all the numeric types provided by the `core`
/// crate.
pub(super) fn all_types() -> [ExternalType; 8] {
    [
        ExternalType::i16(),
        ExternalType::i32(),
        ExternalType::u32(),
        ExternalType::u64(),
        ExternalType::i64(),
        ExternalType::f32(),
        ExternalType::f64(),
        ExternalType::bool(),
    ]
}

impl ExternalType {
    /// Returns a `ExternalType` instance describing the `i16` type from the
    /// `core` crate.
    fn i16() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::SmallInt), syn::parse_quote!(i16))
            .postgres_types(["int2", "smallint"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .into()
    }

    /// Returns a `ExternalType` instance describing the `i32` type from the
    /// `core` crate.
    fn i32() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::Integer), syn::parse_quote!(i32))
            .postgres_types(["int4", "cardinal_number", "integer", "int"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .into()
    }

    /// Returns a `ExternalType` instance describing the `u32` type from the
    /// `core` crate.
    fn u32() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::Oid), syn::parse_quote!(u32))
            .postgres_types(["oid", "regproc", "xid", "regtype"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .into()
    }

    /// Returns a `ExternalType` instance describing the `u64` type from the
    /// `core` crate.
    fn u64() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::PgLsn), syn::parse_quote!(u64))
            .postgres_types(["pg_lsn"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .into()
    }

    /// Returns a `ExternalType` instance describing the `i64` type from the
    /// `core` crate.
    fn i64() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::BigInt), syn::parse_quote!(i64))
            .postgres_types(["int8", "bigint"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .into()
    }

    /// Returns a `ExternalType` instance describing the `f32` type from the
    /// `core` crate.
    fn f32() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::Float), syn::parse_quote!(f32))
            .postgres_types(["float4", "real"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_partial_eq()
            .supports_partial_ord()
            .into()
    }

    /// Returns a `ExternalType` instance describing the `f64` type from the
    /// `core` crate.
    fn f64() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::Double), syn::parse_quote!(f64))
            .postgres_types(["float8", "double precision", "numeric"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_partial_eq()
            .supports_partial_ord()
            .into()
    }

    /// Returns a `ExternalType` instance describing the `bool` type from the
    /// `core` crate.
    fn bool() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::Bool), syn::parse_quote!(bool))
            .postgres_types(["bool", "boolean"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .into()
    }
}
