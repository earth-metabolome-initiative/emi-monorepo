//! Submodule defining the graph-related traits.

pub mod vocabulary;
pub mod graph;
pub mod directed_graph;
pub mod undirected_graph;
pub mod algorithms;

pub use vocabulary::*;
pub use graph::*;
pub use directed_graph::*;
pub use undirected_graph::*;
pub use algorithms::*;
