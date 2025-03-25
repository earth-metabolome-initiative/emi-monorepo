//! Submodule defining traits characterizing a sparse matrix.

use super::{BiMatrix2D, Matrix2D, SparseMatrix, SymmetricMatrix2D};

/// Trait defining a sparse bidimensional matrix.
pub trait SparseMatrix2D: Matrix2D + SparseMatrix {
    /// Iterator over the sparse columns of a row.
    type SparseRow<'a>: ExactSizeIterator<Item = <Self as Matrix2D>::ColumnIndex>
        + DoubleEndedIterator<Item = <Self as Matrix2D>::ColumnIndex>
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
    /// Iterator over the row indices of the empty rows.
    type EmptyRowIndices<'a>: ExactSizeIterator<Item = Self::RowIndex>
        + DoubleEndedIterator<Item = Self::RowIndex>
    where
        Self: 'a;
    /// Iterator over the row indices of the non-empty rows.
    type NonEmptyRowIndices<'a>: ExactSizeIterator<Item = Self::RowIndex>
        + DoubleEndedIterator<Item = Self::RowIndex>
    where
        Self: 'a;

    /// Returns an iterator over the sparse columns of a row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row index.
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

    /// Returns the number of empty rows.
    fn number_of_empty_rows(&self) -> Self::RowIndex;

    /// Returns the number of non-empty rows.
    fn number_of_non_empty_rows(&self) -> Self::RowIndex;

    /// Returns an iterator over the row indices of the empty rows.
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_>;

    /// Returns an iterator over the row indices of the non-empty rows.
    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_>;
}

/// Trait defining a sparse matrix which supports efficient operations on
/// columns.
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
    fn number_of_defined_values_in_column(&self, column: Self::ColumnIndex) -> Self::RowIndex {
        self.transposed().number_of_defined_values_in_row(column)
    }

    /// Returns an iterator over the sparse sizes of all columns.
    ///
    /// # Arguments
    ///
    /// * `column` - The column index.
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

/// Trait defining a sparse symmetric matrix.
pub trait SparseSymmetricMatrix2D:
    SymmetricMatrix2D<SymmetricMatrix = <Self as SparseSymmetricMatrix2D>::SymmetricSparseMatrix>
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
