//! Module defining a graph.

use super::numeric_identifier::NumericIdentifier;

/// Trait for a graph.
pub trait Graph {
    /// The identifiers of the nodes in the graph.
    type NodeId: NumericIdentifier;

    /// The identifiers of the edges in the graph.
    type EdgeId: NumericIdentifier;

	/// Returns the minimum node identifier in the graph.
	fn min_node_id(&self) -> Self::NodeId;

	/// Returns the maximum node identifier in the graph.
	fn max_node_id(&self) -> Self::NodeId;
}
