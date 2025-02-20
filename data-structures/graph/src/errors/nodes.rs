//! Submodule defining common errors relative to nodes.

use crate::traits::Graph;

/// Error enumeration relative to nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeError<G: Graph + ?Sized> {
    /// The node does not exist.
    UnknownNodeId(G::NodeId),
    /// The node symbol does not exist.
    UnknownNodeSymbol(G::NodeSymbol),
}

impl<G: Graph> core::fmt::Display for NodeError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            NodeError::UnknownNodeId(node_id) => write!(f, "Unknown node ID: {node_id}"),
            NodeError::UnknownNodeSymbol(node_symbol) => {
                write!(f, "Unknown node symbol: {node_symbol:?}")
            }
        }
    }
}

impl<G: Graph> core::error::Error for NodeError<G> {}

impl<G: Graph> From<NodeError<G>> for super::Error<G> {
    fn from(error: NodeError<G>) -> Self {
        super::Error::NodeError(error)
    }
}
