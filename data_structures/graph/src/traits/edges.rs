//! Trait defining a data structure to handle Edges, such as a simple edge list,
//! or a ragged list or a compressed sparse row matrix.

use algebra::prelude::{
    MatrixMut, SizedRowsSparseMatrix2D, SizedSparseMatrix, SparseMatrix, SparseMatrix2D,
    SparseMatrixMut,
};
use num_traits::{ConstZero, SaturatingAdd};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use super::Edge;

/// Trait defining a data structure to handle Edges, such as a simple edge list,
/// or a ragged list or a compressed sparse row matrix.
pub trait Edges {
    /// The type of the edge.
    type Edge: Edge<SourceNodeId = Self::SourceNodeId, DestinationNodeId = Self::DestinationNodeId>;
    /// The type of the source node ID.
    type SourceNodeId: PositiveInteger + IntoUsize + TryFromUsize + SaturatingAdd;
    /// The type of the destination node ID.
    type DestinationNodeId: PositiveInteger + IntoUsize + TryFromUsize + SaturatingAdd;
    /// The type of the edge identifier.
    type EdgeId: PositiveInteger + IntoUsize + TryFromUsize + SaturatingAdd;
    /// The underlying matrix type.
    type Matrix: SizedRowsSparseMatrix2D<
            RowIndex = Self::SourceNodeId,
            ColumnIndex = Self::DestinationNodeId,
            SparseIndex = Self::EdgeId,
        >;

    /// Returns a reference to the underlying matrix.
    fn matrix(&self) -> &Self::Matrix;

    /// Returns the number of edges.
    fn number_of_edges(&self) -> Self::EdgeId {
        self.matrix().number_of_defined_values()
    }

    /// Returns whether the graph has any edges.
    fn has_edges(&self) -> bool {
        !self.matrix().is_empty()
    }

    /// Returns the successors of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `source` - The identifier of the source node.
    fn successors(
        &self,
        source: Self::SourceNodeId,
    ) -> <Self::Matrix as SparseMatrix2D>::SparseRow<'_> {
        self.matrix().sparse_row(source)
    }

    /// Returns whether the given source has that successor.
    ///
    /// # Arguments
    ///
    /// * `source` - The identifier of the source node.
    /// * `destination` - The identifier of the destination node.
    fn has_successor(
        &self,
        source: Self::SourceNodeId,
        destination: Self::DestinationNodeId,
    ) -> bool {
        self.matrix().has_entry(source, destination)
    }

    /// Returns whether the given source node has successors.
    ///
    /// # Arguments
    ///
    /// * `source` - The identifier of the source node.
    fn has_successors(&self, source: Self::SourceNodeId) -> bool {
        self.out_degree(source) > Self::DestinationNodeId::ZERO
    }

    /// Returns the outbound degree of the node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `source` - The identifier of the source node.
    fn out_degree(&self, source: Self::SourceNodeId) -> Self::DestinationNodeId {
        self.matrix().number_of_defined_values_in_row(source)
    }

    /// Returns an iterator over the out degrees of the nodes.
    fn out_degrees(&self) -> <Self::Matrix as SizedRowsSparseMatrix2D>::SparseRowSizes<'_> {
        self.matrix().sparse_row_sizes()
    }

    /// Returns the iterator of the edges.
    fn sparse_coordinates(&self) -> <Self::Matrix as SparseMatrix>::SparseCoordinates<'_> {
        self.matrix().sparse_coordinates()
    }
}

/// Trait defining a data structure to handle edges that can grow dynamically.
pub trait GrowableEdges: Edges<Matrix = <Self as GrowableEdges>::GrowableMatrix> + Default {
    /// The type of the growable matrix.
    type GrowableMatrix: SparseMatrixMut<Entry = Self::Edge> + SparseMatrix2D;

    /// The error that may be returned when adding an edge.
    type Error: core::error::Error + From<<Self::GrowableMatrix as MatrixMut>::Error>;

    /// Creates a new growable edges representation with the provided graph
    /// shape.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the graph.
    /// * `number_of_edges` - The number of edges.
    fn with_shaped_capacity(
        shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape,
        number_of_edges: Self::EdgeId,
    ) -> Self;

    /// Creates a new growable edges representation with the provided graph
    /// shape.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the graph.
    fn with_shape(shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape) -> Self;

    /// Creates a new growable edges representation with the provided graph
    /// shape.
    ///
    /// # Arguments
    ///
    /// * `number_of_edges` - The number of edges.
    fn with_capacity(number_of_edges: Self::EdgeId) -> Self;

    /// Returns a mutable reference to the underlying matrix.
    fn matrix_mut(&mut self) -> &mut Self::GrowableMatrix;

    /// Adds an edge to the graph.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge to add.
    ///
    /// # Errors
    ///
    /// Returns an error if the entry cannot be added. Possible reasons include:
    /// - The entries are not provided in the expected order.
    /// - The entry is out of bounds.
    /// - The entry is already defined.
    fn add(&mut self, edge: Self::Edge) -> Result<(), Self::Error> {
        Ok(self.matrix_mut().add(edge)?)
    }
}
