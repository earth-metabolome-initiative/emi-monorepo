//! Submodule defining the properties of a directed graph.

/// Trait defining the properties of a directed graph.
pub trait DirectedGraph: super::Graph {
	/// Successors of a node.
	type SuccessorsIter: Iterator<Item = Self::NodeId>;
	/// Predecessors of a node.
	type PredecessorsIter: Iterator<Item = Self::NodeId>;

	/// Returns the successors of the node with the given identifier.
	fn successors(&self, id: Self::NodeId) -> Self::SuccessorsIter;

	/// Returns the predecessors of the node with the given identifier.
	fn predecessors(&self, id: Self::NodeId) -> Self::PredecessorsIter;
}