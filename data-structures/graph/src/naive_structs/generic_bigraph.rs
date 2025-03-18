//! Submodule providing a naively implemented `GenericBiGraph`.

use algebra::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use super::generic_monoplex_bipartite_graph_builder::MonoplexBipartiteGraphBuilderError;
use crate::traits::{
    BidirectionalVocabulary, BipartiteGraph, Edges, Graph, MonoplexGraph, Vocabulary, VocabularyRef,
};

/// Struct representing a generic bigraph.
pub struct GenericBiGraph<LeftNodes, RightNodes, Edges> {
    /// The left nodes of the graph.
    left_nodes: LeftNodes,
    /// The right nodes of the graph.
    right_nodes: RightNodes,
    /// The edges of the graph.
    edges: Edges,
}

impl<LeftNodes, RightNodes, Edges> TryFrom<(LeftNodes, RightNodes, Edges)>
    for GenericBiGraph<LeftNodes, RightNodes, Edges>
{
    type Error = MonoplexBipartiteGraphBuilderError;

    fn try_from(
        (left_nodes, right_nodes, edges): (LeftNodes, RightNodes, Edges),
    ) -> Result<Self, Self::Error> {
        Ok(Self { left_nodes, right_nodes, edges })
    }
}

impl<LeftNodes, RightNodes, Edges> Default for GenericBiGraph<LeftNodes, RightNodes, Edges>
where
    LeftNodes: Default,
    RightNodes: Default,
    Edges: Default,
{
    fn default() -> Self {
        Self {
            left_nodes: LeftNodes::default(),
            right_nodes: RightNodes::default(),
            edges: Edges::default(),
        }
    }
}

impl<LeftNodes, RightNodes, E> Graph for GenericBiGraph<LeftNodes, RightNodes, E>
where
    LeftNodes: Vocabulary,
    RightNodes: Vocabulary,
{
    fn is_empty(&self) -> bool {
        self.left_nodes.is_empty() && self.right_nodes.is_empty()
    }
}

impl<LeftNodes, RightNodes, E> BipartiteGraph for GenericBiGraph<LeftNodes, RightNodes, E>
where
    LeftNodes: Vocabulary + VocabularyRef + BidirectionalVocabulary,
    RightNodes: Vocabulary + VocabularyRef + BidirectionalVocabulary,
    LeftNodes::SourceSymbol: PositiveInteger + IntoUsize + TryFromUsize,
    RightNodes::SourceSymbol: PositiveInteger + IntoUsize + TryFromUsize,
{
    type LeftNodeId = LeftNodes::SourceSymbol;
    type RightNodeId = RightNodes::SourceSymbol;
    type LeftNodeSymbol = LeftNodes::DestinationSymbol;
    type RightNodeSymbol = RightNodes::DestinationSymbol;
    type LeftNodes = LeftNodes;
    type RightNodes = RightNodes;

    fn left_nodes_vocabulary(&self) -> &Self::LeftNodes {
        &self.left_nodes
    }

    fn right_nodes_vocabulary(&self) -> &Self::RightNodes {
        &self.right_nodes
    }
}

impl<LeftNodes, RightNodes, E> MonoplexGraph for GenericBiGraph<LeftNodes, RightNodes, E>
where
    E: Edges,
    LeftNodes: Vocabulary,
    RightNodes: Vocabulary,
{
    type Edge = E::Edge;
    type Edges = E;

    fn edges(&self) -> &Self::Edges {
        &self.edges
    }
}
