//! Submodule providing an iterator over the indices of the rows with values
//! for the [`GenericMatrix2DWithPaddedDiagonal`] struct, which automatically
//! adds diagonal values when missing.

use multi_ranged::SimpleRange;
use numeric_common_traits::prelude::{IntoUsize, TryFromUsize};

use crate::traits::SparseMatrix2D;

/// Iterator over the indices of the rows with values for the
/// [`GenericMatrix2DWithPaddedDiagonal`] struct, which automatically adds
/// diagonal values when missing.
pub struct SparseRowsWithPaddedDiagonal<'matrix, M: SparseMatrix2D> {
    /// The underlying matrix.
    matrix: &'matrix M,
    /// The iterator over the row indices.
    row_indices: SimpleRange<M::RowIndex>,
    /// The row index and iterator from the beginning of the iterator.
    start_row: Option<(M::RowIndex, M::SparseRow<'matrix>)>,
    /// The row index and iterator from the end of the iterator.
    end_row: Option<(M::RowIndex, M::SparseRow<'matrix>)>,
}

impl<'matrix, M: SparseMatrix2D> From<&'matrix M> for SparseRowsWithPaddedDiagonal<'matrix, M> {
    fn from(matrix: &'matrix M) -> Self {
        Self { matrix, row_indices: matrix.row_indices(), start_row: None, end_row: None }
    }
}

impl<M: SparseMatrix2D> Iterator for SparseRowsWithPaddedDiagonal<'_, M>
where
    M::RowIndex: IntoUsize,
    M::ColumnIndex: TryFromUsize,
{
    type Item = M::RowIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start_row, start_row_iterator)) = self.start_row.as_mut() {
            if start_row_iterator.next().is_some() {
                Some(*start_row)
            } else {
                self.start_row = None;
                self.next()
            }
        } else if let Some(new_start_row) = self.row_indices.next() {
            self.start_row = Some((new_start_row, self.matrix.sparse_row(new_start_row)));
            self.next()
        } else if let Some((end_row, end_row_iterator)) = self.end_row.as_mut() {
            if end_row_iterator.next().is_some() {
                Some(*end_row)
            } else {
                self.end_row = None;
                self.next()
            }
        } else {
            None
        }
    }
}

impl<M: SparseMatrix2D> DoubleEndedIterator for SparseRowsWithPaddedDiagonal<'_, M>
where
    M::RowIndex: IntoUsize,
    M::ColumnIndex: TryFromUsize,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some((end_row, end_row_iterator)) = self.end_row.as_mut() {
            if end_row_iterator.next().is_some() {
                Some(*end_row)
            } else {
                self.end_row = None;
                self.next_back()
            }
        } else if let Some(new_end_row) = self.row_indices.next_back() {
            self.end_row = Some((new_end_row, self.matrix.sparse_row(new_end_row)));
            self.next_back()
        } else if let Some((start_row, start_row_iterator)) = self.start_row.as_mut() {
            if start_row_iterator.next().is_some() {
                Some(*start_row)
            } else {
                self.start_row = None;
                self.next_back()
            }
        } else {
            None
        }
    }
}
