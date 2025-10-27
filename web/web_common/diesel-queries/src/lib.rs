#![doc = include_str!("../README.md")]

pub mod traits;

/// Prelude module which re-exports commonly used items.
pub mod prelude {
    pub use crate::traits::*;
}
