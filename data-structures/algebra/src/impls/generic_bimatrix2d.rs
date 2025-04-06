//! [`GenericBiMatrix2D`] data structure, which provides a wrapper to a matrix
//! and its transposed version.

use crate::prelude::*;

/// [`GenericBiMatrix2D`] data structure, which provides a wrapper to a matrix
/// and its transposed version.
pub struct GenericBiMatrix2D<M, T> {
    /// The matrix.
    matrix: M,
    /// The transposed matrix.
    transposed: T,
}

impl<
        T: Matrix2D,
        M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
    > GenericBiMatrix2D<M, T>
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

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_columns()
    }

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

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.matrix.sparse_coordinates()
    }

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
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.matrix.number_of_defined_values()
    }

    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        self.matrix.select(sparse_index)
    }

    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        self.matrix.rank(coordinates)
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

    type EmptyRowIndices<'a>
        = M::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = M::NonEmptyRowIndices<'a>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.matrix.sparse_row(row)
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.matrix.sparse_columns()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.matrix.sparse_rows()
    }

    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.matrix.empty_row_indices()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.matrix.non_empty_row_indices()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_empty_rows()
    }

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

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.matrix.number_of_defined_values_in_row(row)
    }

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
    fn rank_row(&self, row: Self::RowIndex) -> Self::SparseIndex {
        self.matrix.rank_row(row)
    }

    fn select_row(&self, sparse_index: Self::SparseIndex) -> Self::RowIndex {
        self.matrix.select_row(sparse_index)
    }

    fn select_column(&self, sparse_index: Self::SparseIndex) -> Self::ColumnIndex {
        self.matrix.select_column(sparse_index)
    }
}

impl<T, M> TransposableMatrix2D<T> for GenericBiMatrix2D<M, T>
where
    T: Matrix2D + Clone,
    M: TransposableMatrix2D<T, RowIndex = T::ColumnIndex, ColumnIndex = T::RowIndex>,
{
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

    fn matrix(&self) -> &Self::Matrix {
        &self.matrix
    }

    fn transposed(&self) -> &T {
        &self.transposed
    }
}
