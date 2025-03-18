//! Submodule defining the [`ValuedMatrix2D`] trait.

use super::{ImplicitValuedMatrix, Matrix2D, SparseMatrix2D, SparseValuedMatrix, ValuedMatrix};
use crate::traits::TotalOrd;

/// Trait defining a bidimensional matrix.
pub trait ValuedMatrix2D: Matrix2D + ValuedMatrix {}

/// Trait defining a bidimensional matrix.
pub trait ValuedSparseMatrix2D: SparseMatrix2D + ValuedMatrix2D + SparseValuedMatrix {
    /// Iterator over the values of a row.
    type SparseRowValues<'a>: ExactSizeIterator<Item = <Self as ValuedMatrix>::Value>
        + DoubleEndedIterator<Item = <Self as ValuedMatrix>::Value>
    where
        Self: 'a;

    /// Returns the maximal value on the row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row.
    ///
    /// # Returns
    ///
    /// The maximal value on the row, if the row has at least one value.
    fn sparse_row_max_value(&self, row: Self::RowIndex) -> Option<<Self as ValuedMatrix>::Value>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_row_values(row).max_by(TotalOrd::total_cmp)
    }

    /// Returns the maximal value and its column on the row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row.
    ///
    /// # Returns
    ///
    /// The maximal value and its column on the row,
    /// if the row has at least one value.
    fn sparse_row_max_value_and_column(
        &self,
        row: Self::RowIndex,
    ) -> Option<(<Self as ValuedMatrix>::Value, Self::ColumnIndex)>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_row_values(row)
            .zip(self.sparse_row(row))
            .max_by(|(_, value1), (_, value2)| value1.total_cmp(value2))
    }

    /// Returns the minimal value on the row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row.
    ///
    /// # Returns
    ///
    /// The minimal value on the row, if the row has at least one value.
    fn sparse_row_min_value(&self, row: Self::RowIndex) -> Option<Self::Value>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_row_values(row).min_by(TotalOrd::total_cmp)
    }

    /// Returns the minimal value and its column on the row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row.
    ///
    /// # Returns
    ///
    /// The minimal value and its column on the row,
    /// if the row has at least one value.
    fn sparse_row_min_value_and_column(
        &self,
        row: Self::RowIndex,
    ) -> Option<(Self::Value, Self::ColumnIndex)>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_row_values(row)
            .zip(self.sparse_row(row))
            .min_by(|(_, value1), (_, value2)| value1.total_cmp(value2))
    }

    /// Returns an iterator over the values of a row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row.
    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_>;

    /// Returns an iterator over the maximum values of the rows.
    ///
    /// # Returns
    ///
    /// An iterator over the maximum values of the rows.
    /// If a row has no values, the iterator will return `None`.
    fn sparse_row_max_values(&self) -> impl Iterator<Item = Option<Self::Value>>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_rows().map(move |row| self.sparse_row_max_value(row))
    }

    /// Returns an iterator over the minimum values of the rows.
    ///
    /// # Returns
    ///
    /// An iterator over the minimum values of the rows.
    /// If a row has no values, the iterator will return `None`.
    fn sparse_row_min_values(&self) -> impl Iterator<Item = Option<Self::Value>>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_rows().map(move |row| self.sparse_row_min_value(row))
    }
}

/// Iterator over the values of a row.
pub struct ImplicitValuedSparseRowIteraror<'matrix, M: SparseMatrix2D> {
    iter: M::SparseRow<'matrix>,
    row: M::RowIndex,
    matrix: &'matrix M,
}

impl<'matrix, M: SparseMatrix2D> ImplicitValuedSparseRowIteraror<'matrix, M> {
    /// Creates a new iterator over the values of a row.
    ///
    /// # Arguments
    ///
    /// * `matrix`: The matrix.
    /// * `row`: The row.
    pub fn new(matrix: &'matrix M, row: M::RowIndex) -> Self {
        Self { iter: matrix.sparse_row(row), row, matrix }
    }
}

impl<M> Iterator for ImplicitValuedSparseRowIteraror<'_, M>
where
    M: SparseMatrix2D + ImplicitValuedMatrix,
{
    type Item = M::Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|column| self.matrix.implicit_value(&(self.row, column)))
    }
}

impl<M> ExactSizeIterator for ImplicitValuedSparseRowIteraror<'_, M>
where
    M: SparseMatrix2D + ImplicitValuedMatrix,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<M> DoubleEndedIterator for ImplicitValuedSparseRowIteraror<'_, M>
where
    M: SparseMatrix2D + ImplicitValuedMatrix,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|column| self.matrix.implicit_value(&(self.row, column)))
    }
}

/// Trait defining a bidimensional matrix.
pub trait ImplicitValuedSparseMatrix2D: ValuedSparseMatrix2D + ImplicitValuedMatrix {
    /// Iterator over the sparse columns of a row.
    type SparseRowImplicitValues<'a>: ExactSizeIterator<Item = Self::Value>
        + DoubleEndedIterator<Item = Self::Value>
    where
        Self: 'a;

    /// Returns an iterator over the values of a row.
    ///
    /// # Arguments
    ///
    /// * `row`: The row.
    fn sparse_row_implicit_values(&self, row: Self::RowIndex) -> Self::SparseRowImplicitValues<'_>;
}

impl<M> ImplicitValuedSparseMatrix2D for M
where
    M: ValuedSparseMatrix2D + ImplicitValuedMatrix,
{
    type SparseRowImplicitValues<'a>
        = ImplicitValuedSparseRowIteraror<'a, M>
    where
        Self: 'a;

    fn sparse_row_implicit_values(&self, row: Self::RowIndex) -> Self::SparseRowImplicitValues<'_> {
        ImplicitValuedSparseRowIteraror { iter: self.sparse_row(row), row, matrix: self }
    }
}
