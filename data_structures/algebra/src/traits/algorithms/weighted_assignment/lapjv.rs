//! Submodule providing an implementation of the LAPJV algorithm.

mod errors;
mod inner;

pub use errors::LAPJVError;
use inner::Inner;
use num_traits::ConstZero;
use numeric_common_traits::prelude::{Finite, Number, TryFromUsize};

use crate::{
    impls::PaddedMatrix2D,
    traits::{DenseValuedMatrix2D, SparseValuedMatrix2D},
};

/// Trait defining the LAPJV algorithm for solving the Weighted Assignment
/// Problem.
pub trait LAPJV: DenseValuedMatrix2D + Sized
where
    Self::Value: Number,
    Self::ColumnIndex: TryFromUsize,
{
    #[allow(clippy::type_complexity)]
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
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `max_cost` is not a finite number (`LAPJVError::MaximalCostNotFinite`)
    /// - `max_cost` is not positive (`LAPJVError::MaximalCostNotPositive`)
    /// - The matrix is not square (`LAPJVError::NonSquareMatrix`)
    /// - The matrix is empty (`LAPJVError::EmptyMatrix`)
    /// - The matrix contains zero values (`LAPJVError::ZeroValues`)
    /// - The matrix contains negative values (`LAPJVError::NegativeValues`)
    /// - The matrix contains non-finite values (`LAPJVError::NonFiniteValues`)
    /// - The matrix contains a value larger than the maximum cost
    ///   (`LAPJVError::ValueTooLarge`)
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

impl<M: DenseValuedMatrix2D> LAPJV for M
where
    M::Value: Number,
    M::ColumnIndex: TryFromUsize,
{
}

/// Trait defining the LAPJV algorithm for solving the Weighted Assignment
/// Problem, adapted for the Sparse Matrix type.
pub trait SparseLAPJV: SparseValuedMatrix2D + Sized
where
    Self::Value: Number,
    Self::RowIndex: TryFromUsize,
    Self::ColumnIndex: TryFromUsize,
{
    #[allow(clippy::type_complexity)]
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
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `padding_cost` is not a finite number
    ///   (`LAPJVError::PaddingValueNotFinite`)
    /// - `padding_cost` is not positive (`LAPJVError::PaddingValueNotPositive`)
    /// - `padding_cost` is greater than or equal to `max_cost`
    ///   (`LAPJVError::ValueTooLarge`)
    /// - `max_cost` is not a finite number (`LAPJVError::MaximalCostNotFinite`)
    /// - `max_cost` is not positive (`LAPJVError::MaximalCostNotPositive`)
    /// - The matrix is not square after padding (`LAPJVError::NonSquareMatrix`)
    /// - The matrix contains values that are greater than the padding cost
    /// - The matrix contains zero, negative or non-finite values
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

impl<M: SparseValuedMatrix2D> SparseLAPJV for M
where
    M::Value: Number,
    M::RowIndex: TryFromUsize,
    M::ColumnIndex: TryFromUsize,
{
}
