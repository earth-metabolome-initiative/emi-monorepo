//! Submodule providing methods populating the [`ExternalType`] struct with
//! several numeric types from the `core` crate.

use common_traits::builder::Builder;

use crate::structs::ExternalType;

/// Returns a vector containing all the numeric types provided by the `core`
/// crate.
pub(super) fn all_types() -> [ExternalType; 7] {
    [
        ExternalType::i16(),
        ExternalType::i32(),
        ExternalType::u32(),
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
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::SmallInt").unwrap())
            .rust_type(syn::parse_str("i16").unwrap())
            .postgres_types(["int2", "smallint"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `i32` type from the
    /// `core` crate.
    fn i32() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Integer").unwrap())
            .rust_type(syn::parse_str("i32").unwrap())
            .postgres_types(["int4", "integer", "int"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `u32` type from the
    /// `core` crate.
    fn u32() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Oid").unwrap())
            .rust_type(syn::parse_str("u32").unwrap())
            .postgres_types(["oid"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `i64` type from the
    /// `core` crate.
    fn i64() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::BigInt").unwrap())
            .rust_type(syn::parse_str("i64").unwrap())
            .postgres_types(["int8", "bigint"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `f32` type from the
    /// `core` crate.
    fn f32() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Float").unwrap())
            .rust_type(syn::parse_str("f32").unwrap())
            .postgres_types(["float4", "real"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_partial_eq()
            .supports_partial_ord()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `f64` type from the
    /// `core` crate.
    fn f64() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Double").unwrap())
            .rust_type(syn::parse_str("f64").unwrap())
            .postgres_types(["float8", "double precision", "numeric"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_partial_eq()
            .supports_partial_ord()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `bool` type from the
    /// `core` crate.
    fn bool() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Bool").unwrap())
            .rust_type(syn::parse_str("bool").unwrap())
            .postgres_types(["bool", "boolean"])
            .unwrap()
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }
}
