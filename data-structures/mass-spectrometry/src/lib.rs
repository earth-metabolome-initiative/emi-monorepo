#![doc = include_str!("../README.md")]

pub mod traits;
pub mod structs;

/// Prelude module for the mass-spectrometry crate.
pub mod prelude {
    pub use crate::traits::*;
    pub use crate::structs::*;
    pub use functional_properties::similarity::ScalarSimilarity;
}