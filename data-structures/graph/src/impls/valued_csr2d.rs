//! Submodule implementing Edges for [`ValuedCSR2D`](algebra::prelude::ValuedCSR2D).

use algebra::prelude::*;

use crate::{errors::builder::edges::EdgesBuilderError, prelude::*};

impl<
        SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
        RowIndex: PositiveInteger + TryFromUsize + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
        Value: Number,
    > Edges for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    type Edge = (RowIndex, ColumnIndex, Value);
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
        ColumnIndex: PositiveInteger + IntoUsize + TryFromUsize + From<SparseIndex>,
        Value: Number,
    > GrowableEdges for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
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
