//! Submodule implementing the method `std` for the [`RequiredCrate`] struct which
//! initializes a `RequiredCrate` instance describing the `std` crate.

use crate::{RequiredCrate, RequiredType};
use common_traits::builder::Builder;

impl RequiredCrate {
    /// Initializes a `RequiredCrate` instance describing the `std` crate.
    pub fn std() -> Self {
        Self::new()
            .name("std".to_string())
            .unwrap()
            .add_types(vec![
                RequiredType::string(),
                RequiredType::vec_u8(),
                RequiredType::ip_addr(),
                RequiredType::mac_addr(),
            ])
            .unwrap()
            .build()
            .unwrap()
    }
}

impl RequiredType {
    /// Returns a `RequiredType` instance describing the `String` type from the `std` crate.
    fn string() -> Self {
        RequiredType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Text").unwrap())
            .rust_type(syn::parse_str("String").unwrap())
            .postgres_types(["text", "character varying", "varchar", "bpchar", "name"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }

    /// Returns a `RequiredType` instance describing the `Vec<u8>` type from the `std` crate.
    fn vec_u8() -> Self {
        RequiredType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Binary").unwrap())
            .rust_type(syn::parse_str("Vec<u8>").unwrap())
            .postgres_types(["bytea", "bit", "varbit"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }

    /// Returns a `RequiredType` instance describing the `std::net::IpAddr` type from the `std` crate.
    fn ip_addr() -> Self {
        RequiredType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::Inet").unwrap())
            .rust_type(syn::parse_str("std::net::IpAddr").unwrap())
            .postgres_types(["inet", "cidr"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }

    /// Returns a `RequiredType` instance describing the `std::net::MacAddr` type from the `std` crate.
    fn mac_addr() -> Self {
        RequiredType::new()
            .diesel_type(syn::parse_str("diesel::sql_types::MacAddr").unwrap())
            .rust_type(syn::parse_str("std::net::MacAddr").unwrap())
            .postgres_types(["macaddr", "macaddr8"])
            .unwrap()
            .supports_clone()
            .supports_default()
            .supports_hash()
            .supports_ord()
            .build()
            .unwrap()
    }
}