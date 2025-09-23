//! Submodule implementing the method `core` for the [`RequiredCrate`] struct which
//! initializes a `RequiredCrate` instance describing the `core` crate.

use crate::RequiredCrate;
use common_traits::builder::Builder;

mod numeric;

impl RequiredCrate {
    /// Initializes a `RequiredCrate` instance describing the `core` crate.
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
