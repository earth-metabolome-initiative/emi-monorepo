//! Submodule providing a naively implemented generic Monoparted Graph.

use algebra::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use super::generic_monoplex_monopartite_graph_builder::MonoplexMonopartiteGraphBuilderError;
use crate::traits::{
    BidirectionalVocabulary, Edges, Graph, MonopartiteGraph, MonoplexGraph, Vocabulary,
    VocabularyRef,
};

/// Struct representing a generic graph.
pub struct GenericGraph<Nodes, Edges> {
    /// The nodes of the graph.
    nodes: Nodes,
    /// The edges of the graph.
    edges: Edges,
}

impl<Nodes, Edges> Default for GenericGraph<Nodes, Edges>
where
    Nodes: Default,
    Edges: Default,
{
    fn default() -> Self {
        Self { nodes: Nodes::default(), edges: Edges::default() }
    }
}

impl<Nodes, Edges> TryFrom<(Nodes, Edges)> for GenericGraph<Nodes, Edges> {
    type Error = MonoplexMonopartiteGraphBuilderError;

    fn try_from((nodes, edges): (Nodes, Edges)) -> Result<Self, Self::Error> {
        Ok(Self { nodes, edges })
    }
}

impl<Nodes, Edges> Graph for GenericGraph<Nodes, Edges>
where
    Nodes: Vocabulary,
{
    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl<Nodes, Edges> MonopartiteGraph for GenericGraph<Nodes, Edges>
where
    Nodes: VocabularyRef + BidirectionalVocabulary,
    Nodes::SourceSymbol: PositiveInteger + IntoUsize + TryFromUsize,
{
    type NodeId = Nodes::SourceSymbol;
    type NodeSymbol = Nodes::DestinationSymbol;
    type Nodes = Nodes;

    fn nodes_vocabulary(&self) -> &Self::Nodes {
        &self.nodes
    }
}

impl<Nodes, E> MonoplexGraph for GenericGraph<Nodes, E>
where
    Nodes: Vocabulary,
    E: Edges,
{
    type Edge = E::Edge;
    type Edges = E;

    fn edges(&self) -> &Self::Edges {
        &self.edges
    }
}
