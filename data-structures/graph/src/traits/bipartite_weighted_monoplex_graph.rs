//! Submodule defining a trait for a bipartite weighted monoplex graph.
//!
//! Such a graph is characterized by the fact that:
//!
//! * It is bipartite, i.e. it has two sets of nodes, and all edges can solely
//!   connect nodes from different sets.
//! * It is weighted, i.e. each edge has an associated weight.
//! * It is monoplex, i.e. it has only one type of edge.

use super::{MonoplexBipartiteGraph, WeightedEdges, WeightedMonoplexGraph};

/// Trait implemented by bipartite weighted monoplex graphs.
pub trait BipartiteWeightedMonoplexGraph:
    WeightedMonoplexGraph<
        WeightedEdges = <Self as BipartiteWeightedMonoplexGraph>::BipartiteWeightedEdges,
    > + MonoplexBipartiteGraph<
        MonoplexBipartiteEdges = <Self as BipartiteWeightedMonoplexGraph>::BipartiteWeightedEdges,
    >
{
    /// The underlying type used to represent the edges of the graph.
    type BipartiteWeightedEdges: WeightedEdges<
            SourceNodeId = Self::LeftNodeId,
            DestinationNodeId = Self::RightNodeId,
            Weight = Self::Weight,
            WeightedEdge = Self::Edge,
        >;
}

impl<G> BipartiteWeightedMonoplexGraph for G
where
    G: WeightedMonoplexGraph + MonoplexBipartiteGraph<MonoplexBipartiteEdges = G::WeightedEdges>,
    <G as WeightedMonoplexGraph>::WeightedEdges: WeightedEdges<
            SourceNodeId = G::LeftNodeId,
            DestinationNodeId = G::RightNodeId,
            Weight = G::Weight,
            WeightedEdge = G::Edge,
        >,
{
    type BipartiteWeightedEdges = G::WeightedEdges;
}
