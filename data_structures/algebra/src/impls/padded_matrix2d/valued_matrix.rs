//! Submodule for the `PaddedMatrix2D` struct, implementing the `ValuedMatrix`
//! trait and its related traits.

use multi_ranged::Step;
use numeric_common_traits::prelude::{IntoUsize, TryFromUsize};

use super::{PaddedMatrix2D, imputed_row_values::ImputedRowValues};
use crate::{
    impls::M2DValues,
    traits::{
        DenseMatrix, DenseMatrix2D, DenseValuedMatrix, DenseValuedMatrix2D, SparseValuedMatrix,
        SparseValuedMatrix2D, ValuedMatrix, ValuedMatrix2D,
    },
};

impl<M: ValuedMatrix2D, Map> ValuedMatrix for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type Value = M::Value;
}

impl<M: ValuedMatrix2D, Map> ValuedMatrix2D for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
}

impl<M: ValuedMatrix2D, Map> DenseMatrix for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
}

impl<M: ValuedMatrix2D, Map> DenseMatrix2D for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
}

impl<M: SparseValuedMatrix2D, Map> DenseValuedMatrix for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize + Step,
    Map: Fn((M::RowIndex, M::ColumnIndex)) -> M::Value,
{
    type Values<'a>
        = M2DValues<'a, Self>
    where
        Self: 'a;

    #[inline]
    fn value(&self, (row_index, column_index): Self::Coordinates) -> Self::Value {
        if row_index >= self.matrix.number_of_rows()
            || column_index >= self.matrix.number_of_columns()
        {
            return (self.map)((row_index, column_index));
        }

        self.matrix
            .sparse_row(row_index)
            .zip(self.matrix.sparse_row_values(row_index))
            .find(|(sparse_column_index, _)| *sparse_column_index == column_index)
            .map_or_else(|| (self.map)((row_index, column_index)), |(_, value)| value)
    }

    #[inline]
    fn values(&self) -> Self::Values<'_> {
        self.into()
    }
}

impl<M: SparseValuedMatrix2D, Map> SparseValuedMatrix for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize + Step,
    Map: Fn((M::RowIndex, M::ColumnIndex)) -> M::Value,
{
    type SparseValues<'a>
        = M2DValues<'a, Self>
    where
        Self: 'a;

    #[inline]
    fn sparse_values(&self) -> Self::SparseValues<'_> {
        self.into()
    }
}

impl<M: SparseValuedMatrix2D, Map> SparseValuedMatrix2D for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize + Step,
    Map: Fn((M::RowIndex, M::ColumnIndex)) -> M::Value,
{
    type SparseRowValues<'a>
        = ImputedRowValues<'a, M, Map>
    where
        Self: 'a;

    #[inline]
    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_> {
        ImputedRowValues::new(self, row)
    }
}

impl<M: SparseValuedMatrix2D, Map> DenseValuedMatrix2D for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize + Step,
    Map: Fn((M::RowIndex, M::ColumnIndex)) -> M::Value,
{
    type RowValues<'a>
        = ImputedRowValues<'a, M, Map>
    where
        Self: 'a;

    #[inline]
    fn row_values(&self, row: Self::RowIndex) -> Self::RowValues<'_> {
        ImputedRowValues::new(self, row)
    }
}
