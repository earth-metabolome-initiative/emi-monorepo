#![doc = include_str!("../README.md")]

pub mod error;
mod impls;
pub mod traits;

/// Prelude module to re-export commonly used items.
pub mod prelude {
    pub use crate::traits::*;
}
