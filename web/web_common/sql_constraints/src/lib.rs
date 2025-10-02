#![doc = include_str!("../README.md")]

pub mod constraints;
pub mod error;
pub mod traits;

/// Prelude module re-exporting commonly used items from the crate.
pub mod prelude {
    pub use common_traits::builder::Builder;
    pub use sql_traits::prelude::*;

    pub use crate::{constraints::*, error::Error, traits::*};
}
