#![doc = include_str!("../README.md")]
#![cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod sorted_array;
pub mod sorted_vec;

/// Prelude module for the sorted-vec crate.
pub mod prelude {
    pub use crate::{sorted_array::*, sorted_vec::*};
}
