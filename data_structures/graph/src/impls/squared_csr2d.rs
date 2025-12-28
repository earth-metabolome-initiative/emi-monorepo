//! Submodule implementing Edges-related traits for [`SquaredSquareCSR2D`].

use std::fmt::Debug;

use algebra::prelude::*;
use numeric_common_traits::prelude::TryFromUsize;

use crate::{errors::builder::edges::EdgesBuilderError, prelude::*};

impl<M> Edges for SquareCSR2D<M>
where
    M: SizedRowsSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
    M::RowIndex: TryFromUsize,
    M::SparseIndex: TryFromUsize,
{
    type Edge = <Self as Matrix>::Coordinates;
    type SourceNodeId = <Self as Matrix2D>::RowIndex;
    type DestinationNodeId = <Self as Matrix2D>::ColumnIndex;
    type EdgeId = <Self as SparseMatrix>::SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<M> GrowableEdges for SquareCSR2D<M>
where
    M: Debug
        + SparseMatrixMut<
            MinimalShape = <Self as Matrix>::Coordinates,
            Entry = <Self as Matrix>::Coordinates,
            Error = MutabilityError<M>,
        > + SizedRowsSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>
        + 'static,
    M::RowIndex: TryFromUsize,
    M::SparseIndex: TryFromUsize,
{
    type GrowableMatrix = Self;
    type Error = EdgesBuilderError<Self>;

    fn matrix_mut(&mut self) -> &mut Self::GrowableMatrix {
        self
    }

    fn with_capacity(number_of_edges: Self::EdgeId) -> Self {
        <Self as SparseMatrixMut>::with_sparse_capacity(number_of_edges)
    }

    fn with_shape(shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape) -> Self {
        <Self as SparseMatrixMut>::with_sparse_shape(shape)
    }

    fn with_shaped_capacity(
        shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape,
        number_of_edges: Self::EdgeId,
    ) -> Self {
        <Self as SparseMatrixMut>::with_sparse_shaped_capacity(shape, number_of_edges)
    }
}
