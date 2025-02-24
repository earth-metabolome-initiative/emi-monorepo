//! Iterator of the sparse coordinates of the CSR2D matrix.

use core::iter::{repeat, Repeat, Take};

use crate::prelude::*;

/// Iterator of the sparse coordinates of the CSR2D matrix.
pub struct CSR2DRows<'a, CSR: SparseMatrix2D> {
    /// The CSR matrix.
    csr2d: &'a CSR,
    /// The row index.
    next_row: CSR::RowIndex,
    /// The end row index.
    back_row: CSR::RowIndex,
    /// The row associated with the index at the beginning of the iteration.
    next: Take<Repeat<CSR::RowIndex>>,
    /// The row associated with the index at the end of the iteration.
    back: Take<Repeat<CSR::RowIndex>>,
}

impl<CSR: SparseMatrix2D> Iterator for CSR2DRows<'_, CSR> {
    type Item = CSR::RowIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.next().or_else(|| {
            self.next_row += CSR::RowIndex::ONE;
            if self.next_row < self.back_row {
                self.next = repeat(self.next_row)
                    .take(self.csr2d.number_of_defined_values_in_row(self.next_row).into_usize());
                self.next.next()
            } else {
                self.back.next()
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let next_row_rank = self.csr2d.rank(self.next_row).into_usize();
        let already_observed_in_next_row =
            self.csr2d.number_of_defined_values_in_row(self.next_row).into_usize()
                - self.next.len();
        let back_row_rank = self.csr2d.rank(self.back_row).into_usize();
        let already_observed_in_back_row =
            self.csr2d.number_of_defined_values_in_row(self.back_row).into_usize()
                - self.back.len();
        let remaining = back_row_rank
            - next_row_rank
            - already_observed_in_next_row
            - already_observed_in_back_row;
        (remaining, Some(remaining))
    }
}

impl<CSR: SparseMatrix2D> ExactSizeIterator for CSR2DRows<'_, CSR> {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<CSR: SparseMatrix2D> DoubleEndedIterator for CSR2DRows<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.next().or_else(|| {
            self.back_row -= CSR::RowIndex::ONE;
            if self.back_row > self.next_row {
                self.back = repeat(self.back_row)
                    .take(self.csr2d.number_of_defined_values_in_row(self.back_row).into_usize());
                self.back.next()
            } else {
                self.next.next()
            }
        })
    }
}

impl<'a, CSR: SparseMatrix2D> From<&'a CSR> for CSR2DRows<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = csr2d.number_of_rows() - CSR::RowIndex::ONE;
        let next =
            repeat(next_row).take(csr2d.number_of_defined_values_in_row(next_row).into_usize());
        let back =
            repeat(back_row).take(csr2d.number_of_defined_values_in_row(back_row).into_usize());
        Self { csr2d, next_row, back_row, next, back }
    }
}
