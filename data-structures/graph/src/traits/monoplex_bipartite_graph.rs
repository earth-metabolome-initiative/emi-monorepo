//! Submodule defining a trait characterizing monoplex bipartite graphs.
//!
//! These graphs are characterized by the fact that:
//!
//! * They are bipartite, i.e., they have two types of nodes.
//! * They are monoplex, i.e., they have only one type of edges.

use super::{BipartiteGraph, Edges, MonoplexGraph};

/// Trait defining the properties of a monoplex bipartite graph.
pub trait MonoplexBipartiteGraph:
    MonoplexGraph<Edges = <Self as MonoplexBipartiteGraph>::MonoplexBipartiteEdges> + BipartiteGraph
{
    /// The edges of the graph.
    type MonoplexBipartiteEdges: Edges<
        SourceNodeId = <Self as BipartiteGraph>::LeftNodeId,
        DestinationNodeId = <Self as BipartiteGraph>::RightNodeId,
    >;
}

impl<G> MonoplexBipartiteGraph for G
where
    G: MonoplexGraph,
    G: BipartiteGraph,
    G::Edges: Edges<
        SourceNodeId = <G as BipartiteGraph>::LeftNodeId,
        DestinationNodeId = <G as BipartiteGraph>::RightNodeId,
    >,
{
    type MonoplexBipartiteEdges = G::Edges;
}
