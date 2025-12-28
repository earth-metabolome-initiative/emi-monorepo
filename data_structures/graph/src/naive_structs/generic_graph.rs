//! Submodule providing a naively implemented generic Monoparted Graph.

use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use crate::traits::{BidirectionalVocabulary, Edges, Graph, MonopartiteGraph, MonoplexGraph};

#[cfg(feature = "arbitrary")]
mod arbitrary_impl;

#[derive(Clone, PartialEq, Eq, Hash)]
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

impl<Nodes, Edges> core::fmt::Debug for GenericGraph<Nodes, Edges>
where
    Nodes: core::fmt::Debug,
    Edges: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GenericGraph")
            .field("nodes", &self.nodes)
            .field("edges", &self.edges)
            .finish()
    }
}

impl<Nodes, Edges> From<(Nodes, Edges)> for GenericGraph<Nodes, Edges> {
    fn from((nodes, edges): (Nodes, Edges)) -> Self {
        Self { nodes, edges }
    }
}

impl<Nodes, E> Graph for GenericGraph<Nodes, E>
where
    Nodes: BidirectionalVocabulary,
    Nodes::SourceSymbol: PositiveInteger + IntoUsize + TryFromUsize,
    E: Edges<SourceNodeId = Nodes::SourceSymbol, DestinationNodeId = Nodes::SourceSymbol>,
{
    fn has_edges(&self) -> bool {
        self.edges.has_edges()
    }

    fn has_nodes(&self) -> bool {
        !self.nodes.is_empty()
    }
}

impl<Nodes, E> MonopartiteGraph for GenericGraph<Nodes, E>
where
    Nodes: BidirectionalVocabulary,
    Nodes::SourceSymbol: PositiveInteger + IntoUsize + TryFromUsize,
    E: Edges<SourceNodeId = Nodes::SourceSymbol, DestinationNodeId = Nodes::SourceSymbol>,
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
    Nodes: BidirectionalVocabulary,
    Nodes::SourceSymbol: PositiveInteger + IntoUsize + TryFromUsize,
    E: Edges<SourceNodeId = Nodes::SourceSymbol, DestinationNodeId = Nodes::SourceSymbol>,
{
    type Edge = E::Edge;
    type Edges = E;

    fn edges(&self) -> &Self::Edges {
        &self.edges
    }
}
