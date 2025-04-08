#![doc = include_str!("../README.md")]

pub mod structs;
pub mod traits;

/// Prelude module for the mass-spectrometry crate.
pub mod prelude {
    pub use functional_properties::similarity::ScalarSimilarity;

    pub use crate::{structs::*, traits::*};
}
