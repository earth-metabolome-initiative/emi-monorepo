#![doc = include_str!("../README.md")]

pub mod structs;
pub mod traits;
pub mod utils;
pub use structs::Error;

/// Prelude module re-exporting commonly used items.
pub mod prelude {
    pub use crate::{structs::*, traits::*};
}
