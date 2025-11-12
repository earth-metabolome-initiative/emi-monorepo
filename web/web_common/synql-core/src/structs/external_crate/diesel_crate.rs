//! Submodule implementing the method `diesel` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalMacro, ExternalTrait, ExternalType};

static DIESEL_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `diesel`
    /// crate.
    pub fn diesel() -> Arc<ExternalCrate> {
        DIESEL_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("diesel")
                        .unwrap()
                        // .version("2.3.3")
                        .git("https://github.com/diesel-rs/diesel", "main")
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
                        .add_types([
                            Arc::new(
                                ExternalType::new()
                                    .diesel_type(syn::parse_quote!(diesel::sql_types::Interval))
                                    .rust_type(syn::parse_quote!(
                                        diesel::pg::data_types::PgInterval
                                    ))
                                    .postgres_type("interval")
                                    .unwrap()
                                    .supports_copy()
                                    .supports_eq()
                                    .build()
                                    .unwrap(),
                            ),
                            Arc::new(
                                ExternalType::new()
                                    .diesel_type(syn::parse_quote!(diesel::result::Error))
                                    .rust_type(syn::parse_quote!(diesel::result::Error))
                                    .build()
                                    .unwrap(),
                            ),
                        ])
                        .unwrap()
                        .add_traits([
                            ExternalTrait::new()
                                .name("Queryable")
                                .unwrap()
                                .path(syn::parse_quote!(diesel::Queryable))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("Selectable")
                                .unwrap()
                                .path(syn::parse_quote!(diesel::Selectable))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("AsChangeset")
                                .unwrap()
                                .path(syn::parse_quote!(diesel::AsChangeset))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("Identifiable")
                                .unwrap()
                                .path(syn::parse_quote!(diesel::Identifiable))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("Insertable")
                                .unwrap()
                                .path(syn::parse_quote!(diesel::Insertable))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("Associations")
                                .unwrap()
                                .path(syn::parse_quote!(diesel::Associations))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("OptionalExtension")
                                .unwrap()
                                .path(syn::parse_quote!(diesel::OptionalExtension))
                                .build()
                                .unwrap(),
                        ])
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}
