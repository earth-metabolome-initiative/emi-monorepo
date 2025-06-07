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

    /// Returns a DOT representation for the Monoplex Bipartite Graph.
    fn to_mb_dot(&self) -> String {
        use std::fmt::Write;
        let mut dot = String::new();
        writeln!(dot, "  graph {{").unwrap();

        for left_node_id in self.left_node_ids() {
            writeln!(dot, "  L{left_node_id} [color=red];").unwrap();
        }

        for right_node_id in self.right_node_ids() {
            writeln!(dot, "  R{right_node_id} [color=blue];").unwrap();
        }

        for (src, dst) in self.edges().sparse_coordinates() {
            writeln!(dot, "  L{src} -> R{dst};").unwrap();
        }

        writeln!(dot, "  }}").unwrap();
        dot
    }
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
