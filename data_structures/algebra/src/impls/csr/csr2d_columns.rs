//! Iterator of the sparse coordinates of the CSR2D matrix.

use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::IntoUsize;

use crate::prelude::*;

/// Iterator of the sparse coordinates of the CSR2D matrix.
pub struct CSR2DColumns<'a, CSR: SparseMatrix2D> {
    /// The CSR matrix.
    csr2d: &'a CSR,
    /// The row index.
    next_row: CSR::RowIndex,
    /// The end row index.
    back_row: CSR::RowIndex,
    /// The row associated with the index at the beginning of the iteration.
    next: CSR::SparseRow<'a>,
    /// The row associated with the index at the end of the iteration.
    back: CSR::SparseRow<'a>,
}

impl<CSR: SparseMatrix2D> Iterator for CSR2DColumns<'_, CSR> {
    type Item = CSR::ColumnIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.next().or_else(|| {
            self.next_row += CSR::RowIndex::ONE;
            if self.next_row < self.back_row {
                self.next = self.csr2d.sparse_row(self.next_row);
                self.next.next()
            } else {
                self.back.next()
            }
        })
    }
}

impl<'matrix, CSR: SizedSparseMatrix2D> ExactSizeIterator for CSR2DColumns<'matrix, CSR>
where
    CSR::SparseRow<'matrix>: ExactSizeIterator,
{
    fn len(&self) -> usize {
        let next_row_rank = self.csr2d.rank_row(self.next_row).into_usize();
        let already_observed_in_next_row =
            self.csr2d.number_of_defined_values_in_row(self.next_row).into_usize()
                - self.next.len();
        let back_row_rank = self.csr2d.rank_row(self.back_row).into_usize();
        let already_observed_in_back_row =
            self.csr2d.number_of_defined_values_in_row(self.back_row).into_usize()
                - self.back.len();
        back_row_rank - next_row_rank - already_observed_in_next_row - already_observed_in_back_row
    }
}

impl<CSR: SparseMatrix2D> DoubleEndedIterator for CSR2DColumns<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.next().or_else(|| {
            self.back_row -= CSR::RowIndex::ONE;
            if self.back_row > self.next_row {
                self.back = self.csr2d.sparse_row(self.back_row);
                self.back.next()
            } else {
                self.next.next()
            }
        })
    }
}

impl<'a, CSR: SparseMatrix2D> From<&'a CSR> for CSR2DColumns<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = csr2d.number_of_rows() - CSR::RowIndex::ONE;
        let next = csr2d.sparse_row(next_row);
        let back = csr2d.sparse_row(back_row);
        Self { csr2d, next_row, back_row, next, back }
    }
}
