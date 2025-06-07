//! Iterator of the sparse coordinates of the CSR2D matrix.

use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::IntoUsize;

use crate::prelude::*;

/// Iterator of the sparse coordinates of the CSR2D matrix.
pub struct CSR2DSizedRowsizes<'a, CSR: SizedRowsSparseMatrix2D> {
    /// The CSR matrix.
    csr2d: &'a CSR,
    /// The row index.
    next_row: CSR::RowIndex,
    /// The end row index.
    back_row: CSR::RowIndex,
}

impl<CSR: SizedRowsSparseMatrix2D> Iterator for CSR2DSizedRowsizes<'_, CSR> {
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

impl<CSR: SizedRowsSparseMatrix2D> ExactSizeIterator for CSR2DSizedRowsizes<'_, CSR> {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<CSR: SizedRowsSparseMatrix2D> DoubleEndedIterator for CSR2DSizedRowsizes<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.next_row <= self.back_row).then(|| {
            self.back_row -= CSR::RowIndex::ONE;
            self.csr2d.number_of_defined_values_in_row(self.back_row)
        })
    }
}

impl<'a, CSR: SizedRowsSparseMatrix2D> From<&'a CSR> for CSR2DSizedRowsizes<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = csr2d.number_of_rows() - CSR::RowIndex::ONE;
        Self { csr2d, next_row, back_row }
    }
}
