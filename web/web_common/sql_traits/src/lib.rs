#![doc = include_str!("../README.md")]

pub mod errors;
mod impls;
pub mod structs;
pub mod traits;
pub mod utils;

/// Prelude module re-exporting commonly used items from the crate.
pub mod prelude {
    pub use crate::{structs::*, traits::*};
}
