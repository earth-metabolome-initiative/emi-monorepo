//! Submodule implementing the method `std` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `std` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalType};

static STD_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `std` crate.
    pub fn std() -> Arc<ExternalCrate> {
        STD_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("std".to_string())
                        .unwrap()
                        .add_types(vec![
                            Arc::new(ExternalType::string()),
                            Arc::new(ExternalType::str()),
                            Arc::new(ExternalType::vec_u8()),
                            Arc::new(ExternalType::vec_i16()),
                            Arc::new(ExternalType::vec_i32()),
                            Arc::new(ExternalType::vec_u32()),
                            Arc::new(ExternalType::vec_f32()),
                            Arc::new(ExternalType::vec_f64()),
                            Arc::new(ExternalType::vec_bool()),
                            Arc::new(ExternalType::ip_addr()),
                            Arc::new(ExternalType::mac_addr()),
                            Arc::new(ExternalType::vec_string()),
                            Arc::new(ExternalType::system_time()),
                        ])
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}

impl ExternalType {
    /// Returns a `ExternalType` instance describing the `String` type from the
    /// `std` crate.
    fn string() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Text))
            .rust_type(syn::parse_quote!(String))
            .postgres_types([
                "text",
                "char",
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
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `str` type from the
    /// `str` crate.
    fn str() -> Self {
        ExternalType::new()
            .rust_type(syn::parse_quote!(str))
            .supports_copy()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serialize()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `Vec<u8>` type from the
    /// `std` crate.
    fn vec_u8() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Binary))
            .rust_type(syn::parse_quote!(Vec<u8>))
            .postgres_types(["bytea", "bit", "varbit"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a ` ExternalType` instance describing the `Vec<i16>` type from
    /// the `std` crate.
    fn vec_i16() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::SmallInt>))
            .rust_type(syn::parse_quote!(Vec<i16>))
            .postgres_types(["_int2", "int2[]", "_smallint", "int2vector"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a ` ExternalType` instance describing the `Vec<i32>` type from
    /// the `std` crate.
    fn vec_i32() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Integer>))
            .rust_type(syn::parse_quote!(Vec<i32>))
            .postgres_types(["_int4", "int4[]", "_integer", "_cardinal_number"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a ` ExternalType` instance describing the `Vec<u32>` type from
    /// the `std` crate.
    fn vec_u32() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Oid>))
            .rust_type(syn::parse_quote!(Vec<u32>))
            .postgres_types(["_oid", "oid[]", "_regtype", "regtype[]", "oidvector"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a ` ExternalType` instance describing the `Vec<f32>` type from
    /// the `std` crate.
    fn vec_f32() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Float>))
            .rust_type(syn::parse_quote!(Vec<f32>))
            .postgres_types(["_float4", "float4[]", "_real"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_partial_eq()
            .supports_partial_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a ` ExternalType` instance describing the `Vec<f64>` type from
    /// the `std` crate.
    fn vec_f64() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Double>))
            .rust_type(syn::parse_quote!(Vec<f64>))
            .postgres_types(["_float8", "float8[]", "_double_precision"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_partial_eq()
            .supports_partial_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a ` ExternalType` instance describing the `Vec<bool>` type from
    /// the `std` crate.
    fn vec_bool() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Bool>))
            .rust_type(syn::parse_quote!(Vec<bool>))
            .postgres_types(["_bool", "bool[]"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns an [`ExternalType`] instance describing the `Vec<String>` type
    /// from the `std` crate.
    fn vec_string() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Array<diesel::sql_types::Text>))
            .rust_type(syn::parse_quote!(Vec<String>))
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
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `std::net::IpAddr` type
    /// from the `std` crate.
    fn ip_addr() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Inet))
            .rust_type(syn::parse_quote!(std::net::IpAddr))
            .postgres_types(["inet", "cidr"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `std::net::MacAddr`
    /// type from the `std` crate.
    fn mac_addr() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::MacAddr))
            .rust_type(syn::parse_quote!(std::net::MacAddr))
            .postgres_types(["macaddr", "macaddr8"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns a `ExternalType` instance describing the `std::time::SystemTime`
    /// type from the `std` crate.
    fn system_time() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_quote!(diesel::sql_types::Timestamp))
            .rust_type(syn::parse_quote!(std::time::SystemTime))
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
            .supports_default()
            .supports_hash()
            .supports_ord()
            .supports_serde()
            .unwrap()
            .build()
            .unwrap()
    }
}
