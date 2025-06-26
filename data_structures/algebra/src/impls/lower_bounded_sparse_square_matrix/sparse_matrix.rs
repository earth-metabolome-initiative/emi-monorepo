//! Submodule providing the implementation of the `SparseMatrix2D` trait
//! and related traits for the `LowerBoundedSquareMatrix` struct.

use super::{LowerBoundedSquareMatrix, lower_bounded_sparse_row::LowerBoundedSparseRow};
use crate::{
    impls::{CSR2DColumns, CSR2DRows},
    traits::{SparseMatrix, SparseMatrix2D, SquareMatrix},
};

impl<M> SparseMatrix for LowerBoundedSquareMatrix<M>
where
    M: SquareMatrix + SparseMatrix2D,
{
    type SparseIndex = usize;
    type SparseCoordinates<'a>
        = crate::impls::CSR2DView<'a, Self>
    where
        Self: 'a;

    #[inline]
    fn is_empty(&self) -> bool {
        // A lower bounded sparse square matrix is empty if it has no defined
        // values above the threshold index. For a coordinate to be considered
        // as defined in this filtered case, all of its indices must be larger
        // or equal to the threshold index. Therefore, we can use a method that
        // returns the last sparse entry in the matrix, which is necessarily the
        // value at the last defined row and column. If any of the indices of
        // this entry is less than the threshold index, then the matrix is empty.
        self.last_sparse_coordinates().is_none()
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        let (last_row, last_column) = self.matrix.last_sparse_coordinates()?;
        if last_row < self.index || last_column < self.index {
            return None;
        }
        Some((last_row, last_column))
    }

    #[inline]
    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }
}

impl<M> SparseMatrix2D for LowerBoundedSquareMatrix<M>
where
    M: SquareMatrix + SparseMatrix2D,
{
    type SparseRow<'a>
        = LowerBoundedSparseRow<'a, M>
    where
        Self: 'a;
    type SparseColumns<'a>
        = CSR2DColumns<'a, Self>
    where
        Self: 'a;
    type SparseRows<'a>
        = CSR2DRows<'a, Self>
    where
        Self: 'a;

    #[inline]
    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        LowerBoundedSparseRow::new(self, row)
    }

    #[inline]
    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        self.sparse_row(row).any(|col| col == column)
    }

    #[inline]
    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.into()
    }

    #[inline]
    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.into()
    }
}
