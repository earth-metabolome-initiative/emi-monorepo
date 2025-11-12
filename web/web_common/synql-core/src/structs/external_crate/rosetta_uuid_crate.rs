//! Submodule implementing the method `rosetta_uuid` for the [`ExternalCrate`]
//! struct which initializes a `ExternalCrate` instance describing the
//! `rosetta_uuid` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalType};

static ROSETTA_UUID_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `rosetta_uuid` crate.
    pub fn rosetta_uuid() -> Arc<ExternalCrate> {
        ROSETTA_UUID_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("rosetta_uuid")
                        .unwrap()
                        .git(
                            "https://github.com/earth-metabolome-initiative/emi-monorepo",
                            "postgres-crate",
                        )
                        .features(["diesel", "serde"])
                        .add_type(Arc::new(
                            ExternalType::new()
                                .diesel_type(syn::parse_quote!(rosetta_uuid::diesel_impls::Uuid))
                                .rust_type(syn::parse_quote!(rosetta_uuid::Uuid))
                                .postgres_type("uuid")
                                .unwrap()
                                .supports_copy()
                                .supports_eq()
                                .supports_serde()
                                .build()
                                .unwrap(),
                        ))
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}
