//! Iterator of the sparse coordinates of the CSR2D matrix.

use crate::prelude::*;

/// Iterator of the sparse coordinates of the CSR2D matrix.
pub struct CSR2DRowSizes<'a, CSR: SparseMatrix2D> {
    /// The CSR matrix.
    csr2d: &'a CSR,
    /// The row index.
    next_row: CSR::RowIndex,
    /// The end row index.
    back_row: CSR::RowIndex,
}

impl<CSR: SparseMatrix2D> Iterator for CSR2DRowSizes<'_, CSR> {
    type Item = CSR::ColumnIndex;

    fn next(&mut self) -> Option<Self::Item> {
        (self.next_row <= self.back_row).then(|| {
            let out_degree = self.csr2d.number_of_defined_values_in_row(self.next_row);
            self.next_row += CSR::RowIndex::ONE;
            out_degree
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.back_row + CSR::RowIndex::ONE - self.next_row).into_usize();
        (remaining, Some(remaining))
    }
}

impl<CSR: SparseMatrix2D> ExactSizeIterator for CSR2DRowSizes<'_, CSR> {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<CSR: SparseMatrix2D> DoubleEndedIterator for CSR2DRowSizes<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.next_row <= self.back_row).then(|| {
            self.back_row -= CSR::RowIndex::ONE;
            self.csr2d.number_of_defined_values_in_row(self.back_row)
        })
    }
}

impl<'a, CSR: SparseMatrix2D> From<&'a CSR> for CSR2DRowSizes<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = csr2d.number_of_rows() - CSR::RowIndex::ONE;
        Self { csr2d, next_row, back_row }
    }
}
