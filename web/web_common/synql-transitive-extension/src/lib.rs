#![doc = include_str!("../README.md")]

pub mod structs;
pub mod traits;

/// Prelude module re-exporting commonly used items from the
/// `synql-transitive-extension` crate.
pub mod prelude {
    pub use crate::{structs::*, traits::*};
}
