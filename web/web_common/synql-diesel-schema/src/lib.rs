#![doc = include_str!("../README.md")]

pub mod structs;
pub mod traits;

/// Prelude module re-exporting the most important traits and structs.
pub mod prelude {
    pub use synql_core::prelude::*;

    pub use crate::{structs::*, traits::*};
}
