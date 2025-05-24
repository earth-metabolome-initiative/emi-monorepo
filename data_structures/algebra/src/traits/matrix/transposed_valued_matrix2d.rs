//! Submodule providing the [`TransposedValuedMatrix2D`] trait.

use common_traits::prelude::TotalOrd;

use super::{
    BiMatrix2D, SizedSparseBiMatrix2D, SparseMatrix2D, SparseValuedMatrix2D, SymmetricMatrix2D,
    ValuedMatrix, ValuedMatrix2D,
};

/// Trait defining a sparse matrix which supports efficient operations on
/// columns.
pub trait ValuedBiMatrix2D:
    BiMatrix2D<
        Matrix = <Self as ValuedBiMatrix2D>::ValuedMatrix,
        TransposedMatrix = <Self as ValuedBiMatrix2D>::ValuedTransposedMatrix,
    > + ValuedMatrix2D
{
    /// The type of the underlying valued matrix.
    type ValuedMatrix: ValuedMatrix2D<
            RowIndex = Self::RowIndex,
            ColumnIndex = Self::ColumnIndex,
            Value = Self::Value,
        >;
    /// The type of the underlying transposed valued matrix.
    type ValuedTransposedMatrix: ValuedMatrix2D<
            RowIndex = Self::ColumnIndex,
            ColumnIndex = Self::RowIndex,
            Value = Self::Value,
        >;
}

/// Trait defining a sparse matrix which supports efficient operations on
/// columns.
pub trait ValuedSizedSparseBiMatrix2D:
    ValuedBiMatrix2D<
        ValuedMatrix = <Self as ValuedSizedSparseBiMatrix2D>::ValuedSparseMatrix,
        ValuedTransposedMatrix = <Self as ValuedSizedSparseBiMatrix2D>::ValuedSparseTransposedMatrix,
    > + SizedSparseBiMatrix2D<
        SizedSparseMatrix = <Self as ValuedSizedSparseBiMatrix2D>::ValuedSparseMatrix,
        SizedSparseTransposedMatrix = <Self as ValuedSizedSparseBiMatrix2D>::ValuedSparseTransposedMatrix,
    >
{
    /// The type of the underlying valued sparse matrix.
    type ValuedSparseMatrix: SparseValuedMatrix2D<
        RowIndex = Self::RowIndex,
        ColumnIndex = Self::ColumnIndex,
        Value = Self::Value,
    >;
    /// The type of the underlying transposed valued sparse matrix.
    type ValuedSparseTransposedMatrix: SparseValuedMatrix2D<
        RowIndex = Self::ColumnIndex,
        ColumnIndex = Self::RowIndex,
        Value = Self::Value,
    >;

    #[inline]
    /// Returns the maximal value on the column.
    ///
    /// # Arguments
    ///
    /// * `column`: The column.
    ///
    /// # Returns
    ///
    /// The maximal value on the column, if the column has at least one value.
    fn sparse_column_max_value(
        &self,
        column: Self::ColumnIndex,
    ) -> Option<<Self as ValuedMatrix>::Value>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_column_values(column).max_by(TotalOrd::total_cmp)
    }

    #[inline]
    /// Returns the minimal value on the column.
    ///
    /// # Arguments
    ///
    /// * `column`: The column.
    ///
    /// # Returns
    ///
    /// The minimal value on the column, if the column has at least one value.
    fn sparse_column_min_value(&self, column: Self::ColumnIndex) -> Option<Self::Value>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_column_values(column).min_by(TotalOrd::total_cmp)
    }

    #[inline]
    /// Returns an iterator over the values of a column.
    ///
    /// # Arguments
    ///
    /// * `column`: The column.
    fn sparse_column_values(
        &self,
        column: Self::ColumnIndex,
    ) -> <Self::ValuedSparseTransposedMatrix as SparseValuedMatrix2D>::SparseRowValues<'_> {
        self.transposed().sparse_row_values(column)
    }
}

impl<M> ValuedSizedSparseBiMatrix2D for M
where
    M: ValuedBiMatrix2D
        + SizedSparseBiMatrix2D<
            SizedSparseMatrix = <M as ValuedBiMatrix2D>::ValuedMatrix,
            SizedSparseTransposedMatrix = <M as ValuedBiMatrix2D>::ValuedTransposedMatrix,
        >,
    M::ValuedMatrix: SparseValuedMatrix2D<
            RowIndex = Self::RowIndex,
            ColumnIndex = Self::ColumnIndex,
            SparseIndex = Self::SparseIndex,
            Value = Self::Value,
        >,
    M::ValuedTransposedMatrix: SparseValuedMatrix2D<
            RowIndex = Self::ColumnIndex,
            ColumnIndex = Self::RowIndex,
            SparseIndex = Self::SparseIndex,
            Value = Self::Value,
        >,
{
    type ValuedSparseMatrix = M::ValuedMatrix;
    type ValuedSparseTransposedMatrix = M::ValuedTransposedMatrix;
}

/// Trait defining a symmetric matrix.
pub trait SymmetricValuedMatrix2D:
    SymmetricMatrix2D<SymmetricMatrix = <Self as SymmetricValuedMatrix2D>::SymmetricValuedMatrix>
    + ValuedBiMatrix2D<
        ValuedMatrix = <Self as SymmetricValuedMatrix2D>::SymmetricValuedMatrix,
        ValuedTransposedMatrix = <Self as SymmetricValuedMatrix2D>::SymmetricValuedMatrix,
    >
{
    /// The type of the underlying symmetric valued matrix.
    type SymmetricValuedMatrix: ValuedMatrix2D<
            RowIndex = Self::RowIndex,
            ColumnIndex = Self::ColumnIndex,
            Value = Self::Value,
        >;
}

impl<M> SymmetricValuedMatrix2D for M
where
    M: SymmetricMatrix2D
        + ValuedBiMatrix2D<
            ValuedMatrix = <M as SymmetricMatrix2D>::SymmetricMatrix,
            ValuedTransposedMatrix = <M as SymmetricMatrix2D>::SymmetricMatrix,
        >,
    M::SymmetricMatrix: ValuedMatrix2D<
            RowIndex = Self::RowIndex,
            ColumnIndex = Self::ColumnIndex,
            Value = Self::Value,
        >,
{
    type SymmetricValuedMatrix = M::SymmetricMatrix;
}

/// Trait defining a sparse symmetric matrix.
pub trait SparseSymmetricValuedMatrix2D:
    SymmetricValuedMatrix2D<
    SymmetricValuedMatrix = <Self as SparseSymmetricValuedMatrix2D>::SymmetricSparseValuedMatrix,
>
{
    /// The underlying symmetric sparse matrix type.
    type SymmetricSparseValuedMatrix: SparseValuedMatrix2D<
            RowIndex = Self::RowIndex,
            ColumnIndex = Self::ColumnIndex,
            Value = Self::Value,
        >;
}

impl<M> SparseSymmetricValuedMatrix2D for M
where
    M: SymmetricValuedMatrix2D + SparseMatrix2D,
    M::SymmetricValuedMatrix: SparseValuedMatrix2D<
            RowIndex = Self::RowIndex,
            ColumnIndex = Self::ColumnIndex,
            Value = Self::Value,
        >,
{
    type SymmetricSparseValuedMatrix = M::SymmetricValuedMatrix;
}
