//! Submodule defining traits to characterize a dense bidimensional matrix.

use super::{DenseMatrix, Matrix2D, ValuedMatrix2D};

/// Trait defining a dense bidimensional matrix.
pub trait DenseMatrix2D: DenseMatrix + ValuedMatrix2D + Matrix2D {
    /// Iterator over the values in a row of the matrix.
    type Row<'a>: ExactSizeIterator<Item = Self::Value> + DoubleEndedIterator<Item = Self::Value>
    where
        Self: 'a;

    /// Returns an iterator over the values in a row of the matrix.
    ///
    /// # Arguments
    ///
    /// * `row`: The row index.
    ///
    /// # Panics
    ///
    /// * Panics if the row index is out of bounds.
    fn row(&self, row: Self::RowIndex) -> Self::Row<'_>;
}
