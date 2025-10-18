#![doc = include_str!("../README.md")]

pub mod structs;
pub mod traits;

/// Prelude module re-exporting commonly used items.
pub mod prelude {
    pub use common_traits::prelude::*;
    pub use sql_traits::prelude::*;

    pub use crate::{structs::*, traits::*};
}
