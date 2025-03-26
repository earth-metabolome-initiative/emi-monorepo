//! Submodule providing the `ValuedCsr2D` type, a 2D CSR matrix which stores
//! values in addition to the row and column indices.

use core::fmt::Debug;

use super::{MutabilityError, CSR2D};
use crate::traits::{
    IntoUsize, Matrix, Matrix2D, Matrix2DRef, MatrixMut, One, PositiveInteger, SparseMatrix, SparseMatrix2D, SparseMatrixMut, SparseValuedMatrix, ValuedMatrix, ValuedMatrix2D, ValuedSparseMatrix2D, Zero
};

/// A 2D CSR matrix which stores values in addition to the row and column
/// indices.
pub struct ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value> {
    csr: CSR2D<SparseIndex, RowIndex, ColumnIndex>,
    values: Vec<Value>,
}

impl<SparseIndex: Debug, RowIndex: Debug, ColumnIndex: Debug, Value: Debug> Debug
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ValuedCSR2D").field("csr", &self.csr).field("values", &self.values).finish()
    }
}

impl<SparseIndex: Zero, RowIndex: Zero, ColumnIndex: Zero, Value> Default
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
{
    fn default() -> Self {
        Self { csr: CSR2D::default(), values: Vec::default() }
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> Matrix2D
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: Matrix2D,
{
    type ColumnIndex = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as Matrix2D>::ColumnIndex;
    type RowIndex = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as Matrix2D>::RowIndex;

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.csr.number_of_columns()
    }

    fn number_of_rows(&self) -> Self::RowIndex {
        self.csr.number_of_rows()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> Matrix2DRef
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: Matrix2DRef,
{
    fn number_of_columns_ref(&self) -> &Self::ColumnIndex {
        self.csr.number_of_columns_ref()
    }

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
    type SparseRowSizes<'a>
        = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrix2D>::SparseRowSizes<'a>
    where
        Self: 'a;
    type EmptyRowIndices<'a> = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrix2D>::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a> = <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrix2D>::NonEmptyRowIndices<'a>
    where
        Self: 'a;

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.csr.sparse_rows()
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.csr.sparse_columns()
    }

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.csr.sparse_row(row)
    }

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.csr.sparse_row_sizes()
    }

    fn rank(&self, row: Self::RowIndex) -> Self::SparseIndex {
        self.csr.rank(row)
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.csr.number_of_defined_values_in_row(row)
    }

    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.csr.empty_row_indices()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.csr.non_empty_row_indices()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.csr.number_of_empty_rows()
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.csr.number_of_non_empty_rows()
    }

    fn number_of_empty_columns(&self) -> Self::ColumnIndex {
        self.csr.number_of_empty_columns()
    }

    fn number_of_non_empty_columns(&self) -> Self::ColumnIndex {
        self.csr.number_of_non_empty_columns()
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

    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.csr.number_of_defined_values()
    }

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.csr.sparse_coordinates()
    }
}

impl<SparseIndex: Zero, RowIndex: Zero, ColumnIndex: Zero, Value> MatrixMut
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: MatrixMut<
        Entry = (RowIndex, ColumnIndex),
        Error = MutabilityError<CSR2D<SparseIndex, RowIndex, ColumnIndex>>,
    >
        + Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type Entry = (RowIndex, ColumnIndex, Value);
    type Error = MutabilityError<Self>;

    fn add(&mut self, (row, column, value): Self::Entry) -> Result<(), Self::Error> {
        self.csr.add((row, column))?;
        self.values.push(value);
        Ok(())
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + From<SparseIndex>,
        Value,
    > SparseMatrixMut for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    CSR2D<SparseIndex, RowIndex, ColumnIndex>: SparseMatrixMut
        + Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>
        + MatrixMut<Entry = (RowIndex, ColumnIndex), Error = MutabilityError<CSR2D<SparseIndex, RowIndex, ColumnIndex>>>,
{
    type MinimalShape =
        <CSR2D<SparseIndex, RowIndex, ColumnIndex> as SparseMatrixMut>::MinimalShape;

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self {
            csr: CSR2D::with_sparse_capacity(number_of_values),
            values: Vec::with_capacity(number_of_values.into_usize()),
        }
    }

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

    fn sparse_values(&self) -> Self::SparseValues<'_> {
        self.values.iter().cloned()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex, Value> ValuedSparseMatrix2D
    for ValuedCSR2D<SparseIndex, RowIndex, ColumnIndex, Value>
where
    Self: SparseMatrix2D + SparseValuedMatrix<Value = Value>,
    Self::Value: Clone,
{
    type SparseRowValues<'a>
        = core::iter::Cloned<core::slice::Iter<'a, Self::Value>>
    where
        Self: 'a;

    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_> {
        let start = self.rank(row).into_usize();
        let end = self.rank(row + Self::RowIndex::ONE).into_usize();
        self.values[start..end].iter().cloned()
    }
}
