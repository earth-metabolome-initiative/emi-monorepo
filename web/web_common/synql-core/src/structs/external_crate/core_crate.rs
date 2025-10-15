//! Submodule implementing the method `core` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `core` crate.

use common_traits::builder::Builder;

use crate::structs::ExternalCrate;

mod numeric;

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `core` crate.
    pub fn core() -> Self {
        Self::new()
            .name("core".to_string())
            .unwrap()
            .add_types(numeric::all_types())
            .unwrap()
            .build()
            .unwrap()
    }
}
