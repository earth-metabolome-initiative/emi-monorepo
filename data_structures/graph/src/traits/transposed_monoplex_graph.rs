//! Submodule defining a Transposed Monoplex Graph.
//!
//! A transposed monoplex graph is a graph where the edges are of a single type
//! and it is possible to efficiently access the predecessors of a node.

use algebra::prelude::{SizedRowsSparseMatrix2D, SparseBiMatrix2D, SparseMatrix2D};

use super::{Edges, MonoplexGraph, TransposedEdges};

/// Trait defining a transposed monoplex graph.
pub trait TransposedMonoplexGraph:
    MonoplexGraph<Edges = <Self as TransposedMonoplexGraph>::TransposedEdges>
{
    /// The types of the edges.
    type TransposedEdges: TransposedEdges;

    /// Returns the predecessors of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id` - The identifier of the destination node.
	fn predecessors(
		&self,
		destination_node_id: <Self::TransposedEdges as Edges>::DestinationNodeId,
    ) -> <<<Self::TransposedEdges as TransposedEdges>::BiMatrix as SparseBiMatrix2D>::SparseTransposedMatrix as SparseMatrix2D>::SparseRow<'_>{
        self.edges().predecessors(destination_node_id)
    }

    /// Returns whether the given destination node has a predecessor with the
    /// given source node identifier.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id` - The identifier of the destination node.
    /// * `source_node_id` - The identifier of the source node.
    fn has_predecessor(
        &self,
        destination_node_id: <Self::TransposedEdges as Edges>::DestinationNodeId,
        source_node_id: <Self::TransposedEdges as Edges>::SourceNodeId,
    ) -> bool {
        self.edges().has_predecessor(destination_node_id, source_node_id)
    }

    /// Returns whether the given destination node has any predecessor.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id` - The identifier of the destination node.
    fn has_predecessors(
        &self,
        destination_node_id: <Self::TransposedEdges as Edges>::DestinationNodeId,
    ) -> bool {
        self.edges().has_predecessors(destination_node_id)
    }

    /// Returns the inbound degree of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id` - The identifier of the destination node.
    ///
    /// # Returns
    ///
    /// The inbound degree of the node.
    fn in_degree(
        &self,
        destination_node_id: <Self::TransposedEdges as Edges>::DestinationNodeId,
    ) -> <Self::TransposedEdges as Edges>::SourceNodeId {
        self.edges().in_degree(destination_node_id)
    }

    /// Returns an iterator over the inbound degrees of the nodes.
    ///
    /// # Returns
    ///
    /// An iterator over the inbound degrees of the nodes.
	fn in_degrees(
		&self
    ) -> <<<Self::TransposedEdges as TransposedEdges>::BiMatrix as SparseBiMatrix2D>::SparseTransposedMatrix as SizedRowsSparseMatrix2D>::SparseRowSizes<'_>{
        self.edges().in_degrees()
    }
}

impl<G> TransposedMonoplexGraph for G
where
    G: MonoplexGraph,
    G::Edges: TransposedEdges,
{
    type TransposedEdges = G::Edges;
}
