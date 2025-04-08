//! Module defining a bipartite graph.

/// Trait for a bipartite graph.
pub trait Graph {
    /// Returns whether the graph has any nodes.
    fn has_nodes(&self) -> bool;

    /// Returns whether the graph has any edges.
    fn has_edges(&self) -> bool;
}
