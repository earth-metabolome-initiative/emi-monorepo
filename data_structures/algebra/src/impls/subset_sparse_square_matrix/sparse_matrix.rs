//! Submodule providing the implementation of the `SparseMatrix2D` trait
//! and related traits for the `SubsetSquareMatrix` struct.

use core::iter::Copied;

use iter_utils::prelude::*;

use super::SubsetSquareMatrix;
use crate::{
    impls::{CSR2DColumns, CSR2DRows},
    traits::{SparseMatrix, SparseMatrix2D, SquareMatrix, Vector},
};

impl<M, I> SparseMatrix for SubsetSquareMatrix<M, I>
where
    M: SquareMatrix + SparseMatrix2D,
    I: Vector<Value = M::Index>,
{
    type SparseIndex = usize;
    type SparseCoordinates<'a>
        = crate::impls::CSR2DView<'a, Self>
    where
        Self: 'a;

    #[inline]
    fn is_empty(&self) -> bool {
        self.last_sparse_coordinates().is_none()
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        self.sparse_coordinates().next_back()
    }

    #[inline]
    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }
}

impl<M, I> SparseMatrix2D for SubsetSquareMatrix<M, I>
where
    M: SquareMatrix + SparseMatrix2D,
    I: Vector<Value = M::Index>,
{
    type SparseRow<'a>
        = Intersection<M::SparseRow<'a>, Copied<I::Iter<'a>>>
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
        self.matrix.sparse_row(row).sorted_intersection(self.indices.iter().copied())
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
