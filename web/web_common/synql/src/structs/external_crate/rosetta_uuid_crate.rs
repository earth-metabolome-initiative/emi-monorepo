//! Submodule implementing the method `rosetta_uuid` for the [`ExternalCrate`]
//! struct which initializes a `ExternalCrate` instance describing the
//! `rosetta_uuid` crate.

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `rosetta_uuid` crate.
    pub fn rosetta_uuid() -> ExternalCrate {
        ExternalCrate::new("rosetta_uuid")
            .unwrap()
            .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
            .features(["diesel", "serde"])
            .types([ExternalType::new(
                syn::parse_quote!(rosetta_uuid::diesel_impls::Uuid),
                syn::parse_quote!(rosetta_uuid::Uuid),
            )
            .postgres_type("uuid")
            .unwrap()
            .supports_copy()
            .supports_eq()
            .into()])
            .unwrap()
            .into()
    }
}
