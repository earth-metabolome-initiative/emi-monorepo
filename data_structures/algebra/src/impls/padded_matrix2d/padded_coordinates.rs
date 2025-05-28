//! Submodule providing an iterator over all of the (row, column) coordinates
//! of a dense rectangular matrix.

use multi_ranged::SimpleRange;
use num_traits::ConstZero;
use numeric_common_traits::prelude::IntoUsize;

use crate::traits::Matrix2D;

/// Iterator over all of the (row, column) coordinates of a dense rectangular
pub struct PaddedCoordinates<M: Matrix2D> {
    /// The underlying matrix.
    matrix: M,
    /// The row iterator.
    row_iter: SimpleRange<M::RowIndex>,
    /// The column iterator.
    column_iter: SimpleRange<M::ColumnIndex>,
    /// The current row index.
    current_row: M::RowIndex,
}

impl<M: Matrix2D> From<M> for PaddedCoordinates<M> {
    fn from(matrix: M) -> Self {
        Self {
            row_iter: matrix.row_indices(),
            column_iter: matrix.column_indices(),
            current_row: M::RowIndex::ZERO,
            matrix,
        }
    }
}

impl<M: Matrix2D> Iterator for PaddedCoordinates<M> {
    type Item = (M::RowIndex, M::ColumnIndex);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(column_index) = self.column_iter.next() {
            Some((self.current_row, column_index))
        } else if let Some(row_index) = self.row_iter.next() {
            self.current_row = row_index;
            self.column_iter = self.matrix.column_indices();
            self.column_iter.next().map(|column_index| (self.current_row, column_index))
        } else {
            None
        }
    }
}

impl<M: Matrix2D> DoubleEndedIterator for PaddedCoordinates<M> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(column_index) = self.column_iter.next_back() {
            Some((self.current_row, column_index))
        } else if let Some(row_index) = self.row_iter.next_back() {
            self.current_row = row_index;
            self.column_iter = self.matrix.column_indices();
            self.column_iter.next_back().map(|column_index| (self.current_row, column_index))
        } else {
            None
        }
    }
}

impl<M: Matrix2D> ExactSizeIterator for PaddedCoordinates<M> {
    fn len(&self) -> usize {
        let number_of_columns = self.matrix.number_of_columns().into_usize();
        number_of_columns * self.row_iter.len() + self.column_iter.len()
    }
}
