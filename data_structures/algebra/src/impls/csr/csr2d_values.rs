//! Iterator of the sparse coordinates of the M2D matrix.

use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::IntoUsize;

use crate::prelude::*;

/// Iterator of the sparse coordinates of the M2D matrix.
pub struct M2DValues<'a, M: SparseValuedMatrix2D> {
    /// The M matrix.
    matrix: &'a M,
    /// The row index.
    next_row: M::RowIndex,
    /// The end row index.
    back_row: M::RowIndex,
    /// The row associated with the index at the beginning of the iteration.
    next: M::SparseRowValues<'a>,
    /// The row associated with the index at the end of the iteration.
    back: M::SparseRowValues<'a>,
}

impl<M: SparseValuedMatrix2D> Iterator for M2DValues<'_, M> {
    type Item = M::Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.next().or_else(|| {
            self.next_row += M::RowIndex::ONE;
            if self.next_row < self.back_row {
                self.next = self.matrix.sparse_row_values(self.next_row);
                self.next.next()
            } else {
                self.back.next()
            }
        })
    }
}

impl<'matrix, M: SizedSparseMatrix2D + SparseValuedMatrix2D> ExactSizeIterator
    for M2DValues<'matrix, M>
where
    M::SparseRowValues<'matrix>: ExactSizeIterator,
{
    fn len(&self) -> usize {
        let next_row_rank = self.matrix.rank_row(self.next_row).into_usize();
        let already_observed_in_next_row =
            self.matrix.number_of_defined_values_in_row(self.next_row).into_usize()
                - self.next.len();
        let back_row_rank = self.matrix.rank_row(self.back_row).into_usize();
        let already_observed_in_back_row =
            self.matrix.number_of_defined_values_in_row(self.back_row).into_usize()
                - self.back.len();
        back_row_rank - next_row_rank - already_observed_in_next_row - already_observed_in_back_row
    }
}

impl<M: SparseValuedMatrix2D> DoubleEndedIterator for M2DValues<'_, M> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.next().or_else(|| {
            self.back_row -= M::RowIndex::ONE;
            if self.back_row > self.next_row {
                self.back = self.matrix.sparse_row_values(self.back_row);
                self.back.next()
            } else {
                self.next.next()
            }
        })
    }
}

impl<'a, M: SparseValuedMatrix2D> From<&'a M> for M2DValues<'a, M> {
    fn from(matrix: &'a M) -> Self {
        let next_row = M::RowIndex::ZERO;
        let back_row = matrix.number_of_rows() - M::RowIndex::ONE;
        let next = matrix.sparse_row_values(next_row);
        let back = matrix.sparse_row_values(back_row);
        Self { matrix, next_row, back_row, next, back }
    }
}
