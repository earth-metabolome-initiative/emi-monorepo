//! Submodule implementing the method `diesel_queries` for the [`ExternalCrate`]
//! struct which initializes a `ExternalCrate` instance describing the
//! `diesel-queries` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::{
    structs::{ExternalCrate, TraitDef},
    utils::generic_type,
};

static DIESEL_QUERIES_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Helper function to create a diesel-queries trait without generic
    /// parameters.
    fn create_diesel_queries_trait(name: &str) -> TraitDef {
        let path: syn::Path =
            syn::parse_str(&format!("diesel_queries::prelude::{}", name)).unwrap();
        TraitDef::new().name(name).unwrap().path(path).build().unwrap()
    }

    /// Helper function to create a diesel-queries trait with a single generic
    /// parameter.
    fn create_diesel_queries_trait_with_generic(name: &str, generic_name: &str) -> TraitDef {
        let path: syn::Path =
            syn::parse_str(&format!("diesel_queries::prelude::{}", name)).unwrap();
        TraitDef::new()
            .name(name)
            .unwrap()
            .path(path)
            .generic(generic_type(generic_name))
            .build()
            .unwrap()
    }

    /// Helper function to create a diesel-queries trait with multiple generic
    /// parameters.
    fn create_diesel_queries_trait_with_generics(
        name: &str,
        generic_names: &[&str],
    ) -> TraitDef {
        let path: syn::Path =
            syn::parse_str(&format!("diesel_queries::prelude::{}", name)).unwrap();
        let generics: Vec<_> = generic_names.iter().map(|&n| generic_type(n)).collect();
        TraitDef::new().name(name).unwrap().path(path).generics(generics).build().unwrap()
    }

    /// Returns the cached `ExternalCrate` instance describing the
    /// `diesel-queries` crate.
    pub fn diesel_queries() -> Arc<ExternalCrate> {
        DIESEL_QUERIES_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("diesel-queries")
                        .unwrap()
                        .add_traits([
                            // Read trait
                            Self::create_diesel_queries_trait_with_generic("Read", "C"),
                            // Extension traits
                            Self::create_diesel_queries_trait_with_generic(
                                "ExtensionOf",
                                "Extended",
                            ),
                            Self::create_diesel_queries_trait_with_generic(
                                "TableIsExtensionOf",
                                "Extended",
                            ),
                            Self::create_diesel_queries_trait_with_generics(
                                "Ancestor",
                                &["Extended", "C"],
                            ),
                            // SameAs traits
                            Self::create_diesel_queries_trait_with_generic(
                                "VerticalSameAs",
                                "Ancestral",
                            ),
                            Self::create_diesel_queries_trait_with_generics(
                                "HorizontalSameAs",
                                &["Referenced", "Key"],
                            ),
                            // Column accessor traits
                            Self::create_diesel_queries_trait("TypedColumn"),
                            Self::create_diesel_queries_trait_with_generic("GetColumn", "C"),
                            Self::create_diesel_queries_trait_with_generic("MaybeGetColumn", "C"),
                            // SetColumn traits
                            Self::create_diesel_queries_trait_with_generic("SetColumn", "C"),
                            Self::create_diesel_queries_trait_with_generic("TrySetColumn", "C"),
                            Self::create_diesel_queries_trait_with_generics(
                                "ForeignKey",
                                &["Left", "Right"],
                            ),
                            Self::create_diesel_queries_trait_with_generics(
                                "SetHorizontalColumn",
                                &["HostColumn", "Referenced", "Key"],
                            ),
                            Self::create_diesel_queries_trait_with_generics(
                                "TrySetHorizontalColumn",
                                &["HostColumn", "Referenced", "Key"],
                            ),
                            Self::create_diesel_queries_trait_with_generics(
                                "SetVerticalColumn",
                                &["HostColumn", "AncestorColumn"],
                            ),
                            Self::create_diesel_queries_trait_with_generics(
                                "TrySetVerticalColumn",
                                &["HostColumn", "AncestorColumn"],
                            ),
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
