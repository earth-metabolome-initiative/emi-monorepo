//! A trait defining the properties of a node in a graph.

use super::{numeric_identifier::NumericIdentifier, GraphRef};

/// A trait defining the properties of a node in a graph.
pub trait NodeRef<'graph> {
    /// The identifier of the node.
    type Id: NumericIdentifier;
	/// The graph that the node belongs to.
	type Graph: GraphRef<'graph, NodeRef = Self>;

	/// Returns the identifier of the node.
	fn id(&self) -> Self::Id;

	/// Returns an iterator over the ids of the neighbors of the node.
	fn neighbor_ids(&self) -> impl Iterator<Item = Self::Id>;
}
