//! Submodule defining the properties of a monopartite graph.
//!
//! A monopartite graph is a graph where the nodes are of the same type, i.e
//! they are not divided into different partitions.

use algebra::prelude::*;
use num_traits::{ConstZero, SaturatingAdd};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use super::{BidirectionalVocabulary, Edges, Vocabulary};

/// Trait defining the properties of the monopartited edges of a graph.
pub trait MonopartiteEdges:
    Edges<
        SourceNodeId = <Self as MonopartiteEdges>::NodeId,
        DestinationNodeId = <Self as MonopartiteEdges>::NodeId,
        Matrix = <Self as MonopartiteEdges>::MonopartiteMatrix,
    >
{
    /// The monopartited matrix of the graph.
    type MonopartiteMatrix: SparseSquareMatrix<Index = Self::NodeId>;

    /// The identifier of the node.
    type NodeId: PositiveInteger + IntoUsize + TryFromUsize + SaturatingAdd;

    /// Returns whether the graph has self-loops.
    fn has_self_loops(&self) -> bool {
        self.number_of_self_loops() > Self::NodeId::ZERO
    }

    /// Returns the number of self-loops in the graph.
    fn number_of_self_loops(&self) -> Self::NodeId;
}

impl<E> MonopartiteEdges for E
where
    E: Edges<DestinationNodeId = <E as Edges>::SourceNodeId>,
    E::Matrix: SparseSquareMatrix<Index = E::SourceNodeId>,
{
    type MonopartiteMatrix = E::Matrix;
    type NodeId = E::SourceNodeId;

    fn number_of_self_loops(&self) -> Self::NodeId {
        self.matrix().number_of_defined_diagonal_values()
    }
}

/// Trait defining the properties of a monopartited graph.
pub trait MonopartiteGraph: super::Graph {
    /// The dense identifier of the nodes in the graph.
    type NodeId: PositiveInteger + IntoUsize + TryFromUsize;
    /// The symbol of the node.
    type NodeSymbol: Symbol;
    /// The vocabulary holding the symbols of the nodes.
    type Nodes: BidirectionalVocabulary<SourceSymbol = Self::NodeId, DestinationSymbol = Self::NodeSymbol>;

    /// Returns the nodes vocabulary.
    fn nodes_vocabulary(&self) -> &Self::Nodes;

    /// Returns the iterator over the node identifiers.
    fn node_ids(&self) -> <Self::Nodes as Vocabulary>::Sources<'_> {
        self.nodes_vocabulary().sources()
    }

    /// Returns the iterator over the node symbols.
    fn nodes(&self) -> <Self::Nodes as Vocabulary>::Destinations<'_> {
        self.nodes_vocabulary().destinations()
    }

    /// Returns the number of nodes in the graph.
    fn number_of_nodes(&self) -> Self::NodeId {
        if let Ok(number_of_nodes) = Self::NodeId::try_from_usize(self.nodes_vocabulary().len()) {
            number_of_nodes
        } else {
            panic!("The number of nodes exceeds the capacity of the node identifier.")
        }
    }
}
