//! Submodule defining common errors for the graph crate.

pub mod bipartite_graph_error;
pub mod builder;
pub mod monopartite_graph_error;
pub mod nodes;
pub use bipartite_graph_error::BipartiteError;
pub use monopartite_graph_error::MonopartiteError;
