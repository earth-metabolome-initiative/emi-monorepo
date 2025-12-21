//! Submodule implementing the method `diesel_builders` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel-builders` crate.

use crate::structs::ExternalCrate;

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `diesel-builders`
    /// crate.
    pub fn diesel_builders() -> ExternalCrate {
        ExternalCrate::new("diesel-builders")
            .unwrap()
            .git("https://github.com/LucaCappelletti94/diesel-builders", "main")
            .into()
    }
}
