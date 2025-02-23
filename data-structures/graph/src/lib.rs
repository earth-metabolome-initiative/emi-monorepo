#![doc = include_str!("../README.md")]

#![cfg(feature = "alloc")]
extern crate alloc;

mod impls;
pub mod naive_structs;
pub mod errors;
pub mod traits;

/// Prelude module for the graph crate.
pub mod prelude {
    pub use crate::traits::*;
    pub use crate::naive_structs::*;
}
