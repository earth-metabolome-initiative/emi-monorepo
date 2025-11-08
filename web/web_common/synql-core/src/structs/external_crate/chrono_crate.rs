//! Submodule implementing the method `chrono` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `chrono` crate.

use std::sync::Arc;

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Initializes an `ExternalCrate` instance describing the `chrono` crate.
    pub fn chrono() -> ExternalCrate {
        ExternalCrate::new()
            .name("chrono")
            .unwrap()
            .version("0.4.42")
            .add_type(Arc::new(
                ExternalType::new()
                    .diesel_type(syn::parse_quote!(diesel::sql_types::Timestamp))
                    .rust_type(syn::parse_quote!(chrono::NaiveDateTime))
                    .postgres_type("timestamp")
                    .unwrap()
                    .supports_copy()
                    .supports_eq()
                    .supports_serde()
                    .unwrap()
                    .build()
                    .unwrap(),
            ))
            .unwrap()
            .add_type(Arc::new(
                ExternalType::new()
                    .diesel_type(syn::parse_quote!(diesel::sql_types::Timestamptz))
                    .rust_type(syn::parse_quote!(chrono::DateTime<chrono::Utc>))
                    .postgres_type("timestamptz")
                    .unwrap()
                    .supports_copy()
                    .supports_eq()
                    .supports_serde()
                    .unwrap()
                    .build()
                    .unwrap(),
            ))
            .unwrap()
            .add_type(Arc::new(
                ExternalType::new()
                    .diesel_type(syn::parse_quote!(diesel::sql_types::Date))
                    .rust_type(syn::parse_quote!(chrono::NaiveDate))
                    .postgres_type("date")
                    .unwrap()
                    .supports_copy()
                    .supports_eq()
                    .supports_serde()
                    .unwrap()
                    .build()
                    .unwrap(),
            ))
            .unwrap()
            .add_type(Arc::new(
                ExternalType::new()
                    .diesel_type(syn::parse_quote!(diesel::sql_types::Time))
                    .rust_type(syn::parse_quote!(chrono::NaiveTime))
                    .postgres_type("time")
                    .unwrap()
                    .supports_copy()
                    .supports_eq()
                    .supports_serde()
                    .unwrap()
                    .build()
                    .unwrap(),
            ))
            .unwrap()
            .add_type(Arc::new(
                ExternalType::new()
                    .diesel_type(syn::parse_quote!(diesel::sql_types::Interval))
                    .rust_type(syn::parse_quote!(chrono::Duration))
                    .postgres_type("interval")
                    .unwrap()
                    .supports_copy()
                    .supports_eq()
                    .supports_serde()
                    .unwrap()
                    .build()
                    .unwrap(),
            ))
            .unwrap()
            .build()
            .unwrap()
    }
}
