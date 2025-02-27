//! Submodule defining a builder for an edge list.

mod builder;
mod directed_builder;
mod undirected_builder;
pub use builder::*;
pub use directed_builder::*;
pub use undirected_builder::*;
