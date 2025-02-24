//! Submodule implementing Edges for CSR2D.

use crate::{errors::builder::edges::EdgesBuilderError, prelude::*};
use algebra::prelude::*;

impl<
        SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
        RowIndex: PositiveInteger + TryFromUsize + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
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
        SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
        RowIndex: PositiveInteger + TryFromUsize + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + TryFrom<SparseIndex> + TryFromUsize,
    > GrowableEdges for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    type GrowableMatrix = Self;
    type Error = EdgesBuilderError<Self>;

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
