#![doc = include_str!("../README.md")]

mod traits_enumeration;
pub use traits_enumeration::TraitEnum;
mod required_crate;
pub use required_crate::RequiredCrate;
mod traits;
pub use traits::*;
mod impls;