#![doc = include_str!("../README.md")]

mod required_crate;
pub use required_crate::RequiredCrate;
mod traits;
pub use traits::*;
mod impls;
mod required_type;
pub use required_type::{RequiredType, Trait};
