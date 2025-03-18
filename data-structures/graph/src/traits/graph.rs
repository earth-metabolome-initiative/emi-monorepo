//! Module defining a bipartite graph.

/// Trait for a bipartite graph.
pub trait Graph {
	/// Returns whether the graph is empty.
	fn is_empty(&self) -> bool;
}
