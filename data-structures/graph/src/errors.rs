//! Submodule defining common errors for the graph crate.

pub mod builder;
pub mod nodes;
pub mod monopartite_graph_error;
pub mod bipartite_graph_error;
pub use monopartite_graph_error::MonopartiteError;
pub use bipartite_graph_error::BipartiteError;
