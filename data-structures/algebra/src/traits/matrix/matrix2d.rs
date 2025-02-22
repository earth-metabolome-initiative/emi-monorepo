//! Trait defining a bidimensional matrix.

use crate::traits::{IntoUsize, Matrix, PositiveInteger, SparseMatrix};

/// Trait defining a bidimensional matrix.
pub trait Matrix2D:
    Matrix<Coordinates = (<Self as Matrix2D>::RowIndex, <Self as Matrix2D>::ColumnIndex)>
{
    /// Type of the row index.
    type RowIndex: PositiveInteger + IntoUsize;
    /// Type of the column index.
    type ColumnIndex: PositiveInteger + IntoUsize;

    /// Returns the number of rows of the matrix.
    fn number_of_rows(&self) -> Self::RowIndex;

    /// Returns the number of columns of the matrix.
    fn number_of_columns(&self) -> Self::ColumnIndex;
}

impl<M: Matrix2D> Matrix for M {
    type Coordinates = (<Self as Matrix2D>::RowIndex, <Self as Matrix2D>::ColumnIndex);

    fn number_of_elements(&self) -> usize {
        self.number_of_rows().into_usize() * self.number_of_columns().into_usize()
    }
}

/// Trait defining a sparse bidimensional matrix.
pub trait SparseMatrix2D: Matrix2D + SparseMatrix {
    /// Iterator over the sparse column coordinates of a row.
    type SparseRowColumns<'a>: ExactSizeIterator<Item = Self::ColumnIndex>
        + DoubleEndedIterator<Item = Self::ColumnIndex>
    where
        Self: 'a;
    /// Iterator over all the column coordinates of the matrix.
    type SparseColumns<'a>: ExactSizeIterator<Item = Self::ColumnIndex>
        + DoubleEndedIterator<Item = Self::ColumnIndex>
    where
        Self: 'a;
    /// Iterator over all the row coordinates of the matrix.
    type SparseRows<'a>: ExactSizeIterator<Item = Self::RowIndex>
        + DoubleEndedIterator<Item = Self::RowIndex>
    where
        Self: 'a;

    /// Returns an iterator over the sparse columns of a row.
    /// 
    /// # Arguments
    /// 
    /// * `row`: The row index.
    /// 
    fn row_sparse_columns(&self, row: Self::RowIndex) -> Self::SparseRowColumns<'_>;

    /// Returns an iterator over all the columns of the matrix.
    fn sparse_columns(&self) -> Self::SparseColumns<'_>;

    /// Returns an iterator over all the rows of the matrix.
    fn sparse_rows(&self) -> Self::SparseRows<'_>;

    /// Returns the number of defined values in a row.
    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> usize;

    /// Returns the rank of a row.
    fn rank(&self, row: Self::RowIndex) -> usize;
}

/// Trait defining a transposable bidimensional matrix.
pub trait TransposableMatrix2D: Matrix2D {
    /// The type of the transposed matrix.
    type Transposed: Matrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>;

    /// Returns the transpose of the matrix.
    fn transpose(&self) -> Self::Transposed;
}
