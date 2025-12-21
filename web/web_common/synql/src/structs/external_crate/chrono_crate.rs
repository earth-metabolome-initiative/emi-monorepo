//! Submodule implementing the method `chrono` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `chrono` crate.

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `chrono`
    /// crate.
    pub fn chrono() -> ExternalCrate {
        ExternalCrate::new("chrono")
            .unwrap()
            .version("0.4.42")
            .types([
                ExternalType::new(
                    syn::parse_quote!(diesel::sql_types::Timestamp),
                    syn::parse_quote!(chrono::NaiveDateTime),
                )
                .postgres_types(["timestamp", "timestamp without time zone"])
                .unwrap()
                .supports_copy()
                .supports_eq()
                .into(),
                ExternalType::new(
                    syn::parse_quote!(diesel::sql_types::Timestamptz),
                    syn::parse_quote!(chrono::DateTime<chrono::Utc>),
                )
                .postgres_types(["timestamptz", "timestamp with time zone"])
                .unwrap()
                .supports_copy()
                .supports_eq()
                .into(),
                ExternalType::new(
                    syn::parse_quote!(diesel::sql_types::Date),
                    syn::parse_quote!(chrono::NaiveDate),
                )
                .postgres_type("date")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .into(),
                ExternalType::new(
                    syn::parse_quote!(diesel::sql_types::Time),
                    syn::parse_quote!(chrono::NaiveTime),
                )
                .postgres_type("time")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .into(),
                ExternalType::new(
                    syn::parse_quote!(diesel::sql_types::Interval),
                    syn::parse_quote!(chrono::Duration),
                )
                .postgres_type("interval")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .into(),
            ])
            .unwrap()
            .into()
    }
}
