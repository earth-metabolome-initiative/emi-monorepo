//! Submodule implementing the method `std` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `std` crate.

use common_traits::builder::Builder;
use lazy_static::lazy_static;

use crate::structs::{ExternalCrate, ExternalType};

lazy_static! {
    pub static ref STD_CRATE: ExternalCrate<'static> = ExternalCrate::new()
        .name("std".to_string())
        .unwrap()
        .add_types(vec![
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
        .build()
        .unwrap();
}

impl ExternalCrate<'_> {
    /// Initializes a `ExternalCrate` instance describing the `std` crate.
    pub fn std() -> &'static ExternalCrate<'static> {
        &STD_CRATE
    }
}

impl<'data> ExternalType<'data> {
    /// Returns a `ExternalType` instance describing the `String` type from the
    /// `std` crate.
    fn string() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Text").unwrap())
            .rust_type(syn::parse_str("String").unwrap())
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

    /// Returns a `ExternalType` instance describing the `Vec<u8>` type from the
    /// `std` crate.
    fn vec_u8() -> Self {
        ExternalType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Binary").unwrap())
            .rust_type(syn::parse_str("Vec<u8>").unwrap())
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
            .diesel_type(
                syn::parse_str("diesel::sql_types::Array<diesel::sql_types::SmallInt>").unwrap(),
            )
            .rust_type(syn::parse_str("Vec<i16>").unwrap())
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
            .diesel_type(
                syn::parse_str("diesel::sql_types::Array<diesel::sql_types::Integer>").unwrap(),
            )
            .rust_type(syn::parse_str("Vec<i32>").unwrap())
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
            .diesel_type(
                syn::parse_str("diesel::sql_types::Array<diesel::sql_types::Oid>").unwrap(),
            )
            .rust_type(syn::parse_str("Vec<u32>").unwrap())
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
            .diesel_type(
                syn::parse_str("diesel::sql_types::Array<diesel::sql_types::Float>").unwrap(),
            )
            .rust_type(syn::parse_str("Vec<f32>").unwrap())
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
            .diesel_type(
                syn::parse_str("diesel::sql_types::Array<diesel::sql_types::Double>").unwrap(),
            )
            .rust_type(syn::parse_str("Vec<f64>").unwrap())
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
            .diesel_type(
                syn::parse_str("diesel::sql_types::Array<diesel::sql_types::Bool>").unwrap(),
            )
            .rust_type(syn::parse_str("Vec<bool>").unwrap())
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
            .diesel_type(
                syn::parse_str("diesel::sql_types::Array<diesel::sql_types::Text>").unwrap(),
            )
            .rust_type(syn::parse_str("Vec<String>").unwrap())
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
            .diesel_type(syn::parse_str("diesel::sql_types::Inet").unwrap())
            .rust_type(syn::parse_str("std::net::IpAddr").unwrap())
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
            .diesel_type(syn::parse_str("diesel::sql_types::MacAddr").unwrap())
            .rust_type(syn::parse_str("std::net::MacAddr").unwrap())
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
            .diesel_type(syn::parse_str("diesel::sql_types::Timestamp").unwrap())
            .rust_type(syn::parse_str("std::time::SystemTime").unwrap())
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
