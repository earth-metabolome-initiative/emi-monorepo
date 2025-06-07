//! Submodule providing an iterator over the indices of non-empty rows in a CSR
//! matrix.

use multi_ranged::SimpleRange;
use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::IntoUsize;

use crate::traits::SizedRowsSparseMatrix2D;

/// Iterator over the indices of non-empty rows in a CSR matrix.
pub struct CSR2DNonEmptyRowIndices<'a, CSR: SizedRowsSparseMatrix2D> {
    /// The CSR matrix.
    csr2d: &'a CSR,
    /// The iterator of the row indices and their sizes.
    row_sizes: (SimpleRange<CSR::RowIndex>, CSR::SparseRowSizes<'a>),
    /// The number of non-empty rows returned so far.
    returned_non_empty_rows: CSR::RowIndex,
}

impl<CSR: SizedRowsSparseMatrix2D> Iterator for CSR2DNonEmptyRowIndices<'_, CSR> {
    type Item = CSR::RowIndex;

    fn next(&mut self) -> Option<Self::Item> {
        while let (Some(row_index), Some(row_size)) =
            (self.row_sizes.0.next(), self.row_sizes.1.next())
        {
            if row_size > CSR::ColumnIndex::ZERO {
                self.returned_non_empty_rows += CSR::RowIndex::ONE;
                return Some(row_index);
            }
        }
        None
    }
}

impl<CSR: SizedRowsSparseMatrix2D> ExactSizeIterator for CSR2DNonEmptyRowIndices<'_, CSR> {
    fn len(&self) -> usize {
        (self.csr2d.number_of_rows() - self.returned_non_empty_rows).into_usize()
    }
}

impl<CSR: SizedRowsSparseMatrix2D> DoubleEndedIterator for CSR2DNonEmptyRowIndices<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let (Some(row_index), Some(row_size)) =
            (self.row_sizes.0.next_back(), self.row_sizes.1.next_back())
        {
            if row_size > CSR::ColumnIndex::ZERO {
                self.returned_non_empty_rows += CSR::RowIndex::ONE;
                return Some(row_index);
            }
        }
        None
    }
}

impl<'a, CSR: SizedRowsSparseMatrix2D> From<&'a CSR> for CSR2DNonEmptyRowIndices<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        CSR2DNonEmptyRowIndices {
            csr2d,
            row_sizes: (csr2d.row_indices(), csr2d.sparse_row_sizes()),
            returned_non_empty_rows: CSR::RowIndex::ZERO,
        }
    }
}
