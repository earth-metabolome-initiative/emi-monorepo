//! Submodule implementing the method `rosetta_timestamp` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `rosetta_timestamp` crate.

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `rosetta_timestamp` crate.
    pub fn rosetta_timestamp() -> ExternalCrate {
        ExternalCrate::new("rosetta_timestamp")
            .unwrap()
            .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
            .features(["diesel", "serde"])
            .types([ExternalType::new(
                syn::parse_quote!(rosetta_timestamp::diesel_impls::TimestampUTC),
                syn::parse_quote!(rosetta_timestamp::TimestampUTC),
            )
            .postgres_types(["timestamp with time zone", "timestamptz"])
            .unwrap()
            .supports_copy()
            .supports_eq()
            .into()])
            .unwrap()
            .into()
    }
}
