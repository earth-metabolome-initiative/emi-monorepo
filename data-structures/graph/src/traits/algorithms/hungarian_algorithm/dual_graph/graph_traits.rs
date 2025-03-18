//! Submodule providing the implementation of graph traits for the `Dual`
//! struct.

use super::{Dual, DualEdges};
use crate::traits::{BipartiteGraph, BipartiteWeightedMonoplexGraph, Graph, MonoplexGraph};

impl<G: BipartiteWeightedMonoplexGraph + ?Sized> Graph for Dual<'_, G> {
    fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }
}
impl<G: BipartiteWeightedMonoplexGraph + ?Sized> BipartiteGraph for Dual<'_, G> {
    type LeftNodeId = G::LeftNodeId;
    type LeftNodeSymbol = G::LeftNodeSymbol;
    type RightNodeId = G::RightNodeId;
    type RightNodeSymbol = G::RightNodeSymbol;
    type LeftNodes = G::LeftNodes;
    type RightNodes = G::RightNodes;

    fn left_nodes_vocabulary(&self) -> &Self::LeftNodes {
        self.graph.left_nodes_vocabulary()
    }

    fn right_nodes_vocabulary(&self) -> &Self::RightNodes {
        self.graph.right_nodes_vocabulary()
    }
}

impl<'graph, G: BipartiteWeightedMonoplexGraph + ?Sized> MonoplexGraph for Dual<'graph, G> {
    type Edge = G::Edge;
    type Edges = DualEdges<'graph, G::Edges>;

    fn edges(&self) -> &Self::Edges {
        &self.edges
    }
}
