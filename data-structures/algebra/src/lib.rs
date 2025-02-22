#![doc = include_str!("../README.md")]

mod traits;
pub mod error;
pub mod impls;

/// Prelude module for the algebra crate.
pub mod prelude {
    pub use crate::traits::*;
    pub use crate::impls::*;
}