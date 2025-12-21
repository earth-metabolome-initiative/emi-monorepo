//! Submodule implementing the method `serde` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `serde` crate.

use crate::structs::ExternalCrate;

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `serde`
    /// crate.
    pub fn serde() -> ExternalCrate {
        ExternalCrate::new("serde").unwrap().version("1.0").features(["derive"]).into()
    }
}
