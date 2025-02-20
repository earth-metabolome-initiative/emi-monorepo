#![doc = include_str!("../README.md")]

pub mod traits;
pub mod errors;

/// Prelude module for the graph crate.
pub mod prelude {
    pub use crate::traits::*;
}