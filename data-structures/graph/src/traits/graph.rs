//! Module defining a bipartite graph.

use algebra::prelude::{IntoUsize, PositiveInteger, SparseMatrix2D, Symbol, TryFromUsize};

use crate::prelude::*;

/// Trait for a bipartite graph.
pub trait Graph {
    /// The dense identifiers of the source nodes in the graph.
    type SourceNodeId: PositiveInteger + IntoUsize + TryFromUsize;
    /// The dense identifiers of the destination nodes in the graph.
    type DestinationNodeId: PositiveInteger + IntoUsize + TryFromUsize;
    /// The dense identifiers of the edges in the graph.
    type EdgeId: PositiveInteger + IntoUsize;
    /// The representation of an edge in the graph.
    type Edge: Edge<SourceNodeId = Self::SourceNodeId, DestinationNodeId = Self::DestinationNodeId>;
    /// The symbol of the source node.
    type SourceNodeSymbol: Symbol;
    /// The symbol of the destination node.
    type DestinationNodeSymbol: Symbol;
    /// The vocabulary holding the symbols of the source nodes.
    type Sources: VocabularyRef<SourceSymbol = Self::SourceNodeId, DestinationSymbol = Self::SourceNodeSymbol>
        + BidirectionalVocabulary<
            SourceSymbol = Self::SourceNodeId,
            DestinationSymbol = Self::SourceNodeSymbol,
        >;
    /// The vocabulary holding the symbols of the destination nodes.
    type Destinations: VocabularyRef<
            SourceSymbol = Self::DestinationNodeId,
            DestinationSymbol = Self::DestinationNodeSymbol,
        > + BidirectionalVocabulary<
            SourceSymbol = Self::DestinationNodeId,
            DestinationSymbol = Self::DestinationNodeSymbol,
        >;
    /// The edges data structure.
    type Edges: Edges<
        Edge = Self::Edge,
        SourceNodeId = Self::SourceNodeId,
        DestinationNodeId = Self::DestinationNodeId,
        EdgeId = Self::EdgeId,
    >;

    /// Returns a reference to the vocabulary of the source nodes.
    fn source_vocabulary(&self) -> &Self::Sources;

    /// Returns a reference to the edges of the graph.
    fn edges(&self) -> &Self::Edges;

    /// Returns an iterator over the source node IDs in the graph.
    fn source_ids(&self) -> <Self::Sources as Vocabulary>::Sources<'_> {
        self.source_vocabulary().sources()
    }

    /// Returns an iterator over the node symbols in the graph.
    fn sources(&self) -> <Self::Sources as VocabularyRef>::DestinationRefs<'_> {
        self.source_vocabulary().destination_refs()
    }

    /// Returns the Symbol of the node with the given ID.
    fn source(&self, source_id: &Self::SourceNodeId) -> Option<&Self::SourceNodeSymbol> {
        self.source_vocabulary().convert_ref(source_id)
    }

    /// Returns the ID of the node with the given symbol.
    fn source_id(&self, symbol: &Self::SourceNodeSymbol) -> Option<Self::SourceNodeId> {
        self.source_vocabulary().invert(symbol)
    }

    /// Returns the number of source nodes in the graph.
    fn number_of_source_nodes(&self) -> usize {
        self.source_vocabulary().len()
    }

    /// Returns a reference to the vocabulary of the destination nodes.
    fn destination_vocabulary(&self) -> &Self::Destinations;

    /// Returns an iterator over the destination node IDs in the graph.
    fn destination_ids(&self) -> <Self::Destinations as Vocabulary>::Sources<'_> {
        self.destination_vocabulary().sources()
    }

    /// Returns an iterator over the node symbols in the graph.
    fn destinations(&self) -> <Self::Destinations as VocabularyRef>::DestinationRefs<'_> {
        self.destination_vocabulary().destination_refs()
    }

    /// Returns the Symbol of the node with the given ID.
    fn destination(
        &self,
        destination_id: &Self::DestinationNodeId,
    ) -> Option<&Self::DestinationNodeSymbol> {
        self.destination_vocabulary().convert_ref(destination_id)
    }

    /// Returns the ID of the node with the given symbol.
    fn destination_id(
        &self,
        symbol: &Self::DestinationNodeSymbol,
    ) -> Option<Self::DestinationNodeId> {
        self.destination_vocabulary().invert(symbol)
    }

    /// Returns the number of destination nodes in the graph.
    fn number_of_destination_nodes(&self) -> usize {
        self.destination_vocabulary().len()
    }

    /// Returns the number of edges in the graph.
    fn number_of_edges(&self) -> Self::EdgeId {
        self.edges().number_of_edges()
    }

    /// Returns whether the graph is empty.
    fn is_empty(&self) -> bool {
        self.source_vocabulary().is_empty() && self.destination_vocabulary().is_empty()
    }

    /// Returns the successors of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `source` - The identifier of the source node.
    fn successors(
        &self,
        source: Self::SourceNodeId,
    ) -> <<Self::Edges as Edges>::Matrix as SparseMatrix2D>::SparseRow<'_> {
        self.edges().successors(source)
    }

    /// Returns the outbound degree of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `source` - The identifier of the source node.
    fn out_degree(&self, source: Self::SourceNodeId) -> Self::DestinationNodeId {
        self.edges().out_degree(source)
    }

    /// Iterates across all out degrees of the graph.
    fn out_degrees(
        &self,
    ) -> <<Self::Edges as Edges>::Matrix as SparseMatrix2D>::SparseRowSizes<'_> {
        self.edges().out_degrees()
    }
}
