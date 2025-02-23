//! Submodule defining common errors relative to nodes.

use crate::traits::Graph;

/// Error enumeration relative to nodes.
#[derive(Clone, PartialEq, Eq)]
pub enum NodeError<G: Graph + ?Sized> {
    /// The node does not exist.
    UnknownSourceNodeId(G::SourceNodeId),
    /// The node symbol does not exist.
    UnknownNodeSymbol(G::SourceNodeSymbol),
}

impl<G: Graph + ?Sized> core::fmt::Debug for NodeError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<G: Graph+ ?Sized> core::fmt::Display for NodeError<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            NodeError::UnknownSourceNodeId(id) => write!(f, "The node with id {id:?} does not exist."),
            NodeError::UnknownNodeSymbol(symbol) => write!(f, "The node with symbol {symbol:?} does not exist."),
        }
    }
}

impl<G: Graph> core::error::Error for NodeError<G> {}

impl<G: Graph> From<NodeError<G>> for super::Error<G> {
    fn from(error: NodeError<G>) -> Self {
        super::Error::NodeError(error)
    }
}
