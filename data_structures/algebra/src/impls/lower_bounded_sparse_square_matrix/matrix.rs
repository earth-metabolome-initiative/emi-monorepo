//! Submodule implementing the `Matrix` trait and related traits for the
//! `LowerBoundedSquareMatrix` struct.

use super::LowerBoundedSquareMatrix;
use crate::traits::{Matrix, Matrix2D, SquareMatrix};

impl<M> Matrix for LowerBoundedSquareMatrix<M>
where
    M: SquareMatrix,
{
    type Coordinates = M::Coordinates;

    #[inline]
    fn shape(&self) -> Vec<usize> {
        self.matrix.shape()
    }
}

impl<M> Matrix2D for LowerBoundedSquareMatrix<M>
where
    M: SquareMatrix,
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

impl<M> SquareMatrix for LowerBoundedSquareMatrix<M>
where
    M: SquareMatrix,
{
    type Index = M::Index;

    #[inline]
    fn order(&self) -> Self::Index {
        self.matrix.order()
    }
}
