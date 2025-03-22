//! Iterator of the sparse coordinates of the [`ValuedCSR2D`] matrix.

use crate::prelude::*;

/// Iterator of the sparse coordinates of the [`ValuedCSR2D`] matrix.
pub struct ValuedCSR2DColumns<'a, CSR: ValuedSparseMatrix2D> {
    /// The CSR matrix.
    csr2d: &'a CSR,
    /// The row index.
    next_row: CSR::RowIndex,
    /// The end row index.
    back_row: CSR::RowIndex,
    /// The row associated with the index at the beginning of the iteration.
    next: CSR::SparseRowValues<'a>,
    /// The row associated with the index at the end of the iteration.
    back: CSR::SparseRowValues<'a>,
}

impl<CSR: ValuedSparseMatrix2D> Iterator for ValuedCSR2DColumns<'_, CSR> {
    type Item = CSR::Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.next().or_else(|| {
            self.next_row += CSR::RowIndex::ONE;
            if self.next_row < self.back_row {
                self.next = self.csr2d.sparse_row_values(self.next_row);
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

impl<CSR: ValuedSparseMatrix2D> ExactSizeIterator for ValuedCSR2DColumns<'_, CSR> {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<CSR: ValuedSparseMatrix2D> DoubleEndedIterator for ValuedCSR2DColumns<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.next().or_else(|| {
            self.back_row -= CSR::RowIndex::ONE;
            if self.back_row > self.next_row {
                self.back = self.csr2d.sparse_row_values(self.back_row);
                self.back.next()
            } else {
                self.next.next()
            }
        })
    }
}

impl<'a, CSR: ValuedSparseMatrix2D> From<&'a CSR> for ValuedCSR2DColumns<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = csr2d.number_of_rows() - CSR::RowIndex::ONE;
        let next = csr2d.sparse_row_values(next_row);
        let back = csr2d.sparse_row_values(back_row);
        Self { csr2d, next_row, back_row, next, back }
    }
}
