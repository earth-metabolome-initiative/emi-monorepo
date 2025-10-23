//! Submodule implementing the method `diesel` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel` crate.

use common_traits::builder::Builder;
use lazy_static::lazy_static;

use crate::structs::{ExternalCrate, ExternalMacro, ExternalTrait, ExternalType};

lazy_static! {
    pub static ref DIESEL_CRATE: ExternalCrate<'static> = ExternalCrate::new()
        .name("diesel".to_string())
        .unwrap()
        .version("2.3.1")
        .add_macros([
            ExternalMacro::new().name("table").unwrap().build().unwrap(),
            ExternalMacro::new()
                .name("allow_tables_to_appear_in_same_query")
                .unwrap()
                .build()
                .unwrap(),
            ExternalMacro::new().name("joinable").unwrap().build().unwrap(),
        ])
        .unwrap()
        .add_type(
            ExternalType::new()
                .diesel_type(syn::parse_str("diesel::sql_types::Interval").unwrap())
                .rust_type(syn::parse_str("diesel::pg::data_types::PgInterval").unwrap())
                .postgres_type("interval")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .build()
                .unwrap()
        )
        .unwrap()
        .add_traits([
            ExternalTrait::new()
                .name("Queryable")
                .unwrap()
                .path(syn::parse_str("diesel::Queryable").unwrap())
                .build()
                .unwrap(),
            ExternalTrait::new()
                .name("Selectable")
                .unwrap()
                .path(syn::parse_str("diesel::Selectable").unwrap())
                .build()
                .unwrap(),
            ExternalTrait::new()
                .name("AsChangeset")
                .unwrap()
                .path(syn::parse_str("diesel::AsChangeset").unwrap())
                .build()
                .unwrap(),
            ExternalTrait::new()
                .name("Identifiable")
                .unwrap()
                .path(syn::parse_str("diesel::Identifiable").unwrap())
                .build()
                .unwrap(),
            ExternalTrait::new()
                .name("Associations")
                .unwrap()
                .path(syn::parse_str("diesel::Associations").unwrap())
                .build()
                .unwrap(),
        ])
        .unwrap()
        .build()
        .unwrap();
}

impl ExternalCrate<'_> {
    /// Initializes a `ExternalCrate` instance describing the `diesel` crate.
    pub fn diesel() -> &'static ExternalCrate<'static> {
        &DIESEL_CRATE
    }
}
