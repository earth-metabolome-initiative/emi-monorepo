//! Submodule implementing a vec-based matrix.

use crate::traits::{Matrix, Matrix2D};

/// Implementation of a matrix using a vector.
pub struct VecMatrix2D<V> {
    /// The data of the matrix.
    data: Vec<V>,
    /// The number of rows.
    number_of_rows: usize,
}

impl<V> Matrix for VecMatrix2D<V> {
    type Coordinates = (usize, usize);

    #[inline]
    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows(), self.number_of_columns()]
    }
}

impl<V> Matrix2D for VecMatrix2D<V> {
    type RowIndex = usize;
    type ColumnIndex = usize;

    #[inline]
    fn number_of_rows(&self) -> usize {
        self.number_of_rows
    }

    #[inline]
    fn number_of_columns(&self) -> usize {
        self.data.len() / self.number_of_rows
    }
}
