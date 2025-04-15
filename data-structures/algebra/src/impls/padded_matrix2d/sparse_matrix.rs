//! Submodule providing the implementation of the `SparseMatrix2D` trait
//! and related traits for the `PaddedMatrix2D` struct.

use super::{PaddedMatrix2D, padded_coordinates::PaddedCoordinates};
use crate::{
    impls::{CSR2DColumns, ranged::SimpleRanged},
    traits::{
        IntoUsize, Matrix2D, SizedRowsSparseMatrix2D, SizedSparseMatrix, SparseMatrix,
        SparseMatrix2D, TryFromUsize, Zero,
    },
};

impl<M: SparseMatrix2D, Map> SparseMatrix for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type SparseIndex = usize;
    type SparseCoordinates<'a>
        = PaddedCoordinates<&'a Self>
    where
        Self: 'a;

    #[inline]
    fn is_empty(&self) -> bool {
        self.number_of_rows() == M::RowIndex::ZERO
            && self.number_of_columns() == M::ColumnIndex::ZERO
    }

    #[inline]
    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }
}

impl<M, Map> SizedSparseMatrix for PaddedMatrix2D<M, Map>
where
    M: SparseMatrix2D,
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    #[inline]
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.number_of_rows().into_usize() * self.number_of_columns().into_usize()
    }
}

impl<M: SparseMatrix2D, Map> SizedRowsSparseMatrix2D for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type SparseRowSizes<'a>
        = crate::impls::CSR2DRowSizes<'a, Self>
    where
        Self: 'a;

    #[inline]
    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.into()
    }

    #[inline]
    fn number_of_defined_values_in_row(&self, _row: Self::RowIndex) -> Self::ColumnIndex {
        self.number_of_columns()
    }
}

impl<M: SparseMatrix2D, Map> SparseMatrix2D for PaddedMatrix2D<M, Map>
where
    M::RowIndex: IntoUsize + TryFromUsize,
    M::ColumnIndex: IntoUsize + TryFromUsize,
{
    type SparseRow<'a>
        = SimpleRanged<M::ColumnIndex>
    where
        Self: 'a;
    type SparseColumns<'a>
        = CSR2DColumns<'a, Self>
    where
        Self: 'a;
    type SparseRows<'a>
        = crate::impls::CSR2DRows<'a, Self>
    where
        Self: 'a;
    type EmptyRowIndices<'a>
        = core::iter::Empty<M::RowIndex>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = SimpleRanged<M::RowIndex>
    where
        Self: 'a;

    #[inline]
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        core::iter::empty()
    }

    #[inline]
    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.row_indices()
    }

    #[inline]
    fn number_of_empty_rows(&self) -> Self::RowIndex {
        Self::RowIndex::ZERO
    }

    #[inline]
    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.number_of_rows()
    }

    #[inline]
    fn sparse_row(&self, _row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.column_indices()
    }

    #[inline]
    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.into()
    }

    #[inline]
    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.into()
    }
}
