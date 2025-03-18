//! Submodule defining a trait characterizing monoplex monopartite graphs.
//!
//! These graphs are characterized by the fact that:
//!
//! * They are monopartite, i.e., they have only one type of nodes.
//! * They are monoplex, i.e., they have only one type of edges.

use super::{MonopartiteEdges, MonopartiteGraph, MonoplexGraph};

/// Trait defining the properties of monoplex monopartite graphs.
pub trait MonoplexMonopartiteGraph: MonoplexGraph<Edges = <Self as MonoplexMonopartiteGraph>::MonoplexMonopartiteEdges>
    + MonopartiteGraph<NodeId = <<Self as MonoplexMonopartiteGraph>::MonoplexMonopartiteEdges as MonopartiteEdges>::NodeId>
{
    /// The type of edges in the graph.
    type MonoplexMonopartiteEdges: MonopartiteEdges;

    /// Returns whether the graph has self-loops.
    fn has_self_loops(&self) -> bool {
        self.edges().has_self_loops()
    }

    /// Returns the number of self-loops in the graph.
    fn number_of_self_loops(&self) -> Self::NodeId {
        self.edges().number_of_self_loops()
    }
}

impl<G> MonoplexMonopartiteGraph for G
where
    G: MonopartiteGraph + MonoplexGraph,
    G::Edges: MonopartiteEdges<NodeId = G::NodeId>,
{
    type MonoplexMonopartiteEdges = G::Edges;
}
