//! Submodule providing traits to represent a dense valued matrix.
//!
//! A dense valued matrix is a matrix where all the values are defined.

use common_traits::prelude::TotalOrd;

use super::ValuedMatrix2D;
use crate::traits::{DenseMatrix2D, DenseValuedMatrix, ValuedMatrix};

/// Trait defining a bi-dimensional matrix.
pub trait DenseValuedMatrix2D: DenseMatrix2D + ValuedMatrix2D + DenseValuedMatrix {
    /// Iterator over the values of a row.
    type RowValues<'a>: Iterator<Item = <Self as ValuedMatrix>::Value>
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
    fn row_max_value(&self, row: Self::RowIndex) -> Option<<Self as ValuedMatrix>::Value>
    where
        Self::Value: TotalOrd,
    {
        self.row_values(row).max_by(TotalOrd::total_cmp)
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
    fn row_max_value_and_column(
        &self,
        row: Self::RowIndex,
    ) -> Option<(Self::ColumnIndex, Self::Value)>
    where
        Self::Value: TotalOrd,
    {
        self.column_indices()
            .zip(self.row_values(row))
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
    fn row_min_value(&self, row: Self::RowIndex) -> Option<<Self as ValuedMatrix>::Value>
    where
        Self::Value: TotalOrd,
    {
        self.row_values(row).min_by(TotalOrd::total_cmp)
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
    fn row_min_value_and_column(
        &self,
        row: Self::RowIndex,
    ) -> Option<(Self::ColumnIndex, Self::Value)>
    where
        Self::Value: TotalOrd,
    {
        self.column_indices()
            .zip(self.row_values(row))
            .min_by(|(_, value1), (_, value2)| value1.total_cmp(value2))
    }

    /// Returns an iterator of the values of the row.
    fn row_values(&self, row: Self::RowIndex) -> Self::RowValues<'_>;
}

impl<M: DenseValuedMatrix2D> DenseValuedMatrix2D for &M {
    type RowValues<'a>
        = M::RowValues<'a>
    where
        Self: 'a;

    #[inline]
    fn row_values(&self, row: Self::RowIndex) -> Self::RowValues<'_> {
        (*self).row_values(row)
    }
}
