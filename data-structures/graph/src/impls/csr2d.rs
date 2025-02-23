//! Submodule implementing Edges for CSR2D.

use crate::prelude::*;
use algebra::prelude::*;

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + TryFromUsize + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + From<SparseIndex>,
    > Edges for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    type Edge = <Self as Matrix>::Coordinates;
    type SourceNodeId = RowIndex;
    type DestinationNodeId = ColumnIndex;
    type EdgeId = SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + TryFromUsize + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + From<SparseIndex>,
    > GrowableEdges for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    type GrowableMatrix = Self;
    type Error = algebra::error::MutabilityError<Self>;

    fn matrix_mut(&mut self) -> &mut Self::GrowableMatrix {
        self
    }

    fn with_capacity(number_of_edges: Self::EdgeId) -> Self {
        <Self as SparseMatrixMut>::with_sparse_capacity(number_of_edges)
    }

    fn with_shaped_capacity(
        shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape,
        number_of_edges: Self::EdgeId,
    ) -> Self {
        <Self as SparseMatrixMut>::with_sparse_shaped_capacity(shape, number_of_edges)
    }
}
