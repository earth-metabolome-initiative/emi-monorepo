//! Submodule defining an undirected bipartite graph.
//!
//! Such graph as the following properties:
//!
//! * Each node is either in the left or right partition.
//! * All edges are bidirectional, and an edge can only connect nodes from
//!   different partitions.

use super::{MonoplexBipartiteGraph, TransposedEdges, TransposedMonoplexGraph};

/// Trait defining the properties of an undirected bipartite graph.
pub trait UndirectedBipartiteMonoplexGraph: TransposedMonoplexGraph<
    TransposedEdges = <Self as UndirectedBipartiteMonoplexGraph>::UndirectedBipartiteEdges,
> + MonoplexBipartiteGraph<
    MonoplexBipartiteEdges = <Self as UndirectedBipartiteMonoplexGraph>::UndirectedBipartiteEdges,
> {
    /// The edges data structure of the graph.
    type UndirectedBipartiteEdges: TransposedEdges<
        SourceNodeId = Self::LeftNodeId,
        DestinationNodeId = Self::RightNodeId,
    >;
}

impl<G> UndirectedBipartiteMonoplexGraph for G
where
    G: TransposedMonoplexGraph
        + MonoplexBipartiteGraph<MonoplexBipartiteEdges = G::TransposedEdges>,
    G::TransposedEdges:
        TransposedEdges<SourceNodeId = G::LeftNodeId, DestinationNodeId = G::RightNodeId>,
{
    type UndirectedBipartiteEdges = G::TransposedEdges;
}
