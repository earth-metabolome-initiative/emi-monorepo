#![doc = include_str!("../README.md")]

mod traits;

/// Prelude module for the algebra crate.
pub mod prelude {
    pub use crate::traits::*;
}