//! Submodule implementing the method `diesel` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalTrait};

static DIESEL_QUERIES_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `diesel-queries`
    /// crate.
    pub fn diesel_queries() -> Arc<ExternalCrate> {
        DIESEL_QUERIES_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("diesel-queries".to_string())
                        .unwrap()
                        .add_traits([
                            ExternalTrait::new()
                                .name("Read")
                                .unwrap()
                                .path(syn::parse_quote!(diesel_queries::prelude::Read))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("ExtensionOf")
                                .unwrap()
                                .path(syn::parse_quote!(diesel_queries::prelude::ExtensionOf))
                                .build()
                                .unwrap(),
                        ])
                        .unwrap()
                        .git(
                            "https://github.com/earth-metabolome-initiative/emi-monorepo",
                            "postgres-crate",
                        )
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}
