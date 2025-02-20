//! Submodule defining common errors relative to nodes.

use crate::traits::Graph;

/// Error enumeration relative to nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeError<G: Graph> {
	/// The node does not exist.
	NodeIdDoesNotExistInGraph(G::NodeId),
}