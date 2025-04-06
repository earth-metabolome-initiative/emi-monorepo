//! Submodule proving a 2D matrix with implicit values.

use crate::traits::{
    ImplicitValuedMatrix, ImplicitValuedSparseMatrix, ImplicitValuedSparseRowIterator, IntoUsize,
    Matrix, Matrix2D, Matrix2DRef, RankSelectSparseMatrix, SizedRowsSparseMatrix2D,
    SizedSparseMatrix, SizedSparseMatrix2D, SizedSparseValuedMatrix, SparseMatrix, SparseMatrix2D,
    SparseValuedMatrix, SparseValuedMatrix2D, ValuedMatrix, ValuedMatrix2D,
};

#[derive(Clone, Debug)]
/// A 2D matrix with implicit values.
pub struct GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    matrix: M,
    map: Map,
}

impl<M, Map, Value> GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    /// Creates a new [`GenericImplicitValuedMatrix2D`].
    pub fn new(matrix: M, map: Map) -> Self {
        Self { matrix, map }
    }
}

impl<M, Map, Value> Matrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type Coordinates = M::Coordinates;

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<M, Map, Value> Matrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    /// Type of the row index.
    type RowIndex = M::RowIndex;
    /// Type of the column index.
    type ColumnIndex = M::ColumnIndex;

    /// Returns the number of rows of the matrix.
    fn number_of_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_rows()
    }

    /// Returns the number of columns of the matrix.
    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_columns()
    }
}

impl<M, Map, Value> Matrix2DRef for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2DRef,
    Map: Fn(M::Coordinates) -> Value,
{
    fn number_of_rows_ref(&self) -> &Self::RowIndex {
        self.matrix.number_of_rows_ref()
    }

    fn number_of_columns_ref(&self) -> &Self::ColumnIndex {
        self.matrix.number_of_columns_ref()
    }
}

impl<M, Map, Value> SparseMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseCoordinates<'a>
        = M::SparseCoordinates<'a>
    where
        Self: 'a;
    type SparseIndex = M::SparseIndex;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.matrix.sparse_coordinates()
    }

    fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }
}

impl<M, Map, Value> SizedSparseMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SizedSparseMatrix + SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.matrix.number_of_defined_values()
    }
}

impl<M, Map, Value> RankSelectSparseMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SizedSparseMatrix2D + RankSelectSparseMatrix,
    Map: Fn(M::Coordinates) -> Value,
{
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        self.matrix.rank(coordinates)
    }

    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        self.matrix.select(sparse_index)
    }
}

impl<M, Map, Value> SparseMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseColumns<'a>
        = M::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRow<'a>
        = M::SparseRow<'a>
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

impl<M, Map, Value> SizedRowsSparseMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SizedRowsSparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseRowSizes<'a>
        = M::SparseRowSizes<'a>
    where
        Self: 'a;

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.matrix.sparse_row_sizes()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.matrix.number_of_defined_values_in_row(row)
    }
}

impl<M, Map, Value> SizedSparseMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SizedSparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
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

impl<M, Map, Value> ValuedMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseCoordinates<'a>
        = M::SparseCoordinates<'a>
    where
        Self: 'a;
    type SparseIndex = M::SparseIndex;

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

impl<M, Map, Value> SparseMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseColumns<'a>
        = M::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRow<'a>
        = M::SparseRow<'a>
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

impl<M, Map, Value> SizedRowsSparseMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SizedRowsSparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseRowSizes<'a>
        = M::SparseRowSizes<'a>
    where
        Self: 'a;

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.matrix.sparse_row_sizes()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.matrix.number_of_defined_values_in_row(row)
    }
}

impl<M, Map, Value> SizedSparseMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SizedSparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
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

impl<M, Map, Value> ValuedMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type Value = Value;
}

impl<M: Matrix2D, Map, Value> ValuedMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
}

impl<M, Map, Value> ImplicitValuedMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    fn implicit_value(&self, coordinates: &Self::Coordinates) -> Self::Value {
        (self.map)(*coordinates)
    }
}

impl<M, Map, Value> SparseValuedMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseValues<'a>
        = <Self as ImplicitValuedSparseMatrix>::SparseImplicitValues<'a>
    where
        Self: 'a;

    fn sparse_values(&self) -> Self::SparseValues<'_> {
        self.sparse_implicit_values()
    }
}

impl<M, Map, Value> SizedSparseValuedMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SizedSparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    fn select_value(&self, sparse_index: Self::SparseIndex) -> Self::Value {
        self.implicit_value(&self.select(sparse_index))
    }
}

impl<M, Map, Value> SparseValuedMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseRowValues<'a>
        = ImplicitValuedSparseRowIterator<'a, Self>
    where
        Self: 'a;

    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_> {
        ImplicitValuedSparseRowIterator::new(self, row)
    }
}
