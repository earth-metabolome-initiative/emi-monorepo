#![doc = include_str!("../README.md")]

pub mod constraints;
pub mod error;
mod impls;
pub mod structs;
pub mod traits;

/// Prelude module re-exporting commonly used items from the crate.
pub mod prelude {
    pub use common_traits::builder::Builder;

    pub use crate::{constraints::*, error::Error, structs::*, traits::*};
}
