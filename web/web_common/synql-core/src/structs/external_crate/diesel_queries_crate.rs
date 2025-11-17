//! Submodule implementing the method `diesel_queries` for the [`ExternalCrate`]
//! struct which initializes a `ExternalCrate` instance describing the
//! `diesel-queries` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::{
    structs::{ExternalCrate, ExternalTrait},
    utils::generic_type,
};

static DIESEL_QUERIES_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Helper function to create a diesel-queries trait without generic
    /// parameters.
    fn create_diesel_queries_trait(
        name: &str,
    ) -> Result<ExternalTrait, Box<dyn std::error::Error>> {
        let path: syn::Path = syn::parse_str(&format!("diesel_queries::prelude::{}", name))?;
        Ok(ExternalTrait::new().name(name)?.path(path).build()?)
    }

    /// Helper function to create a diesel-queries trait with a single generic
    /// parameter.
    fn create_diesel_queries_trait_with_generic(
        name: &str,
        generic_name: &str,
    ) -> Result<ExternalTrait, Box<dyn std::error::Error>> {
        let path: syn::Path = syn::parse_str(&format!("diesel_queries::prelude::{}", name))?;
        Ok(ExternalTrait::new()
            .name(name)?
            .path(path)
            .generic(generic_type(generic_name))
            .build()?)
    }

    /// Helper function to create a diesel-queries trait with multiple generic
    /// parameters.
    fn create_diesel_queries_trait_with_generics(
        name: &str,
        generic_names: &[&str],
    ) -> Result<ExternalTrait, Box<dyn std::error::Error>> {
        let path: syn::Path = syn::parse_str(&format!("diesel_queries::prelude::{}", name))?;
        let generics: Vec<_> = generic_names.iter().map(|&n| generic_type(n)).collect();
        Ok(ExternalTrait::new().name(name)?.path(path).generics(generics).build()?)
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
                            Self::create_diesel_queries_trait_with_generic("Read", "C").unwrap(),
                            // Extension traits
                            Self::create_diesel_queries_trait_with_generic(
                                "ExtensionOf",
                                "Extended",
                            )
                            .unwrap(),
                            Self::create_diesel_queries_trait_with_generic(
                                "TableIsExtensionOf",
                                "Extended",
                            )
                            .unwrap(),
                            Self::create_diesel_queries_trait_with_generics(
                                "Ancestor",
                                &["Extended", "C"],
                            )
                            .unwrap(),
                            // SameAs traits
                            Self::create_diesel_queries_trait_with_generic(
                                "VerticalSameAs",
                                "Ancestral",
                            )
                            .unwrap(),
                            Self::create_diesel_queries_trait_with_generics(
                                "HorizontalSameAs",
                                &["Referenced", "Key"],
                            )
                            .unwrap(),
                            // Column accessor traits
                            Self::create_diesel_queries_trait("TypedColumn").unwrap(),
                            Self::create_diesel_queries_trait_with_generic("GetColumn", "C")
                                .unwrap(),
                            Self::create_diesel_queries_trait_with_generic("MaybeGetColumn", "C")
                                .unwrap(),
                            // SetColumn traits
                            Self::create_diesel_queries_trait_with_generic("SetColumn", "C")
                                .unwrap(),
                            Self::create_diesel_queries_trait_with_generic("TrySetColumn", "C")
                                .unwrap(),
                            Self::create_diesel_queries_trait_with_generics(
                                "SetHorizontalColumn",
                                &["HostColumn", "Referenced", "Key"],
                            )
                            .unwrap(),
                            Self::create_diesel_queries_trait_with_generics(
                                "TrySetHorizontalColumn",
                                &["HostColumn", "Referenced", "Key"],
                            )
                            .unwrap(),
                            Self::create_diesel_queries_trait_with_generics(
                                "SetVerticalColumn",
                                &["HostColumn", "AncestorColumn"],
                            )
                            .unwrap(),
                            Self::create_diesel_queries_trait_with_generics(
                                "TrySetVerticalColumn",
                                &["HostColumn", "AncestorColumn"],
                            )
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
