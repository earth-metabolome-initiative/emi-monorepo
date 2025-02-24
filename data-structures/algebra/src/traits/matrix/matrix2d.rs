//! Trait defining a bidimensional matrix.

use crate::traits::{IntoUsize, Matrix, PositiveInteger, SparseMatrix};

use super::SquareMatrix;

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
}

/// Trait defining a sparse bidimensional matrix.
pub trait SparseMatrix2D: Matrix2D + SparseMatrix {
    /// Iterator over the sparse column coordinates of a row.
    type SparseRow<'a>: ExactSizeIterator<Item = Self::ColumnIndex>
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
    /// Iterator over the sizes of all of the rows.
    type SparseRowSizes<'a>: ExactSizeIterator<Item = Self::ColumnIndex>
        + DoubleEndedIterator<Item = Self::ColumnIndex>
    where
        Self: 'a;

    /// Returns an iterator over the sparse columns of a row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row index.
    ///
    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_>;

    /// Returns an iterator over all the columns of the matrix.
    fn sparse_columns(&self) -> Self::SparseColumns<'_>;

    /// Returns an iterator over all the rows of the matrix.
    fn sparse_rows(&self) -> Self::SparseRows<'_>;

    /// Returns an iterator over the sizes of all of the rows.
    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_>;

    /// Returns the number of defined values in a row.
    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex;

    /// Returns the rank of a row.
    fn rank(&self, row: Self::RowIndex) -> Self::SparseIndex;
}

/// Trait defining a transposable bidimensional matrix.
pub trait TransposableMatrix2D: Matrix2D {
    /// The type of the transposed matrix.
    type Transposed: Matrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>;

    /// Returns the transpose of the matrix.
    fn transpose(&self) -> Self::Transposed;
}

/// Trait defining a matrix which supports efficient operations on columns.
pub trait BiMatrix2D: Matrix2D {
    /// The type of the underlying matrix.
    type Matrix: Matrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>;
    /// The type of the underlying transposed matrix.
    type TransposedMatrix: Matrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>;

    /// Returns a reference to the underlying matrix.
    fn matrix(&self) -> &Self::Matrix;

    /// Returns a reference to the underlying transposed matrix.
    fn transposed(&self) -> &Self::TransposedMatrix;
}

/// Trait defining a sparse matrix which supports efficient operations on columns.
pub trait SparseBiMatrix2D:
    BiMatrix2D<
        Matrix = <Self as SparseBiMatrix2D>::SparseMatrix,
        TransposedMatrix = <Self as SparseBiMatrix2D>::SparseTransposedMatrix,
    > + SparseMatrix2D
{
    /// The type of the underlying sparse matrix.
    type SparseMatrix: SparseMatrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>;
    /// The type of the underlying transposed sparse matrix.
    type SparseTransposedMatrix: SparseMatrix2D<
        RowIndex = Self::ColumnIndex,
        ColumnIndex = Self::RowIndex,
    >;

    /// Returns an iterator over the sparse rows of a column.
    ///
    /// # Arguments
    ///
    /// * `column`: The column index.
    ///
    fn sparse_column(
        &self,
        column: Self::ColumnIndex,
    ) -> <Self::SparseTransposedMatrix as SparseMatrix2D>::SparseRow<'_> {
        self.transposed().sparse_row(column)
    }

    /// Returns the number of defined values in a column.
    ///
    /// # Arguments
    ///
    /// * `column` - The column index.
    ///
    fn number_of_defined_values_in_column(&self, column: Self::ColumnIndex) -> Self::RowIndex {
        self.transposed().number_of_defined_values_in_row(column)
    }

    /// Returns an iterator over the sparse sizes of all columns.
    ///
    /// # Arguments
    ///
    /// * `column` - The column index.
    ///
    fn sparse_column_sizes(
        &self,
    ) -> <Self::SparseTransposedMatrix as SparseMatrix2D>::SparseRowSizes<'_> {
        self.transposed().sparse_row_sizes()
    }
}

impl<M> SparseBiMatrix2D for M
where
    M: BiMatrix2D + SparseMatrix2D,
    M::Matrix: SparseMatrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>,
    M::TransposedMatrix: SparseMatrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>,
{
    type SparseMatrix = M::Matrix;
    type SparseTransposedMatrix = M::TransposedMatrix;
}

/// Trait defining a symmetric matrix.
pub trait SymmetricMatrix2D:
    SquareMatrix + BiMatrix2D<Matrix = <Self as BiMatrix2D>::TransposedMatrix>
{
}

impl<M> SymmetricMatrix2D for M
where
    M: SquareMatrix + BiMatrix2D<TransposedMatrix = <M as BiMatrix2D>::Matrix>,
    M::Matrix: Matrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>,
{
}

/// Trait defining a sparse symmetric matrix.
pub trait SparseSymmetricMatrix2D:
    SymmetricMatrix2D
    + SparseMatrix2D
    + SparseBiMatrix2D<
        SparseMatrix = <Self as SparseSymmetricMatrix2D>::SymmetricSparseMatrix,
        SparseTransposedMatrix = <Self as SparseSymmetricMatrix2D>::SymmetricSparseMatrix,
    >
{
    /// The underlying symmetric sparse matrix type.
    type SymmetricSparseMatrix: SparseMatrix2D<
        RowIndex = Self::RowIndex,
        ColumnIndex = Self::ColumnIndex,
    >;
}

impl<M> SparseSymmetricMatrix2D for M
where
    M: SymmetricMatrix2D + SparseMatrix2D,
    M::Matrix: SparseMatrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>,
{
    type SymmetricSparseMatrix = M::Matrix;
}
