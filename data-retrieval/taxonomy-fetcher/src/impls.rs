//! Submodule collecting the concrete implementations of the taxonomy-fetcher crate.

pub mod generic;
pub mod open_tree_of_life;
pub mod catalog_of_life;

pub use open_tree_of_life::*;
pub use catalog_of_life::*;