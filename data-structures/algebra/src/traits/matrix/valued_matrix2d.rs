//! Submodule defining the [`ValuedMatrix2D`] trait.

use super::{ImplicitValuedMatrix, Matrix2D};

/// Trait defining a bidimensional matrix.
pub trait ImplicitValuedMatrix2D: Matrix2D + ImplicitValuedMatrix {
    /// Iterator over all of the indices of the rows of the matrix.
    type RowIndices<'a>: Iterator<Item = Self::RowIndex>
    where
        Self: 'a;
    /// Iterator over all of the indices of the columns of the matrix.
    type ColumnIndices<'a>: Iterator<Item = Self::ColumnIndex>
    where
        Self: 'a;

    /// Returns an iterator over all of the indices of the rows of the matrix.
    fn row_indices(&self) -> Self::RowIndices<'_>;

    /// Returns an iterator over all of the indices of the columns of the
    /// matrix.
    fn column_indices(&self) -> Self::ColumnIndices<'_>;

    /// Returns an iterator over the values of a row.
    fn row_values(&self, row: Self::RowIndex) -> impl Iterator<Item = Self::Value> + '_ {
        self.column_indices().map(move |column| self.value(&(row, column)))
    }
	
    /// Returns an iterator over the values of a column.
    fn column_values(&self, column: Self::ColumnIndex) -> impl Iterator<Item = Self::Value> + '_ {
        self.row_indices().map(move |row| self.value(&(row, column)))
    }
}
