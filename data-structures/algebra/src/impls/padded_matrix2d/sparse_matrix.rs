//! Submodule providing the implementation of the `SparseMatrix2D` trait
//! and related traits for the `PaddedMatrix2D` struct.

use super::{padded_coordinates::PaddedCoordinates, PaddedMatrix2D};
use crate::{
    impls::{ranged::SimpleRanged, CSR2DColumns},
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

    fn is_empty(&self) -> bool {
        self.number_of_rows() == M::RowIndex::ZERO
            && self.number_of_columns() == M::ColumnIndex::ZERO
    }

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

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.into()
    }

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

    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        core::iter::empty()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.row_indices()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        Self::RowIndex::ZERO
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.number_of_rows()
    }

    fn sparse_row(&self, _row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.column_indices()
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.into()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.into()
    }
}
