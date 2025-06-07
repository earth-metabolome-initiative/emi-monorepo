//! Submodule for the transposed graph traits.

use algebra::prelude::*;

use super::Edges;

/// Trait defining the properties of a transposed graph.
pub trait TransposedEdges: super::Edges<Matrix = <Self as TransposedEdges>::BiMatrix> {
    /// The type of matrix required to store the transposed edges.
    type BiMatrix: SizedSparseBiMatrix2D<RowIndex = Self::SourceNodeId, ColumnIndex = Self::DestinationNodeId>;

    /// Returns the predecessors of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id` - The identifier of the destination node.
    fn predecessors(
        &self,
        destination_node_id: Self::DestinationNodeId,
    ) -> <<Self::BiMatrix as SparseBiMatrix2D>::SparseTransposedMatrix as SparseMatrix2D>::SparseRow<'_>
    {
        self.matrix().sparse_column(destination_node_id)
    }

    /// Returns the inbound degree of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `destination_node_id` - The identifier of the destination node.
    fn in_degree(&self, destination_node_id: Self::DestinationNodeId) -> Self::SourceNodeId {
        self.matrix().number_of_defined_values_in_column(destination_node_id)
    }

    /// Returns an iterator over the in-boynd degrees of the nodes.
    fn in_degrees(&self) -> <<Self::BiMatrix as SparseBiMatrix2D>::SparseTransposedMatrix as SizedRowsSparseMatrix2D>::SparseRowSizes<'_>{
        self.matrix().sparse_column_sizes()
    }
}

impl<E: Edges> TransposedEdges for E
where
    E::Matrix:
        SizedSparseBiMatrix2D<RowIndex = E::SourceNodeId, ColumnIndex = E::DestinationNodeId>,
{
    type BiMatrix = E::Matrix;
}

/// Trait defining the properties of a graph.
pub trait TransposedGraph: super::Graph {}

impl<G: super::Graph> TransposedGraph for G {}
