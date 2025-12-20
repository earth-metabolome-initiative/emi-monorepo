//! Submodule implementing the method `core` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `core` crate.

use crate::structs::ExternalCrate;
mod numeric;

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `core` crate.
    pub fn core() -> ExternalCrate {
        ExternalCrate::new("core").unwrap().types(numeric::all_types()).unwrap().into()
    }
}
