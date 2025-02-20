//! Trait defining a graph that can be used as a reference by its nodes and edges.

use super::{graph::Graph, EdgeRef, NodeRef};

/// Trait defining a graph that can be used as a reference by its nodes and edges.
pub trait GraphRef<'graph>: Graph {
	/// The type of the node references in the graph.
	type NodeRef: NodeRef<'graph, Graph = Self, Id = Self::NodeId>;
	/// The type of the edge references in the graph.
	type EdgeRef: EdgeRef<'graph, Graph = Self, Id = Self::EdgeId>;

	/// Returns a reference to the node with the given identifier.
	fn node(&self, id: Self::NodeId) -> Option<&Self::NodeRef>;
}