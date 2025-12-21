//! Submodule registering validation errors for external crates.

use crate::structs::ExternalCrate;

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `validation_errors` crate.
    pub fn validation_errors() -> ExternalCrate {
        ExternalCrate::new("validation_errors")
            .unwrap()
            .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
            .into()
    }
}
