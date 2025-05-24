//! Submodule providing traits to represent a sparse valued matrix.
//!
//! A sparse valued matrix is a matrix where not all the values are defined.

use common_traits::prelude::TotalOrd;

use super::ValuedMatrix2D;
use crate::traits::{ImplicitValuedMatrix, SparseMatrix2D, SparseValuedMatrix, ValuedMatrix};

/// Trait defining a bi-dimensional matrix.
pub trait SparseValuedMatrix2D: SparseMatrix2D + ValuedMatrix2D + SparseValuedMatrix {
    /// Iterator over the values of a row.
    type SparseRowValues<'a>: Iterator<Item = <Self as ValuedMatrix>::Value>
        + DoubleEndedIterator<Item = <Self as ValuedMatrix>::Value>
        + Clone
    where
        Self: 'a;

    #[inline]
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

    #[inline]
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
    ) -> Option<(Self::Value, Self::ColumnIndex)>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_row_values(row)
            .zip(self.sparse_row(row))
            .max_by(|(_, value1), (_, value2)| value1.total_cmp(value2))
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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
        self.row_indices().map(move |row| self.sparse_row_max_value(row))
    }

    #[inline]
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
        self.row_indices().map(move |row| self.sparse_row_min_value(row))
    }
}

impl<M: SparseValuedMatrix2D> SparseValuedMatrix2D for &M {
    type SparseRowValues<'a>
        = M::SparseRowValues<'a>
    where
        Self: 'a;

    #[inline]
    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_> {
        (*self).sparse_row_values(row)
    }
}

/// Iterator over the values of a row.
pub struct ImplicitValuedSparseRowIterator<'matrix, M: SparseMatrix2D> {
    iter: M::SparseRow<'matrix>,
    row: M::RowIndex,
    matrix: &'matrix M,
}

impl<M: SparseMatrix2D> Clone for ImplicitValuedSparseRowIterator<'_, M> {
    fn clone(&self) -> Self {
        Self { iter: self.iter.clone(), row: self.row, matrix: self.matrix }
    }
}

impl<'matrix, M: SparseMatrix2D> ImplicitValuedSparseRowIterator<'matrix, M> {
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

impl<M> Iterator for ImplicitValuedSparseRowIterator<'_, M>
where
    M: SparseMatrix2D + ImplicitValuedMatrix,
{
    type Item = M::Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|column| self.matrix.implicit_value(&(self.row, column)))
    }
}

impl<'matrix, M> ExactSizeIterator for ImplicitValuedSparseRowIterator<'matrix, M>
where
    M: SparseMatrix2D + ImplicitValuedMatrix,
    M::SparseRow<'matrix>: ExactSizeIterator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<M> DoubleEndedIterator for ImplicitValuedSparseRowIterator<'_, M>
where
    M: SparseMatrix2D + ImplicitValuedMatrix,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|column| self.matrix.implicit_value(&(self.row, column)))
    }
}

/// Trait defining a bi-dimensional matrix.
pub trait ImplicitSparseValuedMatrix2D: SparseValuedMatrix2D + ImplicitValuedMatrix {
    /// Iterator over the sparse columns of a row.
    type SparseRowImplicitValues<'a>: Iterator<Item = Self::Value>
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

impl<M> ImplicitSparseValuedMatrix2D for M
where
    M: SparseValuedMatrix2D + ImplicitValuedMatrix,
{
    type SparseRowImplicitValues<'a>
        = ImplicitValuedSparseRowIterator<'a, M>
    where
        Self: 'a;

    #[inline]
    fn sparse_row_implicit_values(&self, row: Self::RowIndex) -> Self::SparseRowImplicitValues<'_> {
        ImplicitValuedSparseRowIterator { iter: self.sparse_row(row), row, matrix: self }
    }
}
