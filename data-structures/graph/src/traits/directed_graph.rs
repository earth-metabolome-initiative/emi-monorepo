//! Submodule defining the properties of a directed graph.

use algebra::prelude::*;

use super::{BidirectionalVocabulary, Edges, Vocabulary, VocabularyRef};

/// Trait defining the properties of the directed edges of a graph.
pub trait DirectedEdges:
    Edges<
    SourceNodeId = <Self as DirectedEdges>::NodeId,
    DestinationNodeId = <Self as DirectedEdges>::NodeId,
    Matrix = <Self as DirectedEdges>::DirectedMatrix,
>
{
    /// The directed matrix of the graph.
    type DirectedMatrix: SquareMatrix<Index = Self::NodeId>;

    /// The identifier of the node.
    type NodeId: PositiveInteger + IntoUsize + TryFromUsize;

    /// Returns whether the graph has self-loops.
    fn has_self_loops(&self) -> bool {
        self.number_of_self_loops() > Self::NodeId::ZERO
    }

    /// Returns the number of self-loops in the graph.
    fn number_of_self_loops(&self) -> Self::NodeId;
}

/// Trait defining the properties of a directed graph.
pub trait DirectedGraph:
    super::Graph<
    SourceNodeId = <Self as DirectedGraph>::NodeId,
    DestinationNodeId = <Self as DirectedGraph>::NodeId,
    SourceNodeSymbol = <Self as DirectedGraph>::NodeSymbol,
    DestinationNodeSymbol = <Self as DirectedGraph>::NodeSymbol,
    Edges = <Self as DirectedGraph>::DirectedEdges,
    Sources = <Self as DirectedGraph>::Nodes,
    Destinations = <Self as DirectedGraph>::Nodes,
>
{
    /// The dense identifier of the nodes in the graph.
    type NodeId: PositiveInteger + IntoUsize + TryFromUsize;
    /// The symbol of the node.
    type NodeSymbol: Symbol;
    /// The vocabulary holding the symbols of the nodes.
    type Nodes: VocabularyRef<SourceSymbol = Self::NodeId, DestinationSymbol = Self::NodeSymbol>
        + BidirectionalVocabulary<SourceSymbol = Self::NodeId, DestinationSymbol = Self::NodeSymbol>;
    /// The directed edges of the graph.
    type DirectedEdges: DirectedEdges<NodeId = Self::NodeId>;

    /// Returns the nodes vocabulary.
    fn nodes_vocabulary(&self) -> &Self::Nodes {
        self.source_vocabulary()
    }

    /// Returns the iterator over the node identifiers.
    fn node_ids(&self) -> <Self::Sources as Vocabulary>::Sources<'_> {
        self.nodes_vocabulary().sources()
    }

    /// Returns the iterator over the node symbols.
    fn nodes(&self) -> <Self::Nodes as VocabularyRef>::DestinationRefs<'_> {
        self.nodes_vocabulary().destination_refs()
    }

    /// Returns the number of nodes in the graph.
    fn number_of_nodes(&self) -> Self::NodeId {
        if let Ok(number_of_nodes) = Self::NodeId::try_from_usize(self.nodes_vocabulary().len()) {
            number_of_nodes
        } else {
            panic!("The number of nodes exceeds the capacity of the node identifier.")
        }
    }

    /// Returns whether the graph has self-loops.
    fn has_self_loops(&self) -> bool {
        self.edges().has_self_loops()
    }

    /// Returns the number of self-loops in the graph.
    fn number_of_self_loops(&self) -> Self::NodeId {
        self.edges().number_of_self_loops()
    }
}
