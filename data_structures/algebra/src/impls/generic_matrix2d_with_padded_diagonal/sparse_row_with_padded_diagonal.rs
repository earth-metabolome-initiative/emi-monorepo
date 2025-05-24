//! Submodule providing a wrapper Iterator over the sparse row
//! which includes the diagonal if it is not already present
//! in the underlying matrix. The diagonal element is returned
//! in the sorted position where it is expected to be in the
//! sparse row.

use numeric_common_traits::prelude::{IntoUsize, TryFromUsize};

use crate::{impls::MutabilityError, traits::SparseMatrix2D};

/// A wrapper iterator over the sparse row which includes the diagonal
/// if it is not already present in the underlying matrix.
pub struct SparseRowWithPaddedDiagonal<'matrix, M: SparseMatrix2D + ?Sized + 'matrix> {
    /// The underlying sparse row iterator.
    sparse_row: Option<M::SparseRow<'matrix>>,
    /// The row index of the sparse row.
    row: M::ColumnIndex,
    /// A stored element that might have been saved when the diagonal element
    /// is returned in place of the element in the sparse row, iterating from
    /// the front.
    start_element_backup: Option<M::ColumnIndex>,
    /// A stored element that might have been saved when the diagonal element
    /// is returned in place of the element in the sparse row, iterating from
    /// the back.
    end_element_backup: Option<M::ColumnIndex>,
    /// Whether the diagonal element has been returned.
    diagonal_returned: bool,
}

impl<'matrix, M: SparseMatrix2D + ?Sized + 'matrix> Clone
    for SparseRowWithPaddedDiagonal<'matrix, M>
where
    M::RowIndex: IntoUsize,
    M::ColumnIndex: TryFromUsize,
{
    fn clone(&self) -> Self {
        Self {
            sparse_row: self.sparse_row.clone(),
            row: self.row,
            start_element_backup: self.start_element_backup,
            end_element_backup: self.end_element_backup,
            diagonal_returned: self.diagonal_returned,
        }
    }
}

impl<'matrix, M: SparseMatrix2D + ?Sized + 'matrix> SparseRowWithPaddedDiagonal<'matrix, M>
where
    M::RowIndex: IntoUsize,
    M::ColumnIndex: TryFromUsize,
{
    /// Creates a new `SparseRowWithPaddedDiagonal` with the given sparse row
    /// iterator.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The underlying matrix.
    /// * `row` - The row index of the sparse row.
    pub fn new(matrix: &'matrix M, row: M::RowIndex) -> Result<Self, MutabilityError<M>> {
        let sparse_row = (row < matrix.number_of_rows()).then(|| matrix.sparse_row(row));
        Ok(Self {
            sparse_row,
            row: M::ColumnIndex::try_from_usize(row.into_usize())
                .map_err(|_| MutabilityError::<M>::MaxedOutColumnIndex)?,
            start_element_backup: None,
            end_element_backup: None,
            diagonal_returned: false,
        })
    }
}

impl<'matrix, M: SparseMatrix2D + ?Sized + 'matrix> Iterator
    for SparseRowWithPaddedDiagonal<'matrix, M>
{
    type Item = M::ColumnIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.start_element_backup
            .take()
            .or_else(|| {
                self.sparse_row.as_mut().and_then(|sparse_row| {
                    sparse_row.next().map(|column_index| {
                        if column_index == self.row && !self.diagonal_returned {
                            self.diagonal_returned = true;
                            column_index
                        } else if column_index > self.row && !self.diagonal_returned {
                            self.diagonal_returned = true;
                            self.start_element_backup = Some(column_index);
                            self.row
                        } else {
                            column_index
                        }
                    })
                })
            })
            .or(self.end_element_backup.take())
            .or_else(|| {
                // If the diagonal element has not been returned yet, and
                // we are at the end of the sparse row, we return the diagonal
                // element. Otherwise, we return None.
                (!self.diagonal_returned).then(|| {
                    self.diagonal_returned = true;
                    self.row
                })
            })
    }
}

impl<'matrix, M: SparseMatrix2D + ?Sized + 'matrix> DoubleEndedIterator
    for SparseRowWithPaddedDiagonal<'matrix, M>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.end_element_backup
            .take()
            .or_else(|| {
                self.sparse_row.as_mut().and_then(|sparse_row| {
                    sparse_row.next_back().map(|column_index| {
                        if column_index == self.row && !self.diagonal_returned {
                            self.diagonal_returned = true;
                            column_index
                        } else if column_index < self.row && !self.diagonal_returned {
                            self.diagonal_returned = true;
                            self.end_element_backup = Some(column_index);
                            self.row
                        } else {
                            column_index
                        }
                    })
                })
            })
            .or_else(|| {
                // If the diagonal element has not been returned yet, and
                // we are at the end of the sparse row, we return the diagonal
                // element. Otherwise, we return None.
                (!self.diagonal_returned).then(|| {
                    self.diagonal_returned = true;
                    self.row
                })
            })
            .or_else(|| self.start_element_backup.take())
    }
}
