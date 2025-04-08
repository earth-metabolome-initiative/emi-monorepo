//! Submodule defining a trait characterizing monoplex bipartite graphs.
//!
//! These graphs are characterized by the fact that:
//!
//! * They are bipartite, i.e., they have two types of nodes.
//! * They are monoplex, i.e., they have only one type of edges.

use algebra::prelude::{IntoUsize, SparseMatrix2D, Zero};

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

    /// Returns whether the left partition contains singletons.
    fn left_partition_has_singletons(&self) -> bool {
        self.number_of_singletons_in_left_partition() > <Self as BipartiteGraph>::LeftNodeId::ZERO
    }

    /// Returns the number of singletons in the left partition.
    fn number_of_singletons_in_left_partition(&self) -> <Self as BipartiteGraph>::LeftNodeId {
        self.edges().matrix().number_of_empty_rows()
    }

    /// Returns the number of non-singletons in the left partition.
    fn number_of_non_singletons_in_left_partition(&self) -> <Self as BipartiteGraph>::LeftNodeId {
        self.edges().matrix().number_of_non_empty_rows()
    }

    /// Returns an iterator over the singletons in the left partition.
    fn left_partition_singleton_ids(
        &self,
    ) -> <<Self::MonoplexBipartiteEdges as Edges>::Matrix as SparseMatrix2D>::EmptyRowIndices<'_>
    {
        self.edges().matrix().empty_row_indices()
    }

    /// Returns an iterator over the non-singletons in the left partition.
    fn left_partition_non_singleton_ids(
        &self,
    ) -> <<Self::MonoplexBipartiteEdges as Edges>::Matrix as SparseMatrix2D>::NonEmptyRowIndices<'_>
    {
        self.edges().matrix().non_empty_row_indices()
    }

    /// Returns a DOT representation for the Monoplex Bipartite Graph.
    fn to_mb_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("graph {\n");

        for left_node_id in self.left_node_ids() {
            dot.push_str(&format!("  L{} [color=red];\n", left_node_id.into_usize()));
        }

        for right_node_id in self.right_node_ids() {
            dot.push_str(&format!("  R{} [color=blue];\n", right_node_id.into_usize()));
        }

        for (src, dst) in self.edges().sparse_coordinates() {
            dot.push_str(&format!("  L{} -> R{};\n", src.into_usize(), dst.into_usize()));
        }

        dot.push_str("}\n");
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
