//! Submodule defining a Transposed Monoplex Graph.
//!
//! A transposed monoplex graph is a graph where the edges are of a single type
//! and it is possible to efficiently access the predecessors of a node.

use super::TransposedEdges;
use crate::traits::{MonopartiteEdges, MonoplexMonopartiteGraph, TransposedMonoplexGraph};

/// Trait defining a transposed monoplex graph.
pub trait TransposedMonoplexMonopartiteGraph: TransposedMonoplexGraph<
    TransposedEdges= <Self as TransposedMonoplexMonopartiteGraph>::TransposedMonoplexMonopartiteEdges,
>
    + MonoplexMonopartiteGraph<MonoplexMonopartiteEdges = <Self as TransposedMonoplexMonopartiteGraph>::TransposedMonoplexMonopartiteEdges>
{
    /// The type of edges in the transposed monoplex monopartite graph.
    type TransposedMonoplexMonopartiteEdges: TransposedEdges + MonopartiteEdges<MonopartiteMatrix =
        <Self::TransposedMonoplexMonopartiteEdges as TransposedEdges>::BiMatrix
    >;

    /// Returns whether the provided node is a singleton, i.e., it has no
    /// incoming or outgoing edges.
    fn is_singleton(&self, node: Self::NodeId) -> bool {
        !self.has_successors(node) && !self.has_predecessors(node)
    }
}
