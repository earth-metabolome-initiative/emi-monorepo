//! Submodule implementing the `Matrix` trait and related traits for the
//! `SubsetSquareMatrix` struct.

use super::SubsetSquareMatrix;
use crate::traits::{Matrix, Matrix2D, SparseMatrix2D, SquareMatrix};

impl<M, I> Matrix for SubsetSquareMatrix<M, I>
where
    M: SquareMatrix + SparseMatrix2D,
{
    type Coordinates = M::Coordinates;

    #[inline]
    fn shape(&self) -> Vec<usize> {
        self.matrix.shape()
    }
}

impl<M, I> Matrix2D for SubsetSquareMatrix<M, I>
where
    M: SquareMatrix + SparseMatrix2D,
{
    type RowIndex = M::RowIndex;
    type ColumnIndex = M::ColumnIndex;

    #[inline]
    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_columns()
    }

    #[inline]
    fn number_of_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_rows()
    }
}
