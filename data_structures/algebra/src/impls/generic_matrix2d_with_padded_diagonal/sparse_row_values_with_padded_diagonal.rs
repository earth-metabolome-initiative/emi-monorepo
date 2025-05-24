//! Submodule providing a wrapper Iterator over the sparse row values
//! which includes the diagonal value if it is not already present
//! in the underlying matrix. The diagonal element is returned
//! in the sorted position where it is expected to be in the
//! sparse row.

use numeric_common_traits::prelude::{IntoUsize, TryFromUsize};

use crate::{impls::MutabilityError, traits::SparseValuedMatrix2D};

/// A wrapper iterator over the sparse row values which includes the diagonal
/// if it is not already present in the underlying matrix. The diagonal
/// element is returned in the sorted position where it is expected to be in
/// the sparse row.
pub struct SparseRowValuesWithPaddedDiagonal<'matrix, M: SparseValuedMatrix2D + 'matrix, Map> {
    /// The underlying sparse row iterator.
    sparse_row: Option<(M::SparseRow<'matrix>, M::SparseRowValues<'matrix>)>,
    /// The row index of the sparse row.
    row: M::RowIndex,
    /// The row index of the sparse row, as a column index.
    row_as_column: M::ColumnIndex,
    /// A stored element that might have been saved when the diagonal element
    /// is returned in place of the element in the sparse row, iterating from
    /// the front.
    start_element_backup: Option<M::Value>,
    /// A stored element that might have been saved when the diagonal element
    /// is returned in place of the element in the sparse row, iterating from
    /// the back.
    end_element_backup: Option<M::Value>,
    /// Whether the diagonal element has been returned.
    diagonal_returned: bool,
    /// The map function defining the values of the new diagonal elements.
    map: Map,
}

impl<'matrix, M: SparseValuedMatrix2D + 'matrix, Map> Clone
    for SparseRowValuesWithPaddedDiagonal<'matrix, M, Map>
where
    M::RowIndex: IntoUsize,
    M::ColumnIndex: TryFromUsize,
    M::Value: Clone,
    Map: Clone,
{
    fn clone(&self) -> Self {
        Self {
            sparse_row: self.sparse_row.clone(),
            row: self.row,
            row_as_column: self.row_as_column,
            start_element_backup: self.start_element_backup.clone(),
            end_element_backup: self.end_element_backup.clone(),
            diagonal_returned: self.diagonal_returned,
            map: self.map.clone(),
        }
    }
}

impl<'matrix, M: SparseValuedMatrix2D + 'matrix, Map>
    SparseRowValuesWithPaddedDiagonal<'matrix, M, Map>
where
    M::RowIndex: IntoUsize,
    M::ColumnIndex: TryFromUsize,
{
    /// Creates a new `SparseRowValuesWithPaddedDiagonal` with the given sparse
    /// row iterator.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The underlying matrix.
    /// * `row` - The row index of the sparse row.
    pub fn new(matrix: &'matrix M, row: M::RowIndex, map: Map) -> Result<Self, MutabilityError<M>> {
        Ok(Self {
            sparse_row: (row < matrix.number_of_rows())
                .then(|| (matrix.sparse_row(row), matrix.sparse_row_values(row))),
            row,
            row_as_column: M::ColumnIndex::try_from_usize(row.into_usize())
                .map_err(|_| MutabilityError::<M>::MaxedOutColumnIndex)?,
            start_element_backup: None,
            end_element_backup: None,
            diagonal_returned: false,
            map,
        })
    }
}

impl<'matrix, M: SparseValuedMatrix2D + 'matrix, Map> Iterator
    for SparseRowValuesWithPaddedDiagonal<'matrix, M, Map>
where
    Map: Fn(M::RowIndex) -> M::Value,
{
    type Item = M::Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.start_element_backup
            .take()
            .or_else(|| {
                self.sparse_row.as_mut().and_then(|sparse_row| {
                    sparse_row.0.next().zip(sparse_row.1.next()).map(
                        |(column_index, column_value)| {
                            if column_index == self.row_as_column && !self.diagonal_returned {
                                self.diagonal_returned = true;
                                column_value
                            } else if column_index > self.row_as_column && !self.diagonal_returned {
                                self.diagonal_returned = true;
                                self.start_element_backup = Some(column_value);
                                (self.map)(self.row)
                            } else {
                                column_value
                            }
                        },
                    )
                })
            })
            .or(self.end_element_backup.take())
            .or_else(|| {
                // If the diagonal element has not been returned yet, and
                // we are at the end of the sparse row, we return the diagonal
                // element. Otherwise, we return None.
                (!self.diagonal_returned).then(|| {
                    self.diagonal_returned = true;
                    (self.map)(self.row)
                })
            })
    }
}

impl<'matrix, M: SparseValuedMatrix2D + 'matrix, Map> DoubleEndedIterator
    for SparseRowValuesWithPaddedDiagonal<'matrix, M, Map>
where
    Map: Fn(M::RowIndex) -> M::Value,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.end_element_backup
            .take()
            .or_else(|| {
                self.sparse_row.as_mut().and_then(|sparse_row| {
                    sparse_row.0.next_back().zip(sparse_row.1.next_back()).map(
                        |(column_index, column_value)| {
                            if column_index == self.row_as_column && !self.diagonal_returned {
                                self.diagonal_returned = true;
                                column_value
                            } else if column_index > self.row_as_column && !self.diagonal_returned {
                                self.diagonal_returned = true;
                                self.end_element_backup = Some(column_value);
                                (self.map)(self.row)
                            } else {
                                column_value
                            }
                        },
                    )
                })
            })
            .or_else(|| {
                // If the diagonal element has not been returned yet, and
                // we are at the end of the sparse row, we return the diagonal
                // element. Otherwise, we return None.
                (!self.diagonal_returned).then(|| {
                    self.diagonal_returned = true;
                    (self.map)(self.row)
                })
            })
    }
}
