//! Submodule providing an implementation of the LAPJV algorithm.

mod errors;
mod inner;
use std::fmt::Debug;

pub use errors::LAPJVError;
use inner::Inner;

use crate::{
    impls::PaddedMatrix2D,
    traits::{DenseValuedMatrix2D, Finite, Number, SparseValuedMatrix2D, TryFromUsize, Zero},
};

/// Trait defining the LAPJV algorithm for solving the Weighted Assignment
/// Problem.
pub trait LAPJV: DenseValuedMatrix2D + Sized + Debug
where
    Self::Value: Number,
    Self::ColumnIndex: TryFromUsize,
{
    /// Computes the weighted assignment using the LAPJV algorithm.
    ///
    /// # Arguments
    ///
    /// * `max_cost`: The upper bound for the cost of the assignment.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the row and column indices of the
    /// assignment.
    fn lapjv(
        &self,
        max_cost: Self::Value,
    ) -> Result<Vec<(Self::RowIndex, Self::ColumnIndex)>, LAPJVError> {
        if !max_cost.is_finite() {
            return Err(LAPJVError::MaximalCostNotFinite);
        }

        if max_cost <= Self::Value::ZERO {
            return Err(LAPJVError::MaximalCostNotPositive);
        }

        let mut inner = Inner::new(self, max_cost)?;
        inner.column_reduction()?;
        inner.reduction_transfer();

        // We execute TWICE augmenting row reductions.
        inner.augmenting_row_reduction();
        inner.augmenting_row_reduction();

        inner.augmentation();

        Ok(inner.into())
    }
}

impl<M: DenseValuedMatrix2D + Debug> LAPJV for M
where
    M::Value: Number,
    M::ColumnIndex: TryFromUsize,
{
}

/// Trait defining the LAPJV algorithm for solving the Weighted Assignment
/// Problem, adapted for the Sparse Matrix type.
pub trait SparseLAPJV: SparseValuedMatrix2D + Sized + Debug
where
    Self::Value: Number,
    Self::RowIndex: TryFromUsize,
    Self::ColumnIndex: TryFromUsize,
{
    /// Computes the weighted assignment using the LAPJV algorithm.
    ///
    /// # Arguments
    ///
    /// * `padding_cost`: The cost of padding the matrix to make it square.
    /// * `max_cost`: The upper bound for the cost of the assignment.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the row and column indices of the
    /// assignment.
    fn sparse_lapjv(
        &self,
        padding_cost: Self::Value,
        max_cost: Self::Value,
    ) -> Result<Vec<(Self::RowIndex, Self::ColumnIndex)>, LAPJVError> {
        if !padding_cost.is_finite() {
            return Err(LAPJVError::PaddingValueNotFinite);
        }
        if padding_cost <= Self::Value::ZERO {
            return Err(LAPJVError::PaddingValueNotPositive);
        }

        if padding_cost >= max_cost {
            return Err(LAPJVError::ValueTooLarge);
        }
        if self.is_empty() {
            return Ok(vec![]);
        }

        debug_assert!(
            self.max_sparse_value().unwrap() < padding_cost,
            "The maximum value in the matrix ({}) is greater than the padding cost ({}).",
            self.max_sparse_value().unwrap(),
            padding_cost
        );

        let padding: PaddedMatrix2D<&'_ Self, _> =
            PaddedMatrix2D::new(self, |_| padding_cost).map_err(|_| LAPJVError::NonSquareMatrix)?;
        let assignment = padding.lapjv(max_cost)?;

        Ok(assignment
            .into_iter()
            .filter(|&(row_index, column_index)| !padding.is_imputed((row_index, column_index)))
            .collect())
    }
}

impl<M: SparseValuedMatrix2D + Debug> SparseLAPJV for M
where
    M::Value: Number,
    M::RowIndex: TryFromUsize,
    M::ColumnIndex: TryFromUsize,
{
}
