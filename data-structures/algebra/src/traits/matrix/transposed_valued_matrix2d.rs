//! Submodule providing the [`TransposedValuedMatrix2D`] trait.

use super::{
    BiMatrix2D, SparseBiMatrix2D, SparseMatrix2D, SymmetricMatrix2D, ValuedMatrix, ValuedMatrix2D,
    ValuedSparseMatrix2D,
};
use crate::traits::TotalOrd;

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
pub trait ValuedSparseBiMatrix2D:
    ValuedBiMatrix2D<
        ValuedMatrix = <Self as ValuedSparseBiMatrix2D>::ValuedSparseMatrix,
        ValuedTransposedMatrix = <Self as ValuedSparseBiMatrix2D>::ValuedSparseTransposedMatrix,
    > + SparseBiMatrix2D<
        SparseMatrix = <Self as ValuedSparseBiMatrix2D>::ValuedSparseMatrix,
        SparseTransposedMatrix = <Self as ValuedSparseBiMatrix2D>::ValuedSparseTransposedMatrix,
    >
{
    /// The type of the underlying valued sparse matrix.
    type ValuedSparseMatrix: ValuedSparseMatrix2D<
        RowIndex = Self::RowIndex,
        ColumnIndex = Self::ColumnIndex,
        Value = Self::Value,
    >;
    /// The type of the underlying transposed valued sparse matrix.
    type ValuedSparseTransposedMatrix: ValuedSparseMatrix2D<
        RowIndex = Self::ColumnIndex,
        ColumnIndex = Self::RowIndex,
        Value = Self::Value,
    >;

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

    /// Returns an iterator over the values of a column.
    ///
    /// # Arguments
    ///
    /// * `column`: The column.
    fn sparse_column_values(
        &self,
        column: Self::ColumnIndex,
    ) -> <Self::ValuedSparseTransposedMatrix as ValuedSparseMatrix2D>::SparseRowValues<'_> {
        self.transposed().sparse_row_values(column)
    }
}

impl<M> ValuedSparseBiMatrix2D for M
where
    M: ValuedBiMatrix2D + SparseMatrix2D,
    M::ValuedMatrix: ValuedSparseMatrix2D<
        RowIndex = Self::RowIndex,
        ColumnIndex = Self::ColumnIndex,
        Value = Self::Value,
    >,
    M::ValuedTransposedMatrix: ValuedSparseMatrix2D<
        RowIndex = Self::ColumnIndex,
        ColumnIndex = Self::RowIndex,
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
    type SymmetricSparseValuedMatrix: ValuedSparseMatrix2D<
        RowIndex = Self::RowIndex,
        ColumnIndex = Self::ColumnIndex,
        Value = Self::Value,
    >;
}

impl<M> SparseSymmetricValuedMatrix2D for M
where
    M: SymmetricValuedMatrix2D + SparseMatrix2D,
    M::SymmetricValuedMatrix: ValuedSparseMatrix2D<
        RowIndex = Self::RowIndex,
        ColumnIndex = Self::ColumnIndex,
        Value = Self::Value,
    >,
{
    type SymmetricSparseValuedMatrix = M::SymmetricValuedMatrix;
}
