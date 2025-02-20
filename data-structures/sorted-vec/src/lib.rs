#![doc = include_str!("../README.md")]

#![cfg(feature = "alloc")]
extern crate alloc;

pub mod sorted_vec;
pub mod error;

/// Prelude module for the sorted-vec crate.
pub mod prelude {
    pub use crate::sorted_vec::*;
}