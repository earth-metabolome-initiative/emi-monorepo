#![doc = include_str!("../README.md")]

pub mod similarity;

/// Prelude module for the `functional_properties` crate.
pub mod prelude {
    pub use crate::similarity::*;
}
