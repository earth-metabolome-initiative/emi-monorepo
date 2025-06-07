//! Iterator of the sparse coordinates of the CSR2D matrix.

use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::IntoUsize;

use crate::prelude::*;

/// Iterator of the sparse coordinates of the CSR2D matrix.
pub struct CSR2DRows<'a, CSR: SparseMatrix2D> {
    /// The CSR matrix.
    matrix: &'a CSR,
    /// The row index.
    next_row: CSR::RowIndex,
    /// The end row index.
    back_row: CSR::RowIndex,
    /// The row associated with the index at the beginning of the iteration.
    next: CSR::SparseRow<'a>,
    /// The row associated with the index at the end of the iteration.
    back: CSR::SparseRow<'a>,
}

impl<CSR: SparseMatrix2D> Iterator for CSR2DRows<'_, CSR> {
    type Item = CSR::RowIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.next
            .next()
            .or_else(|| {
                self.next_row += CSR::RowIndex::ONE;
                if self.next_row < self.back_row {
                    self.next = self.matrix.sparse_row(self.next_row);
                    self.next.next()
                } else {
                    self.back.next()
                }
            })
            .map(|_| self.next_row)
    }
}

impl<CSR: SizedSparseMatrix2D> ExactSizeIterator for CSR2DRows<'_, CSR>
where
    for<'a> CSR::SparseRow<'a>: ExactSizeIterator,
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

impl<CSR: SparseMatrix2D> DoubleEndedIterator for CSR2DRows<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back
            .next()
            .or_else(|| {
                self.back_row -= CSR::RowIndex::ONE;
                if self.back_row > self.next_row {
                    self.back = self.matrix.sparse_row(self.back_row);
                    self.back.next()
                } else {
                    self.next.next()
                }
            })
            .map(|_| self.back_row)
    }
}

impl<'a, CSR: SparseMatrix2D> From<&'a CSR> for CSR2DRows<'a, CSR> {
    fn from(matrix: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = matrix.number_of_rows() - CSR::RowIndex::ONE;
        let next = matrix.sparse_row(next_row);
        let back = matrix.sparse_row(back_row);
        Self { matrix, next_row, back_row, next, back }
    }
}
