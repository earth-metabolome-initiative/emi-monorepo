#![doc = include_str!("../README.md")]

pub mod traits;
mod impls;

/// Prelude module to re-export commonly used items.
pub mod prelude {
    pub use crate::traits::*;
}
