//! Submodule defining traits characterizing a sparse matrix.

use super::{
    BiMatrix2D, Matrix2D, RankSelectSparseMatrix, SizedSparseMatrix, SparseMatrix,
    SymmetricMatrix2D,
};

/// Trait defining a sparse bi-dimensional matrix.
pub trait SparseMatrix2D: Matrix2D + SparseMatrix {
    /// Iterator over the sparse columns of a row.
    type SparseRow<'a>: Iterator<Item = <Self as Matrix2D>::ColumnIndex>
        + DoubleEndedIterator<Item = <Self as Matrix2D>::ColumnIndex>
        + Clone
    where
        Self: 'a;
    /// Iterator over all the column coordinates of the matrix.
    type SparseColumns<'a>: Iterator<Item = Self::ColumnIndex>
        + DoubleEndedIterator<Item = Self::ColumnIndex>
    where
        Self: 'a;
    /// Iterator over all the row coordinates of the matrix.
    type SparseRows<'a>: Iterator<Item = Self::RowIndex>
        + DoubleEndedIterator<Item = Self::RowIndex>
    where
        Self: 'a;

    /// Returns an iterator over the sorted sparse columns of a row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row index.
    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_>;

    /// Returns whether the provided row has an entry for the provided column.
    ///
    /// # Arguments
    ///
    /// * `row`: The row index.
    /// * `column`: The column index.
    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool;

    /// Returns an iterator over all the columns of the matrix.
    fn sparse_columns(&self) -> Self::SparseColumns<'_>;

    /// Returns an iterator over all the rows of the matrix.
    fn sparse_rows(&self) -> Self::SparseRows<'_>;
}

impl<M: SparseMatrix2D> SparseMatrix2D for &M {
    type SparseRow<'a>
        = M::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = M::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = M::SparseRows<'a>
    where
        Self: 'a;

    #[inline]
    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        (*self).sparse_row(row)
    }

    #[inline]
    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        (*self).has_entry(row, column)
    }

    #[inline]
    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        (*self).sparse_columns()
    }

    #[inline]
    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        (*self).sparse_rows()
    }
}

/// Trait characterizing the properties of a matrix with empty rows.
pub trait EmptyRows: SparseMatrix2D {
    /// Iterator over the row indices of the empty rows.
    type EmptyRowIndices<'a>: Iterator<Item = Self::RowIndex>
        + DoubleEndedIterator<Item = Self::RowIndex>
    where
        Self: 'a;
    /// Iterator over the row indices of the non-empty rows.
    type NonEmptyRowIndices<'a>: Iterator<Item = Self::RowIndex>
        + DoubleEndedIterator<Item = Self::RowIndex>
    where
        Self: 'a;

    /// Returns the number of empty rows.
    fn number_of_empty_rows(&self) -> Self::RowIndex;

    /// Returns the number of non-empty rows.
    fn number_of_non_empty_rows(&self) -> Self::RowIndex;

    /// Returns an iterator over the row indices of the empty rows.
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_>;

    /// Returns an iterator over the row indices of the non-empty rows.
    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_>;
}

impl<M: EmptyRows> EmptyRows for &M {
    type EmptyRowIndices<'a>
        = M::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = M::NonEmptyRowIndices<'a>
    where
        Self: 'a;

    #[inline]
    fn number_of_empty_rows(&self) -> Self::RowIndex {
        (*self).number_of_empty_rows()
    }

    #[inline]
    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        (*self).number_of_non_empty_rows()
    }

    #[inline]
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        (*self).empty_row_indices()
    }

    #[inline]
    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        (*self).non_empty_row_indices()
    }
}

/// Trait defining a sized sparse matrix which efficiently provides the sizes of
/// all rows.
pub trait SizedRowsSparseMatrix2D: SparseMatrix2D + SizedSparseMatrix {
    /// Iterator over the sizes of all of the rows.
    type SparseRowSizes<'a>: Iterator<Item = Self::ColumnIndex>
        + DoubleEndedIterator<Item = Self::ColumnIndex>
    where
        Self: 'a;

    /// Returns an iterator over the sizes of all of the rows.
    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_>;

    /// Returns the number of defined values in a row.
    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex;
}

/// Trait defining a sized sparse matrix which efficiently provides the sizes of
/// all rows.
pub trait SizedSparseMatrix2D: SizedRowsSparseMatrix2D + RankSelectSparseMatrix {
    /// Returns the rank of a row.
    fn rank_row(&self, row: Self::RowIndex) -> Self::SparseIndex;

    /// Returns the row associated with a given sparse index.
    ///
    /// # Arguments
    ///
    /// * `sparse_index`: The sparse index of the row to get.
    fn select_row(&self, sparse_index: Self::SparseIndex) -> Self::RowIndex;

    /// Returns the column associated with a given sparse index.
    ///
    /// # Arguments
    ///
    /// * `sparse_index`: The sparse index of the column to get.
    fn select_column(&self, sparse_index: Self::SparseIndex) -> Self::ColumnIndex;
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
    type SparseTransposedMatrix: SparseMatrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>;

    #[inline]
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

/// Trait defining a sized sparse bimatrix which efficiently provides the sizes
/// of all rows.
pub trait SizedSparseBiMatrix2D:
    SparseBiMatrix2D<
        SparseMatrix = <Self as SizedSparseBiMatrix2D>::SizedSparseMatrix,
        SparseTransposedMatrix = <Self as SizedSparseBiMatrix2D>::SizedSparseTransposedMatrix,
    > + SizedSparseMatrix2D
{
    /// The type of the underlying sparse matrix.
    type SizedSparseMatrix: SizedSparseMatrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>;
    /// The type of the underlying transposed sparse matrix.
    type SizedSparseTransposedMatrix: SizedSparseMatrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>;

    #[inline]
    /// Returns the number of defined values in a column.
    ///
    /// # Arguments
    ///
    /// * `column` - The column index.
    fn number_of_defined_values_in_column(&self, column: Self::ColumnIndex) -> Self::RowIndex {
        self.transposed().number_of_defined_values_in_row(column)
    }

    #[inline]
    /// Returns an iterator over the sparse sizes of all columns.
    ///
    /// # Arguments
    ///
    /// * `column` - The column index.
    fn sparse_column_sizes(
        &self,
    ) -> <Self::SizedSparseTransposedMatrix as SizedRowsSparseMatrix2D>::SparseRowSizes<'_> {
        self.transposed().sparse_row_sizes()
    }
}

impl<M> SizedSparseBiMatrix2D for M
where
    M: SparseBiMatrix2D + SizedSparseMatrix2D,
    M::Matrix: SizedSparseMatrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>,
    M::TransposedMatrix:
        SizedSparseMatrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>,
{
    type SizedSparseMatrix = M::Matrix;
    type SizedSparseTransposedMatrix = M::TransposedMatrix;
}

/// Trait defining a sparse symmetric matrix.
pub trait SparseSymmetricMatrix2D:
    SymmetricMatrix2D<SymmetricMatrix = <Self as SparseSymmetricMatrix2D>::SymmetricSparseMatrix>
    + SparseMatrix2D
{
    /// The underlying symmetric sparse matrix type.
    type SymmetricSparseMatrix: SparseMatrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>;
}

impl<M> SparseSymmetricMatrix2D for M
where
    M: SymmetricMatrix2D + SparseMatrix2D,
    M::Matrix: SparseMatrix2D<RowIndex = Self::RowIndex, ColumnIndex = Self::ColumnIndex>,
{
    type SymmetricSparseMatrix = M::Matrix;
}
