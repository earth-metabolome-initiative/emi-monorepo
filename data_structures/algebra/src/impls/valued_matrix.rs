//! Submodule providing the `ValuedCsr2D` type, a 2D CSR matrix which stores
//! values in addition to the row and column indices.

use core::fmt::Debug;

use multi_ranged::Step;
use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use super::{CSR2D, MutabilityError};
use crate::traits::{
    EmptyRows, Matrix, Matrix2D, Matrix2DRef, MatrixMut, RankSelectSparseMatrix,
    SizedRowsSparseMatrix2D, SizedSparseMatrix, SizedSparseMatrix2D, SizedSparseValuedMatrix,
    SparseMatrix, SparseMatrix2D, SparseMatrixMut, SparseValuedMatrix, SparseValuedMatrix2D,
    ValuedMatrix, ValuedMatrix2D,
};

#[cfg(feature = "arbitrary")]
mod arbitrary_impl;

/// A 2D CSR matrix which stores values in addition to the row and column
/// indices.
pub struct ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value> {
    csr: CSR2D<SparseIndex, RowIndex, ColumnIndex>,
    values: Vec<Value>,
}

impl<
    SparseIndex: PositiveInteger + TryFromUsize + IntoUsize,
    RowIndex: Step + TryFromUsize + PositiveInteger + IntoUsize,
    ColumnIndex: Step + TryFromUsize + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    Value,
    const ROWS: usize,
    const COLS: usize,
> TryFrom<[[Value; COLS]; ROWS]> for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    type Error = MutabilityError<Self>;

    fn try_from(value: [[Value; COLS]; ROWS]) -> Result<Self, Self::Error> {
        let mut valued_csr: ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value> =
            ValuedCSR2D::with_sparse_shaped_capacity(
                (
                    RowIndex::try_from_usize(ROWS)
                        .map_err(|_| MutabilityError::<Self>::MaxedOutRowIndex)?,
                    ColumnIndex::try_from_usize(COLS)
                        .map_err(|_| MutabilityError::<Self>::MaxedOutColumnIndex)?,
                ),
                SparseIndex::try_from_usize(ROWS * COLS)
                    .map_err(|_| MutabilityError::<Self>::MaxedOutSparseIndex)?,
            );
        for (row, row_values) in valued_csr.row_indices().zip(value) {
            for (column, value) in valued_csr.column_indices().zip(row_values) {
                valued_csr.add((row, column, value)).expect("Failed to add value to ValuedCSR2D");
            }
        }

        Ok(valued_csr)
    }
}

impl<SparseIndex: Debug, RowIndex: Debug, ColumnIndex: Debug, Value: Debug> Debug
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ValuedCSR2D").field("csr", &self.csr).field("values", &self.values).finish()
    }
}

