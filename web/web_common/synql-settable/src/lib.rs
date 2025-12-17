#![doc = include_str!("../README.md")]

pub mod structs;
pub mod traits;

/// Prelude for synql-settable
pub mod prelude {
    pub use crate::{structs::*, traits::*};
}
