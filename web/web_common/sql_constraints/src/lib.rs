#![doc = include_str!("../README.md")]

pub mod traits;
pub mod constraints;
mod impls;
pub mod error;
pub mod structs;

/// Prelude module re-exporting commonly used items from the crate.
pub mod prelude {
	pub use crate::constraints::*;
	pub use crate::error::Error;
	pub use crate::structs::*;
	pub use crate::traits::*;
	pub use common_traits::builder::Builder;
}