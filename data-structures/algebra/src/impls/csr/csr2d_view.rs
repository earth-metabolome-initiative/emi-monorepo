//! Iterator of the sparse coordinates of the CSR2D matrix.

use crate::prelude::*;

/// Iterator of the sparse coordinates of the CSR2D matrix.
pub struct CSR2DView<'a, CSR: SparseMatrix2D> {
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

impl<CSR: SparseMatrix2D> Iterator for CSR2DView<'_, CSR> {
    type Item = CSR::Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.next().map(|column_index| (self.next_row, column_index)).or_else(|| {
            self.next_row += CSR::RowIndex::ONE;
            if self.next_row < self.back_row {
                self.next = self.csr2d.sparse_row(self.next_row);
                self.next()
            } else {
                self.back.next().map(|column_index| (self.back_row, column_index))
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let next_row_rank = self.csr2d.rank(self.next_row);
        let already_observed_in_next_row =
            self.csr2d.number_of_defined_values_in_row(self.next_row).into_usize()
                - self.next.len();
        let back_row_rank = self.csr2d.rank(self.back_row);
        let still_to_be_observed_in_back_row = self.back.len();
        let remaining = back_row_rank + still_to_be_observed_in_back_row
            - next_row_rank
            - already_observed_in_next_row;
        (remaining, Some(remaining))
    }
}

impl<CSR: SparseMatrix2D> ExactSizeIterator for CSR2DView<'_, CSR> {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<CSR: SparseMatrix2D> DoubleEndedIterator for CSR2DView<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.next().map(|column_index| (self.back_row, column_index)).or_else(|| {
            self.back_row -= CSR::RowIndex::ONE;
            if self.back_row > self.next_row {
                self.back = self.csr2d.sparse_row(self.back_row);
                self.next_back()
            } else {
                self.next.next().map(|column_index| (self.next_row, column_index))
            }
        })
    }
}

impl<'a, CSR: SparseMatrix2D> From<&'a CSR> for CSR2DView<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = csr2d.number_of_rows() - CSR::RowIndex::ONE;
        let next = csr2d.sparse_row(next_row);
        let back = csr2d.sparse_row(back_row);
        Self { csr2d, next_row, back_row, next, back }
    }
}
