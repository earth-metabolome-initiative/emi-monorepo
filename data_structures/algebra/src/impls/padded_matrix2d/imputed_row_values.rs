//! Submodule providing a wrapper over the `SparseRowValues` from the
//! `SparseValuedMatrix2D` trait, adding the imputation of all the missing
//! values by using the `Map` generic made available in the `PaddedMatrix2D`
//! struct.

use multi_ranged::SimpleRange;
use numeric_common_traits::prelude::{IntoUsize, TryFromUsize};

use super::PaddedMatrix2D;
use crate::traits::{Matrix2D, SparseValuedMatrix2D, ValuedMatrix};

/// A wrapper over the `SparseRowValues` from the `SparseValuedMatrix2D` trait,
/// adding the imputation of all the missing values by using the `Map` generic
/// made available in the `PaddedMatrix2D` struct.
pub struct ImputedRowValues<'a, M: SparseValuedMatrix2D, Map> {
    /// The underlying sparse matrix.
    matrix: &'a PaddedMatrix2D<M, Map>,
    /// The row index.
    row_index: M::RowIndex,
    /// The iterator over all column indices.
    column_indices: SimpleRange<M::ColumnIndex>,
    /// The iterator over the underlying sparse matrix values.
    sparse_values: Option<M::SparseRowValues<'a>>,
    /// The iterator over the underlying sparse matrix column indices.
    sparse_column_indices: Option<M::SparseRow<'a>>,
    /// The backup of the column index from the start of the iterator.
    next_column_index: Option<M::ColumnIndex>,
    /// The backup of the column index from the end of the iterator.
    next_back_column_index: Option<M::ColumnIndex>,
}

impl<M, Map> Clone for ImputedRowValues<'_, M, Map>
where
    M: SparseValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    fn clone(&self) -> Self {
        Self {
            matrix: self.matrix,
            row_index: self.row_index,
            column_indices: self.column_indices.clone(),
            sparse_values: self.sparse_values.clone(),
            sparse_column_indices: self.sparse_column_indices.clone(),
            next_column_index: self.next_column_index,
            next_back_column_index: self.next_back_column_index,
        }
    }
}

impl<'a, M, Map> ImputedRowValues<'a, M, Map>
where
    M: SparseValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    /// Creates a new `ImputedRowValues` instance.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The underlying sparse matrix.
    /// * `row_index` - The row index.
    pub fn new(matrix: &'a PaddedMatrix2D<M, Map>, row_index: M::RowIndex) -> Self {
        Self {
            matrix,
            row_index,
            column_indices: matrix.column_indices(),
            sparse_values: (row_index < matrix.matrix.number_of_rows())
                .then(|| matrix.matrix.sparse_row_values(row_index)),
            sparse_column_indices: (row_index < matrix.matrix.number_of_rows())
                .then(|| matrix.matrix.sparse_row(row_index)),
            next_column_index: None,
            next_back_column_index: None,
        }
    }
}

impl<M, Map> Iterator for ImputedRowValues<'_, M, Map>
where
    M: SparseValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
    Map: Fn((M::RowIndex, M::ColumnIndex)) -> M::Value,
{
    type Item = <M as ValuedMatrix>::Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let dense_column_index = self.column_indices.next()?;
        if let Some(backup_sparse_column_index) = self.next_column_index {
            debug_assert!(
                backup_sparse_column_index >= dense_column_index,
                "The column indices are not sorted in ascending order: {dense_column_index} < {backup_sparse_column_index}"
            );
            if dense_column_index == backup_sparse_column_index {
                self.next_column_index = None;
                return self.sparse_values.as_mut().and_then(Iterator::next);
            }
        } else if let Some(sparse_column_index) =
            self.sparse_column_indices.as_mut().and_then(Iterator::next)
        {
            debug_assert!(
                sparse_column_index >= dense_column_index,
                "The column indices are not sorted in ascending order."
            );
            if dense_column_index == sparse_column_index {
                return self.sparse_values.as_mut().and_then(Iterator::next);
            }
            self.next_column_index = Some(sparse_column_index);
        }
        Some((self.matrix.map)((self.row_index, dense_column_index)))
    }
}

impl<M, Map> DoubleEndedIterator for ImputedRowValues<'_, M, Map>
where
    M: SparseValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
    Map: Fn((M::RowIndex, M::ColumnIndex)) -> M::Value,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let dense_column_index = self.column_indices.next_back()?;
        if let Some(backup_sparse_column_index) = self.next_back_column_index {
            if dense_column_index == backup_sparse_column_index {
                self.next_back_column_index = None;
                return self.sparse_values.as_mut().and_then(DoubleEndedIterator::next_back);
            }
        } else if let Some(sparse_column_index) =
            self.sparse_column_indices.as_mut().and_then(DoubleEndedIterator::next_back)
        {
            if dense_column_index == sparse_column_index {
                return self.sparse_values.as_mut().and_then(DoubleEndedIterator::next_back);
            }
            self.next_back_column_index = Some(sparse_column_index);
        }
        Some((self.matrix.map)((self.row_index, dense_column_index)))
    }
}

impl<M, Map> ExactSizeIterator for ImputedRowValues<'_, M, Map>
where
    M: SparseValuedMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
    Map: Fn((M::RowIndex, M::ColumnIndex)) -> M::Value,
{
    fn len(&self) -> usize {
        self.column_indices.len()
    }
}