impl<SparseIndex: ConstZero, RowIndex: ConstZero, ColumnIndex: ConstZero, Value> Default
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    fn default() -> Self {
        Self { csr: CSR2D::default(), values: Vec::default() }
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> Matrix
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: Matrix2D,
{
    type Coordinates = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as Matrix>::Coordinates;

    #[inline]
    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> Matrix2D
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: Matrix2D,
{
    type ColumnIndex = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as Matrix2D>::ColumnIndex;
    type RowIndex = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as Matrix2D>::RowIndex;

    #[inline]
    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.csr.number_of_columns()
    }

    #[inline]
    fn number_of_rows(&self) -> Self::RowIndex {
        self.csr.number_of_rows()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> Matrix2DRef
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: Matrix2DRef,
{
    #[inline]
    fn number_of_columns_ref(&self) -> &Self::ColumnIndex {
        self.csr.number_of_columns_ref()
    }

    #[inline]
    fn number_of_rows_ref(&self) -> &Self::RowIndex {
        self.csr.number_of_rows_ref()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> SparseMatrix2D
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: SparseMatrix2D,
{
    type SparseRow<'a>
        = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrix2D>::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrix2D>::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrix2D>::SparseRows<'a>
    where
        Self: 'a;

    #[inline]
    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.csr.sparse_rows()
    }

    #[inline]
    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.csr.sparse_columns()
    }

    #[inline]
    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.csr.sparse_row(row)
    }

    #[inline]
    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        self.csr.has_entry(row, column)
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> EmptyRows
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: EmptyRows,
{
    type EmptyRowIndices<'a>
        = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as EmptyRows>::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as EmptyRows>::NonEmptyRowIndices<'a>
    where
        Self: 'a;

    #[inline]
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.csr.empty_row_indices()
    }

    #[inline]
    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.csr.non_empty_row_indices()
    }

    #[inline]
    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.csr.number_of_empty_rows()
    }

    #[inline]
    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.csr.number_of_non_empty_rows()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> SizedRowsSparseMatrix2D
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: SizedRowsSparseMatrix2D<
            RowIndex = RowIndex,
            ColumnIndex = ColumnIndex,
            SparseIndex = SparseIndex,
        >,
{
    type SparseRowSizes<'a>
        = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SizedRowsSparseMatrix2D>::SparseRowSizes<'a>
    where
        Self: 'a;

    #[inline]
    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.csr.sparse_row_sizes()
    }

    #[inline]
    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.csr.number_of_defined_values_in_row(row)
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> SizedSparseMatrix2D
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: SizedSparseMatrix2D<
            RowIndex = RowIndex,
            ColumnIndex = ColumnIndex,
            SparseIndex = SparseIndex,
        >,
{
    #[inline]
    fn rank_row(&self, row: Self::RowIndex) -> Self::SparseIndex {
        self.csr.rank_row(row)
    }

    #[inline]
    fn select_row(&self, sparse_index: Self::SparseIndex) -> Self::RowIndex {
        self.csr.select_row(sparse_index)
    }

    #[inline]
    fn select_column(&self, sparse_index: Self::SparseIndex) -> Self::ColumnIndex {
        self.csr.select_column(sparse_index)
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> SparseMatrix
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: SparseMatrix2D,
{
    type SparseIndex = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrix>::SparseIndex;
    type SparseCoordinates<'a>
        = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrix>::SparseCoordinates<'a>
    where
        Self: 'a;

    #[inline]
    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.csr.sparse_coordinates()
    }

    #[inline]
    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        self.csr.last_sparse_coordinates()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.csr.is_empty()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> SizedSparseMatrix
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: SizedSparseMatrix2D,
{
    #[inline]
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.csr.number_of_defined_values()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> RankSelectSparseMatrix
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: RankSelectSparseMatrix<SparseIndex = SparseIndex>,
{
    #[inline]
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        self.csr.rank(coordinates)
    }

    #[inline]
    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        self.csr.select(sparse_index)
    }
}

impl<SparseIndex: ConstZero, RowIndex: ConstZero, ColumnIndex: ConstZero, Value> MatrixMut
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: MatrixMut<
            Entry = (RowIndex, ColumnIndex),
            Error = MutabilityError<CSR2D<SparseIndex, RowIndex, ColumnIndex>>,
        > + Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type Entry = (RowIndex, ColumnIndex, Value);
    type Error = MutabilityError<Self>;

    #[inline]
    fn add(&mut self, (row, column, value): Self::Entry) -> Result<(), Self::Error> {
        self.csr.add((row, column))?;
        self.values.push(value);
        Ok(())
    }

    fn increase_shape(&mut self, shape: Self::Coordinates) -> Result<(), Self::Error> {
        self.csr.increase_shape(shape)?;
        Ok(())
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    Value,
> SparseMatrixMut for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: SparseMatrixMut
        + Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>
        + MatrixMut<
            Entry = (RowIndex, ColumnIndex),
            Error = MutabilityError<CSR2D<SparseIndex, RowIndex, ColumnIndex>>,
        >,
{
    type MinimalShape =
        <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrixMut>::MinimalShape;

    #[inline]
    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self {
            csr: CSR2D::with_sparse_capacity(number_of_values),
            values: Vec::with_capacity(number_of_values.into_usize()),
        }
    }

    fn with_sparse_shape(shape: Self::MinimalShape) -> Self {
        Self { csr: CSR2D::with_sparse_shape(shape), values: Vec::new() }
    }

    #[inline]
    fn with_sparse_shaped_capacity(
        shape: Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        Self {
            csr: CSR2D::with_sparse_shaped_capacity(shape, number_of_values),
            values: Vec::with_capacity(number_of_values.into_usize()),
        }
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> ValuedMatrix
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    Self: Matrix,
{
    type Value = Value;
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> ValuedMatrix2D
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    Self: Matrix2D,
{
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> SparseValuedMatrix
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    Self: SparseMatrix + ValuedMatrix<Value = Value>,
    Self::Value: Clone,
{
    type SparseValues<'a>
        = core::iter::Cloned<core::slice::Iter<'a, Self::Value>>
    where
        Self: 'a;

    #[inline]
    fn sparse_values(&self) -> Self::SparseValues<'_> {
        self.values.iter().cloned()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> SizedSparseValuedMatrix
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    Self: SparseValuedMatrix<Value = Value>,
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    Self::Value: Clone,
{
    #[inline]
    fn select_value(&self, sparse_index: Self::SparseIndex) -> Self::Value {
        self.values[sparse_index.into_usize()].clone()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> SparseValuedMatrix2D
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    Self: SizedSparseMatrix2D + SparseValuedMatrix<Value = Value>,
    Self::Value: Clone,
{
    type SparseRowValues<'a>
        = core::iter::Cloned<core::slice::Iter<'a, Self::Value>>
    where
        Self: 'a;

    #[inline]
    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_> {
        let start = self.rank_row(row).into_usize();
        let end = self.rank_row(row + Self::RowIndex::ONE).into_usize();
        self.values[start..end].iter().cloned()
    }
}
