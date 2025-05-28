//! Iterator of the sparse coordinates of the CSR2D matrix.

use core::iter::{RepeatN, repeat_n};

use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::IntoUsize;

use crate::prelude::*;

/// Iterator of the sparse coordinates of the CSR2D matrix.
pub struct CSR2DSizedRows<'a, CSR: SizedRowsSparseMatrix2D> {
    /// The CSR matrix.
    csr2d: &'a CSR,
    /// The row index.
    next_row: CSR::RowIndex,
    /// The end row index.
    back_row: CSR::RowIndex,
    /// The row associated with the index at the beginning of the iteration.
    next: RepeatN<CSR::RowIndex>,
    /// The row associated with the index at the end of the iteration.
    back: RepeatN<CSR::RowIndex>,
}

impl<CSR: SizedRowsSparseMatrix2D> Iterator for CSR2DSizedRows<'_, CSR> {
    type Item = CSR::RowIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.next().or_else(|| {
            self.next_row += CSR::RowIndex::ONE;
            if self.next_row < self.back_row {
                self.next = repeat_n(
                    self.next_row,
                    self.csr2d.number_of_defined_values_in_row(self.next_row).into_usize(),
                );
                self.next.next()
            } else {
                self.back.next()
            }
        })
    }
}

impl<CSR: SizedSparseMatrix2D> ExactSizeIterator for CSR2DSizedRows<'_, CSR> {
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

impl<CSR: SizedRowsSparseMatrix2D> DoubleEndedIterator for CSR2DSizedRows<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.next().or_else(|| {
            self.back_row -= CSR::RowIndex::ONE;
            if self.back_row > self.next_row {
                self.back = repeat_n(
                    self.back_row,
                    self.csr2d.number_of_defined_values_in_row(self.back_row).into_usize(),
                );
                self.back.next()
            } else {
                self.next.next()
            }
        })
    }
}

impl<'a, CSR: SizedRowsSparseMatrix2D> From<&'a CSR> for CSR2DSizedRows<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = csr2d.number_of_rows() - CSR::RowIndex::ONE;
        let next = repeat_n(next_row, csr2d.number_of_defined_values_in_row(next_row).into_usize());
        let back = repeat_n(back_row, csr2d.number_of_defined_values_in_row(back_row).into_usize());
        Self { csr2d, next_row, back_row, next, back }
    }
}
