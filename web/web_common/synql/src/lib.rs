#![doc = include_str!("../README.md")]

pub mod structs;
pub mod traits;
pub mod utils;

/// Prelude module re-exporting commonly used items.
pub mod prelude {
    pub use crate::{structs::*, traits::*};
}
