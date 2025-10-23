//! Submodule implementing the method `chrono` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `chrono` crate.

use common_traits::builder::Builder;
use lazy_static::lazy_static;

use crate::structs::{ExternalCrate, ExternalType};

lazy_static! {
    pub static ref CHRONO_CRATE: ExternalCrate<'static> = ExternalCrate::new()
        .name("chrono")
        .unwrap()
        .version("0.4.42")
        .add_type(
            ExternalType::new()
                .diesel_type(syn::parse_str("diesel::sql_types::Timestamp").unwrap())
                .rust_type(syn::parse_str("chrono::NaiveDateTime").unwrap())
                .postgres_type("timestamp")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .supports_serde()
                .unwrap()
                .build()
                .unwrap()
        )
        .unwrap()
        .add_type(
            ExternalType::new()
                .diesel_type(syn::parse_str("diesel::sql_types::Timestamptz").unwrap())
                .rust_type(syn::parse_str("chrono::DateTime<chrono::Utc>").unwrap())
                .postgres_type("timestamptz")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .supports_serde()
                .unwrap()
                .build()
                .unwrap()
        )
        .unwrap()
        .add_type(
            ExternalType::new()
                .diesel_type(syn::parse_str("diesel::sql_types::Date").unwrap())
                .rust_type(syn::parse_str("chrono::NaiveDate").unwrap())
                .postgres_type("date")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .supports_serde()
                .unwrap()
                .build()
                .unwrap()
        )
        .unwrap()
        .add_type(
            ExternalType::new()
                .diesel_type(syn::parse_str("diesel::sql_types::Time").unwrap())
                .rust_type(syn::parse_str("chrono::NaiveTime").unwrap())
                .postgres_type("time")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .supports_serde()
                .unwrap()
                .build()
                .unwrap()
        )
        .unwrap()
        .add_type(
            ExternalType::new()
                .diesel_type(syn::parse_str("diesel::sql_types::Interval").unwrap())
                .rust_type(syn::parse_str("chrono::Duration").unwrap())
                .postgres_type("interval")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .supports_serde()
                .unwrap()
                .build()
                .unwrap()
        )
        .unwrap()
        .build()
        .unwrap();
}

impl ExternalCrate<'_> {
    /// Initializes an `ExternalCrate` instance describing the `chrono` crate.
    pub fn chrono() -> ExternalCrate<'static> {
        CHRONO_CRATE.clone()
    }
}
