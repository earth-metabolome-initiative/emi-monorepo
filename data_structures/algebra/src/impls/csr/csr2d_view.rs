//! Iterator of the sparse coordinates of the CSR2D matrix.

use std::cmp::Ordering;

use num_traits::{ConstOne, ConstZero, SaturatingSub};
use numeric_common_traits::prelude::IntoUsize;

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
    next: Option<CSR::SparseRow<'a>>,
    /// The row associated with the index at the end of the iteration.
    back: Option<CSR::SparseRow<'a>>,
}

impl<CSR: SparseMatrix2D> Iterator for CSR2DView<'_, CSR> {
    type Item = CSR::Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.as_mut()?.next().map(|column_index| (self.next_row, column_index)).or_else(|| {
            match self.next_row.cmp(&self.back_row) {
                Ordering::Equal => None,
                Ordering::Less => {
                    self.next_row += CSR::RowIndex::ONE;
                    self.next = Some(self.csr2d.sparse_row(self.next_row));
                    self.next()
                }
                Ordering::Greater => {
                    self.back.as_mut()?.next().map(|column_index| (self.back_row, column_index))
                }
            }
        })
    }
}

impl<'matrix, CSR: SizedSparseMatrix2D> ExactSizeIterator for CSR2DView<'matrix, CSR>
where
    CSR::SparseRow<'matrix>: ExactSizeIterator,
{
    fn len(&self) -> usize {
        let next_row_rank = self.csr2d.rank_row(self.next_row).into_usize();
        let already_observed_in_next_row =
            self.csr2d.number_of_defined_values_in_row(self.next_row).into_usize()
                - self.next.as_ref().map_or(0, ExactSizeIterator::len);
        let back_row_rank = self.csr2d.rank_row(self.back_row).into_usize();
        let still_to_be_observed_in_back_row = self.back.as_ref().map_or(0, ExactSizeIterator::len);
        back_row_rank + still_to_be_observed_in_back_row
            - next_row_rank
            - already_observed_in_next_row
    }
}

impl<CSR: SparseMatrix2D> DoubleEndedIterator for CSR2DView<'_, CSR> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.as_mut()?.next().map(|column_index| (self.back_row, column_index)).or_else(|| {
            match self.next_row.cmp(&self.back_row) {
                Ordering::Equal => None,
                Ordering::Less => {
                    self.next.as_mut()?.next().map(|column_index| (self.next_row, column_index))
                }
                Ordering::Greater => {
                    self.back_row += CSR::RowIndex::ONE;
                    self.back = Some(self.csr2d.sparse_row(self.back_row));
                    self.next_back()
                }
            }
        })
    }
}

impl<'a, CSR: SparseMatrix2D> From<&'a CSR> for CSR2DView<'a, CSR> {
    fn from(csr2d: &'a CSR) -> Self {
        let next_row = CSR::RowIndex::ZERO;
        let back_row = csr2d.number_of_rows().saturating_sub(&CSR::RowIndex::ONE);
        let next = (next_row < csr2d.number_of_rows()).then(|| csr2d.sparse_row(next_row));
        let back = (back_row < csr2d.number_of_rows() && next_row < back_row)
            .then(|| csr2d.sparse_row(back_row));
        Self { csr2d, next_row, back_row, next, back }
    }
}
