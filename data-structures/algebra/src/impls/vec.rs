//! Submodule implementing a vec-based matrix.

use crate::traits::Matrix2D;

/// Implementation of a matrix using a vector.
pub struct VecMatrix2D<V> {
    /// The data of the matrix.
    data: Vec<V>,
    /// The number of rows.
    number_of_rows: usize,
}

impl<V> Matrix2D for VecMatrix2D<V> {
    type RowIndex = usize;
    type ColumnIndex = usize;

    fn number_of_rows(&self) -> usize {
        self.number_of_rows
    }

    fn number_of_columns(&self) -> usize {
        self.data.len() / self.number_of_rows
    }
}
