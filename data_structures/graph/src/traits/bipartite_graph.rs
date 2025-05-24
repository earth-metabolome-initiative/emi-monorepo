//! Submodule defining a trait for a bipartite graph.
//!
//! A bipartite graph is a graph whose vertices can be divided into two disjoint
//! sets such that no two vertices within the same set are adjacent.

use algebra::prelude::Symbol;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use super::{BidirectionalVocabulary, Graph, Vocabulary};

/// Trait defining the properties of a bipartite graph.
pub trait BipartiteGraph: Graph {
    /// The dense identifiers of the left nodes in the graph.
    type LeftNodeId: PositiveInteger + IntoUsize + TryFromUsize;
    /// The dense identifiers of the right nodes in the graph.
    type RightNodeId: PositiveInteger + IntoUsize + TryFromUsize;
    /// The symbol of the left node.
    type LeftNodeSymbol: Symbol;
    /// The symbol of the right node.
    type RightNodeSymbol: Symbol;
    /// The vocabulary holding the symbols of the left nodes.
    type LeftNodes: BidirectionalVocabulary<
            SourceSymbol = Self::LeftNodeId,
            DestinationSymbol = Self::LeftNodeSymbol,
        >;
    /// The vocabulary holding the symbols of the right nodes.
    type RightNodes: BidirectionalVocabulary<
            SourceSymbol = Self::RightNodeId,
            DestinationSymbol = Self::RightNodeSymbol,
        >;

    /// Returns a reference to the vocabulary of the left nodes.
    fn left_nodes_vocabulary(&self) -> &Self::LeftNodes;

    /// Returns an iterator over the left node IDs in the graph.
    fn left_node_ids(&self) -> <Self::LeftNodes as Vocabulary>::Sources<'_> {
        self.left_nodes_vocabulary().sources()
    }

    /// Returns an iterator over the left node symbols in the graph.
    fn left_nodes(&self) -> <Self::LeftNodes as Vocabulary>::Destinations<'_> {
        self.left_nodes_vocabulary().destinations()
    }

    /// Returns the Symbol of the node with the given ID.
    fn left_node(&self, left_node_id: &Self::LeftNodeId) -> Option<Self::LeftNodeSymbol> {
        self.left_nodes_vocabulary().convert(left_node_id)
    }

    /// Returns the ID of the node with the given symbol.
    fn left_node_id(&self, symbol: &Self::LeftNodeSymbol) -> Option<Self::LeftNodeId> {
        self.left_nodes_vocabulary().invert(symbol)
    }

    /// Returns the number of left nodes in the graph.
    fn number_of_left_nodes(&self) -> usize {
        self.left_nodes_vocabulary().len()
    }

    /// Returns a reference to the vocabulary of the right nodes.
    fn right_nodes_vocabulary(&self) -> &Self::RightNodes;

    /// Returns an iterator over the right node IDs in the graph.
    fn right_node_ids(&self) -> <Self::RightNodes as Vocabulary>::Sources<'_> {
        self.right_nodes_vocabulary().sources()
    }

    /// Returns an iterator over the node symbols in the graph.
    fn right_nodes(&self) -> <Self::RightNodes as Vocabulary>::Destinations<'_> {
        self.right_nodes_vocabulary().destinations()
    }

    /// Returns the Symbol of the node with the given ID.
    fn right_node(&self, right_node_id: &Self::RightNodeId) -> Option<Self::RightNodeSymbol> {
        self.right_nodes_vocabulary().convert(right_node_id)
    }

    /// Returns the ID of the node with the given symbol.
    fn right_node_id(&self, symbol: &Self::RightNodeSymbol) -> Option<Self::RightNodeId> {
        self.right_nodes_vocabulary().invert(symbol)
    }

    /// Returns the number of right nodes in the graph.
    fn number_of_right_nodes(&self) -> usize {
        self.right_nodes_vocabulary().len()
    }
}
