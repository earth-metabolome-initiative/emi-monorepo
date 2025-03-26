//! Submodule implementing the traits relative to Matrix for the `DualMatrix`
//! struct used in the context of the Hungarian algorithm.

use algebra::prelude::{
    Bounded, Matrix2D, Number, SparseMatrix, SparseMatrix2D, SparseValuedMatrix, TotalOrd,
    ValuedMatrix, ValuedMatrix2D, ValuedSparseMatrix2D, Zero,
};

use super::DualMatrix;

impl<M: ValuedSparseMatrix2D + ?Sized> Matrix2D for DualMatrix<'_, M> {
    type ColumnIndex = M::ColumnIndex;
    type RowIndex = M::RowIndex;

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_columns()
    }

    fn number_of_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_rows()
    }
}

impl<M: ValuedSparseMatrix2D + ?Sized> SparseMatrix for DualMatrix<'_, M> {
    type SparseCoordinates<'a>
        = M::SparseCoordinates<'a>
    where
        Self: 'a;
    type SparseIndex = M::SparseIndex;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.matrix.sparse_coordinates()
    }

    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.matrix.number_of_defined_values()
    }
}

impl<M: ValuedSparseMatrix2D + ?Sized> SparseValuedMatrix for DualMatrix<'_, M>
where
    M::Value: Number,
{
    type SparseValues<'a>
        = SparseValuesIterator<'a, M>
    where
        Self: 'a;

    fn sparse_values(&self) -> Self::SparseValues<'_> {
        SparseValuesIterator {
            inner: self.matrix.sparse_values().zip(self.matrix.sparse_coordinates()),
            matrix: self,
        }
    }
}

impl<M: ValuedSparseMatrix2D + ?Sized> ValuedMatrix for DualMatrix<'_, M> {
    type Value = M::Value;
}

impl<M: ValuedSparseMatrix2D + ?Sized> ValuedMatrix2D for DualMatrix<'_, M> {}

impl<M: ValuedSparseMatrix2D + ?Sized> SparseMatrix2D for DualMatrix<'_, M> {
    type SparseColumns<'a>
        = M::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = M::SparseRows<'a>
    where
        Self: 'a;
    type SparseRow<'a>
        = M::SparseRow<'a>
    where
        Self: 'a;
    type SparseRowSizes<'a>
        = M::SparseRowSizes<'a>
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

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.matrix.sparse_columns()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.matrix.sparse_rows()
    }

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.matrix.sparse_row(row)
    }

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.matrix.sparse_row_sizes()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.matrix.number_of_defined_values_in_row(row)
    }

    fn rank(&self, row: Self::RowIndex) -> Self::SparseIndex {
        self.matrix.rank(row)
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
    
    fn number_of_empty_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_empty_columns()
    }

    fn number_of_non_empty_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_non_empty_columns()
    }
}

impl<M: ValuedSparseMatrix2D + ?Sized> ValuedSparseMatrix2D for DualMatrix<'_, M>
where
    M::Value: Number,
{
    type SparseRowValues<'a>
        = SparseRowValuesIterator<'a, M>
    where
        Self: 'a;

    fn sparse_row_min_value_and_column(
        &self,
        row: Self::RowIndex,
    ) -> Option<(Self::Value, Self::ColumnIndex)>
    where
        Self::Value: algebra::prelude::TotalOrd,
    {
        self.sparse_row_values(row)
            .zip(self.sparse_row(row))
            .min_by(|(_, value1), (_, value2)| value1.total_cmp(value2))
            .or(Some((Self::Value::MAX, Self::ColumnIndex::ZERO)))
    }

    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_> {
        SparseRowValuesIterator {
            inner: self.matrix.sparse_row_values(row).zip(self.matrix.sparse_row(row)),
            row,
            matrix: self,
        }
    }
}

/// Iterator over the values of a `DualMatrix`.
pub struct SparseValuesIterator<'a, M: ValuedSparseMatrix2D + ?Sized> {
    inner: core::iter::Zip<M::SparseValues<'a>, M::SparseCoordinates<'a>>,
    matrix: &'a DualMatrix<'a, M>,
}

impl<M: ValuedSparseMatrix2D + ?Sized> Iterator for SparseValuesIterator<'_, M>
where
    M::Value: Number,
{
    type Item = M::Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(value, (row, column))| {
            if let (Some(left_weight), Some(right_weight)) =
                (self.matrix.left_node_weight(row), self.matrix.right_node_weight(column))
            {
                value - left_weight - right_weight
            } else {
                // If either the left or right node weight is not defined, it means that
                // the smallest weight in either the row or column is the largest possible
                // value. It follows that all other values must be the largest possible value,
                // and by subtracting the largest possible value from the largest possible
                // value, we get zero.
                M::Value::ZERO
            }
        })
    }
}

impl<M: ValuedSparseMatrix2D + ?Sized> core::iter::ExactSizeIterator for SparseValuesIterator<'_, M>
where
    M::Value: Number,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<M: ValuedSparseMatrix2D + ?Sized> core::iter::DoubleEndedIterator
    for SparseValuesIterator<'_, M>
where
    M::Value: Number,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(value, (row, column))| {
            if let (Some(left_weight), Some(right_weight)) =
                (self.matrix.left_node_weight(row), self.matrix.right_node_weight(column))
            {
                value - left_weight - right_weight
            } else {
                // If either the left or right node weight is not defined, it means that
                // the smallest weight in either the row or column is the largest possible
                // value. It follows that all other values must be the largest possible value,
                // and by subtracting the largest possible value from the largest possible
                // value, we get zero.
                M::Value::ZERO
            }
        })
    }
}

/// Iterator over the values of a row of a `DualMatrix`.
pub struct SparseRowValuesIterator<'a, M: ValuedSparseMatrix2D + ?Sized> {
    inner: core::iter::Zip<M::SparseRowValues<'a>, M::SparseRow<'a>>,
    row: M::RowIndex,
    matrix: &'a DualMatrix<'a, M>,
}

impl<M: ValuedSparseMatrix2D + ?Sized> Iterator for SparseRowValuesIterator<'_, M>
where
    M::Value: Number,
{
    type Item = M::Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(value, column)| {
            if let (Some(left_weight), Some(right_weight)) =
                (self.matrix.left_node_weight(self.row), self.matrix.right_node_weight(column))
            {
                value - left_weight - right_weight
            } else {
                // If either the left or right node weight is not defined, it means that
                // the smallest weight in either the row or column is the largest possible
                // value. It follows that all other values must be the largest possible value,
                // and by subtracting the largest possible value from the largest possible
                // value, we get zero.
                M::Value::ZERO
            }
        })
    }
}

impl<M: ValuedSparseMatrix2D + ?Sized> core::iter::ExactSizeIterator
    for SparseRowValuesIterator<'_, M>
where
    M::Value: Number,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<M: ValuedSparseMatrix2D + ?Sized> core::iter::DoubleEndedIterator
    for SparseRowValuesIterator<'_, M>
where
    M::Value: Number,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(value, column)| {
            if let (Some(left_weight), Some(right_weight)) =
                (self.matrix.left_node_weight(self.row), self.matrix.right_node_weight(column))
            {
                value - left_weight - right_weight
            } else {
                // If either the left or right node weight is not defined, it means that
                // the smallest weight in either the row or column is the largest possible
                // value. It follows that all other values must be the largest possible value,
                // and by subtracting the largest possible value from the largest possible
                // value, we get zero.
                M::Value::ZERO
            }
        })
    }
}
