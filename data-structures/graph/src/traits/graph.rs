//! Module defining a graph.

use crate::prelude::*;
use algebra::prelude::{PositiveInteger, Symbol};

/// Trait for a graph.
pub trait Graph: core::fmt::Debug {
    /// The dense identifiers of the nodes in the graph.
    type NodeId: PositiveInteger;
    /// The symbol of the node.
    type NodeSymbol: Symbol;
    /// The vocabulary holding the symbols of the nodes.
    type NodeVocabulary: VocabularyRef<SourceSymbol = Self::NodeId, DestinationSymbol = Self::NodeSymbol>
        + BidirectionalVocabulary<SourceSymbol = Self::NodeId, DestinationSymbol = Self::NodeSymbol>;

    /// Returns  a reference to the vocabulary of the nodes.
    fn node_vocabulary(&self) -> &Self::NodeVocabulary;

    /// Returns an iterator over the node identifiers in the graph.
    fn node_ids(&self) -> <Self::NodeVocabulary as Vocabulary>::Sources<'_> {
        self.node_vocabulary().sources()
    }

    /// Returns an iterator over the node symbols in the graph.
    fn nodes(&self) -> <Self::NodeVocabulary as VocabularyRef>::DestinationRefs<'_> {
        self.node_vocabulary().destination_refs()
    }

    /// Returns the Symbol of the node with the given ID.
    fn node(&self, id: &Self::NodeId) -> Option<&Self::NodeSymbol> {
        self.node_vocabulary().convert_ref(id)
    }

    /// Returns the ID of the node with the given symbol.
    fn node_id(&self, symbol: &Self::NodeSymbol) -> Option<Self::NodeId> {
        self.node_vocabulary().invert(symbol)
    }

    /// Returns the number of nodes in the graph.
    fn number_of_nodes(&self) -> usize {
        self.node_vocabulary().len()
    }

    /// Returns whether the graph is empty.
    fn is_empty(&self) -> bool {
        self.number_of_nodes() == 0
    }
}
