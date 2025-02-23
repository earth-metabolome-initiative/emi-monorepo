//! Submodule for the transposed directed graph traits.

use algebra::prelude::*;

/// Trait defining the properties of a transposed directed graph.
pub trait TransposedDirectedEdges:
    super::DirectedEdges<DirectedMatrix = <Self as TransposedDirectedEdges>::BiMatrix>
{
    /// The type of matrix required to store the transposed edges.
    type BiMatrix: SparseBiMatrix2D<
        RowIndex = Self::SourceNodeId,
        ColumnIndex = Self::DestinationNodeId,
    >;

    /// Returns the predecessors of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `destination` - The identifier of the destination node.
    ///
    fn predecessors(
        &self,
        destination: Self::DestinationNodeId,
    ) -> <<Self::BiMatrix as SparseBiMatrix2D>::SparseTransposedMatrix as SparseMatrix2D>::SparseRow<'_>
    {
        self.matrix().sparse_column(destination)
    }

    /// Returns the inbound degree of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `destination` - The identifier of the destination node.
    ///
    fn in_degree(&self, destination: Self::DestinationNodeId) -> Self::SourceNodeId {
        self.matrix().number_of_defined_values_in_column(destination)
    }
}

/// Trait defining the properties of a directed graph.
pub trait TransposedDirectedGraph:
    super::DirectedGraph<DirectedEdges = <Self as TransposedDirectedGraph>::TransposedDirectedEdges>
{
    /// The directed edges of the graph.
    type TransposedDirectedEdges: TransposedDirectedEdges<
        SourceNodeId = Self::SourceNodeId,
        DestinationNodeId = Self::DestinationNodeId,
        Edge = Self::Edge,
    >;

    /// Returns the predecessors of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `destination` - The identifier of the destination node.
    ///
    fn predecessors(
        &self,
        destination: Self::DestinationNodeId,
    ) -> <<<Self::TransposedDirectedEdges as TransposedDirectedEdges>::BiMatrix as SparseBiMatrix2D>::SparseTransposedMatrix as SparseMatrix2D>::SparseRow<'_>{
        self.edges().predecessors(destination)
    }

    /// Returns the inbound degree of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `destination` - The identifier of the destination node.
    ///
    fn in_degree(&self, destination: Self::DestinationNodeId) -> Self::SourceNodeId {
        self.edges().in_degree(destination)
    }
}
