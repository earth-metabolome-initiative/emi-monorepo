//! Submodule implementing the method `std` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `std` crate.

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `std` crate.
    pub fn std() -> ExternalCrate {
        ExternalCrate::new("std")
            .unwrap()
            .types([
                ExternalType::string(),
                ExternalType::vec_u8(),
                ExternalType::vec_i16(),
                ExternalType::vec_i32(),
                ExternalType::vec_u32(),
                ExternalType::vec_f32(),
                ExternalType::vec_f64(),
                ExternalType::vec_bool(),
                ExternalType::ip_addr(),
                ExternalType::mac_addr(),
                ExternalType::vec_string(),
                ExternalType::system_time(),
            ])
            .unwrap()
            .into()
    }
}

impl ExternalType {
    /// Returns an `ExternalType` instance describing the `String` type from the
    /// `std` crate.
    fn string() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::Text), syn::parse_quote!(String))
            .postgres_types([
                "text",
                "char",
                "character",
                "character varying",
                "varchar",
                "bpchar",
                "pg_node_tree",
                "name",
                "aclitem",
                "yes_or_no",
                "sql_identifier",
                "character_data",
            ])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .into()
    }

    /// Returns an `ExternalType` instance describing the `Vec<u8>` type from
    /// the `std` crate.
    fn vec_u8() -> Self {
        ExternalType::new(syn::parse_quote!(diesel::sql_types::Binary), syn::parse_quote!(Vec<u8>))
            .postgres_types(["bytea", "bit", "varbit"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .into()
    }

    /// Returns an `ExternalType` instance describing the `Vec<i16>` type from
    /// the `std` crate.
    fn vec_i16() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::SmallInt>),
            syn::parse_quote!(Vec<i16>),
        )
        .postgres_types(["_int2", "int2[]", "_smallint", "int2vector"])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_hash()
        .supports_ord()
        .into()
    }

    /// Returns an `ExternalType` instance describing the `Vec<i32>` type from
    /// the `std` crate.
    fn vec_i32() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Integer>),
            syn::parse_quote!(Vec<i32>),
        )
        .postgres_types(["_int4", "int4[]", "_integer", "_cardinal_number"])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_hash()
        .supports_ord()
        .into()
    }

    /// Returns an `ExternalType` instance describing the `Vec<u32>` type from
    /// the `std` crate.
    fn vec_u32() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Oid>),
            syn::parse_quote!(Vec<u32>),
        )
        .postgres_types(["_oid", "oid[]", "_regtype", "regtype[]", "oidvector"])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_hash()
        .supports_ord()
        .into()
    }

    /// Returns an `ExternalType` instance describing the `Vec<f32>` type from
    /// the `std` crate.
    fn vec_f32() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Float>),
            syn::parse_quote!(Vec<f32>),
        )
        .postgres_types(["_float4", "float4[]", "_real"])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_partial_eq()
        .supports_partial_ord()
        .into()
    }

    /// Returns an `ExternalType` instance describing the `Vec<f64>` type from
    /// the `std` crate.
    fn vec_f64() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Double>),
            syn::parse_quote!(Vec<f64>),
        )
        .postgres_types(["_float8", "float8[]", "_double_precision"])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_partial_eq()
        .supports_partial_ord()
        .into()
    }

    /// Returns an `ExternalType` instance describing the `Vec<bool>` type from
    /// the `std` crate.
    fn vec_bool() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Bool>),
            syn::parse_quote!(Vec<bool>),
        )
        .postgres_types(["_bool", "bool[]"])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_hash()
        .supports_ord()
        .into()
    }

    /// Returns an [`ExternalType`] instance describing the `Vec<String>` type
    /// from the `std` crate.
    fn vec_string() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Text>),
            syn::parse_quote!(Vec<String>),
        )
        .postgres_types([
            "_text",
            "text[]",
            "_character varying",
            "_varchar",
            "_bpchar",
            "_name",
            "_char",
            "_aclitem",
        ])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_hash()
        .supports_ord()
        .into()
    }

    /// Returns an `ExternalType` instance describing the `std::net::IpAddr`
    /// type from the `std` crate.
    fn ip_addr() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Inet),
            syn::parse_quote!(std::net::IpAddr),
        )
        .postgres_types(["inet", "cidr"])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_hash()
        .supports_ord()
        .into()
    }

    /// Returns an `ExternalType` instance describing the `std::net::MacAddr`
    /// type from the `std` crate.
    fn mac_addr() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::MacAddr),
            syn::parse_quote!(std::net::MacAddr),
        )
        .postgres_types(["macaddr", "macaddr8"])
        .unwrap()
        .supports_clone()
        .supports_default()
        .supports_hash()
        .supports_ord()
        .into()
    }

    /// Returns an `ExternalType` instance describing the
    /// `std::time::SystemTime` type from the `std` crate.
    fn system_time() -> Self {
        ExternalType::new(
            syn::parse_quote!(diesel::sql_types::Timestamp),
            syn::parse_quote!(std::time::SystemTime),
        )
        .postgres_types([
            "timestamp",
            "time_stamp",
            "timestamp with time zone",
            "timestamptz",
            "date",
            "time",
            "time with time zone",
        ])
        .unwrap()
        .supports_clone()
        .supports_hash()
        .supports_ord()
        .into()
    }
}
