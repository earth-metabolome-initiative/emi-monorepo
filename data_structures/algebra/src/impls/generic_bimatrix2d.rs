//! [`GenericBiMatrix2D`] data structure, which provides a wrapper to a matrix
//! and its transposed version.

use numeric_common_traits::prelude::IntoUsize;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// [`GenericBiMatrix2D`] data structure, which provides a wrapper to a matrix
/// and its transposed version.
pub struct GenericBiMatrix2D<M, T> {
    /// The matrix.
    matrix: M,
    /// The transposed matrix.
    transposed: T,
}

impl<T: Matrix2D, M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>>
    GenericBiMatrix2D<M, T>
{
    /// Creates a new instance of [`GenericBiMatrix2D`].
    ///
    /// # Arguments
    ///
    /// * `matrix` - The matrix.
    pub fn new(matrix: M) -> Self {
        let transposed = matrix.transpose();
        Self { matrix, transposed }
    }
}

impl<M, T> Matrix for GenericBiMatrix2D<M, T>
where
    T: Matrix2D,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
{
    type Coordinates = (M::RowIndex, M::ColumnIndex);

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<M, T> Matrix2D for GenericBiMatrix2D<M, T>
where
    T: Matrix2D,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
{
    type ColumnIndex = M::ColumnIndex;
    type RowIndex = M::RowIndex;

    #[inline]
    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_columns()
    }

    #[inline]
    fn number_of_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_rows()
    }
}

impl<M, T> SparseMatrix for GenericBiMatrix2D<M, T>
where
    T: Matrix2D,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
    M: SparseMatrix,
{
    type SparseIndex = M::SparseIndex;
    type SparseCoordinates<'a>
        = M::SparseCoordinates<'a>
    where
        Self: 'a;

    #[inline]
    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.matrix.sparse_coordinates()
    }

    #[inline]
    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        self.matrix.last_sparse_coordinates()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }
}

impl<M, T> SizedSparseMatrix for GenericBiMatrix2D<M, T>
where
    T: Matrix2D,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
    M: SizedSparseMatrix,
{
    #[inline]
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.matrix.number_of_defined_values()
    }
}

impl<M, T> RankSelectSparseMatrix for GenericBiMatrix2D<M, T>
where
    T: Matrix2D,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
    M: RankSelectSparseMatrix,
{
    #[inline]
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        self.matrix.rank(coordinates)
    }

    #[inline]
    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        self.matrix.select(sparse_index)
    }
}

impl<M, T> SparseMatrix2D for GenericBiMatrix2D<M, T>
where
    T: SparseMatrix2D,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
    M: SparseMatrix2D,
{
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
        self.matrix.sparse_row(row)
    }

    #[inline]
    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        self.matrix.has_entry(row, column)
    }

    #[inline]
    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.matrix.sparse_columns()
    }

    #[inline]
    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.matrix.sparse_rows()
    }
}

impl<M, T> EmptyRows for GenericBiMatrix2D<M, T>
where
    T: EmptyRows,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
    M: EmptyRows,
{
    type EmptyRowIndices<'a>
        = M::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = M::NonEmptyRowIndices<'a>
    where
        Self: 'a;

    #[inline]
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.matrix.empty_row_indices()
    }

    #[inline]
    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.matrix.non_empty_row_indices()
    }

    #[inline]
    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_empty_rows()
    }

    #[inline]
    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_non_empty_rows()
    }
}

impl<M, T> SizedRowsSparseMatrix2D for GenericBiMatrix2D<M, T>
where
    T: SizedRowsSparseMatrix2D,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
    M: SizedRowsSparseMatrix2D,
{
    type SparseRowSizes<'a>
        = M::SparseRowSizes<'a>
    where
        Self: 'a;

    #[inline]
    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.matrix.number_of_defined_values_in_row(row)
    }

    #[inline]
    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.matrix.sparse_row_sizes()
    }
}

impl<M, T> SizedSparseMatrix2D for GenericBiMatrix2D<M, T>
where
    T: SizedSparseMatrix2D,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
    M: SizedSparseMatrix2D,
{
    #[inline]
    fn rank_row(&self, row: Self::RowIndex) -> Self::SparseIndex {
        self.matrix.rank_row(row)
    }

    #[inline]
    fn select_row(&self, sparse_index: Self::SparseIndex) -> Self::RowIndex {
        self.matrix.select_row(sparse_index)
    }

    #[inline]
    fn select_column(&self, sparse_index: Self::SparseIndex) -> Self::ColumnIndex {
        self.matrix.select_column(sparse_index)
    }
}

impl<T, M> TransposableMatrix2D<T> for GenericBiMatrix2D<M, T>
where
    T: Matrix2D + Clone,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
{
    #[inline]
    fn transpose(&self) -> T {
        self.transposed.clone()
    }
}

impl<T, M> crate::traits::BiMatrix2D for GenericBiMatrix2D<M, T>
where
    T: Matrix2D + Clone,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>
        + SparseMatrix2D,
{
    type Matrix = M;
    type TransposedMatrix = T;

    #[inline]
    fn matrix(&self) -> &Self::Matrix {
        &self.matrix
    }

    #[inline]
    fn transposed(&self) -> &T {
        &self.transposed
    }
}

impl<M, T> SquareMatrix for GenericBiMatrix2D<M, T>
where
    T: SquareMatrix,
    M: SquareMatrix<Index = T::Index> + TransposableMatrix2D<T>,
{
    type Index = M::Index;

    fn order(&self) -> Self::Index {
        self.matrix.order()
    }
}

impl<M, T> SparseSquareMatrix for GenericBiMatrix2D<M, T>
where
    T: SparseSquareMatrix,
    M: SparseSquareMatrix<Index = T::Index> + TransposableMatrix2D<T>,
{
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        self.matrix.number_of_defined_diagonal_values()
    }
}
