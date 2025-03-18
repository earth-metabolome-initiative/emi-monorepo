//! Submodule defining common errors relative to nodes.

use crate::traits::{MonopartiteGraph, Vocabulary};

/// Error enumeration relative to nodes.
#[derive(Clone, PartialEq, Eq)]
pub enum NodeError<V: Vocabulary> {
    /// The node does not exist.
    UnknownNodeId(V::SourceSymbol),
    /// The node symbol does not exist.
    UnknownNodeSymbol(V::DestinationSymbol),
}

impl<V: Vocabulary> core::fmt::Debug for NodeError<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<V: Vocabulary> core::fmt::Display for NodeError<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            NodeError::UnknownNodeId(id) => {
                write!(f, "The node with id {id:?} does not exist.")
            }
            NodeError::UnknownNodeSymbol(symbol) => {
                write!(f, "The node with symbol {symbol:?} does not exist.")
            }
        }
    }
}

impl<V: Vocabulary> core::error::Error for NodeError<V> {}

impl<G: MonopartiteGraph> From<NodeError<G::Nodes>> for super::MonopartiteError<G> {
    fn from(error: NodeError<G::Nodes>) -> Self {
        super::MonopartiteError::NodeError(error)
    }
}
