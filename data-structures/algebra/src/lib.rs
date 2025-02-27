#![doc = include_str!("../README.md")]

pub mod error;
pub mod impls;
mod traits;

/// Prelude module for the algebra crate.
pub mod prelude {
    pub use crate::{impls::*, traits::*};
}
