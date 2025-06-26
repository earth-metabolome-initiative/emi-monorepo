//! Submodule providing an implementation of the `RaggedVector` struct.

use std::{fmt::Debug, iter::repeat_n};

use multi_ranged::Step;
use num_traits::ConstZero;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use super::MutabilityError;
use crate::traits::{
    EmptyRows, Matrix, Matrix2D, Matrix2DRef, MatrixMut, SizedRowsSparseMatrix2D,
    SizedSparseMatrix, SparseMatrix, SparseMatrix2D, SparseMatrixMut, TransposableMatrix2D,
};

#[derive(Clone)]
/// Struct representing a ragged vector sparse matrix.
pub struct RaggedVector<SparseIndex, RowIndex, ColumnIndex> {
    /// The data of the vector.
    data: Vec<Vec<ColumnIndex>>,
    /// The number of elements in the matrix.
    pub(super) number_of_defined_values: SparseIndex,
    /// The number of columns.
    pub(super) number_of_columns: ColumnIndex,
    /// The number of rows.
    pub(super) number_of_rows: RowIndex,
    /// The number of non-empty rows.
    pub(super) number_of_non_empty_rows: RowIndex,
}

impl<SparseIndex, RowIndex, ColumnIndex> Debug for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    SparseIndex: Debug,
    RowIndex: Debug,
    ColumnIndex: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RaggedVector")
            .field("number_of_defined_values", &self.number_of_defined_values)
            .field("number_of_columns", &self.number_of_columns)
            .field("number_of_rows", &self.number_of_rows)
            .field("data", &self.data)
            .field("number_of_non_empty_rows", &self.number_of_non_empty_rows)
            .finish()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> Default
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    SparseIndex: ConstZero,
    RowIndex: ConstZero,
    ColumnIndex: ConstZero,
{
    fn default() -> Self {
        Self {
            data: Vec::new(),
            number_of_defined_values: SparseIndex::ZERO,
            number_of_columns: ColumnIndex::ZERO,
            number_of_rows: RowIndex::ZERO,
            number_of_non_empty_rows: RowIndex::ZERO,
        }
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> Matrix for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize,
{
    type Coordinates = (RowIndex, ColumnIndex);

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows.into_usize(), self.number_of_columns.into_usize()]
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> Matrix2D
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize,
{
    type RowIndex = RowIndex;
    type ColumnIndex = ColumnIndex;

    fn number_of_rows(&self) -> Self::RowIndex {
        self.number_of_rows
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.number_of_columns
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> Matrix2DRef
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize,
{
    fn number_of_columns_ref(&self) -> &Self::ColumnIndex {
        &self.number_of_columns
    }

    fn number_of_rows_ref(&self) -> &Self::RowIndex {
        &self.number_of_rows
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> SparseMatrix
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: PositiveInteger + IntoUsize,
{
    type SparseIndex = SparseIndex;
    type SparseCoordinates<'a>
        = crate::impls::CSR2DView<'a, Self>
    where
        Self: 'a;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        if self.is_empty() {
            return None;
        }
        let last_row_index =
            RowIndex::try_from_usize(self.data.len() - 1).expect("The matrix is in a valid state.");
        let last_row_with_values: &Vec<ColumnIndex> =
            self.data.last().expect("The matrix should not be empty.");
        let last_column =
            last_row_with_values.iter().last().copied().expect("The last row should not be empty.");
        Some((last_row_index, last_column))
    }

    fn is_empty(&self) -> bool {
        self.number_of_defined_values == SparseIndex::ZERO
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> SizedSparseMatrix
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: PositiveInteger + IntoUsize,
{
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.number_of_defined_values
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> SparseMatrix2D
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: PositiveInteger + IntoUsize,
{
    type SparseRow<'a>
        = core::iter::Copied<core::slice::Iter<'a, ColumnIndex>>
    where
        Self: 'a;
    type SparseColumns<'a>
        = crate::impls::CSR2DColumns<'a, Self>
    where
        Self: 'a;
    type SparseRows<'a>
        = crate::impls::CSR2DSizedRows<'a, Self>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        let slice = if row.into_usize() >= self.data.len() {
            &[]
        } else {
            self.data[row.into_usize()].as_slice()
        };
        slice.iter().copied()
    }

    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        if row.into_usize() >= self.data.len() {
            return false;
        }

        let columns_in_row = &self.data[row.into_usize()];
        columns_in_row.binary_search(&column).is_ok()
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.into()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.into()
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> EmptyRows
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: PositiveInteger + IntoUsize,
{
    type EmptyRowIndices<'a>
        = crate::impls::CSR2DEmptyRowIndices<'a, Self>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = crate::impls::CSR2DNonEmptyRowIndices<'a, Self>
    where
        Self: 'a;
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.into()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.into()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.number_of_rows() - self.number_of_non_empty_rows()
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.number_of_non_empty_rows
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> SizedRowsSparseMatrix2D
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: PositiveInteger + IntoUsize,
{
    type SparseRowSizes<'a>
        = crate::impls::CSR2DSizedRowsizes<'a, Self>
    where
        Self: 'a;

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.into()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        ColumnIndex::try_from_usize(self.data[row.into_usize()].len()).expect(
            "The number of defined values in a row must be less than the number of columns.",
        )
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> MatrixMut
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: PositiveInteger + IntoUsize,
{
    type Entry = Self::Coordinates;
    type Error = MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        if row.into_usize() >= self.data.len() {
            self.data.extend(repeat_n(Vec::default(), row.into_usize() - self.data.len() + 1));
        }

        let columns_in_row = &mut self.data[row.into_usize()];

        if let Some(last_column) = columns_in_row.last() {
            if *last_column == column {
                return Err(MutabilityError::DuplicatedEntry((row, column)));
            }
            if *last_column > column {
                return Err(MutabilityError::UnorderedCoordinate((row, column)));
            }
        }

        columns_in_row.push(column);

        self.number_of_defined_values += SparseIndex::ONE;
        self.number_of_columns = self.number_of_columns.max(column + ColumnIndex::ONE);
        self.number_of_rows = self.number_of_rows.max(row + RowIndex::ONE);

        if columns_in_row.len() == 1 {
            self.number_of_non_empty_rows += RowIndex::ONE;
        }

        Ok(())
    }

    fn increase_shape(&mut self, shape: Self::Coordinates) -> Result<(), Self::Error> {
        if shape.0 < self.number_of_rows {
            return Err(MutabilityError::IncompatibleShape);
        }

        if shape.1 < self.number_of_columns {
            return Err(MutabilityError::IncompatibleShape);
        }

        self.number_of_rows = shape.0;
        self.number_of_columns = shape.1;

        Ok(())
    }
}

impl<SparseIndex, RowIndex, ColumnIndex>
    TransposableMatrix2D<RaggedVector<SparseIndex, ColumnIndex, RowIndex>>
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    SparseIndex: PositiveInteger + IntoUsize,
{
    fn transpose(&self) -> RaggedVector<SparseIndex, ColumnIndex, RowIndex> {
        let mut transposed: RaggedVector<SparseIndex, ColumnIndex, RowIndex> =
            RaggedVector::with_sparse_shaped_capacity(
                (self.number_of_columns, self.number_of_rows),
                self.number_of_defined_values,
            );

        // We iterate over the rows of the matrix.
        for (row, column) in self.sparse_coordinates() {
            transposed.add((column, row)).expect("The addition should not fail.");
        }

        transposed
    }
}

impl<SparseIndex, RowIndex, ColumnIndex> SparseMatrixMut
    for RaggedVector<SparseIndex, RowIndex, ColumnIndex>
where
    Self: SparseMatrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex, SparseIndex = SparseIndex>,
    SparseIndex: PositiveInteger + IntoUsize,
    RowIndex: Step + PositiveInteger + IntoUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
{
    type MinimalShape = Self::Coordinates;

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self::with_sparse_shaped_capacity((RowIndex::ZERO, ColumnIndex::ZERO), number_of_values)
    }

    fn with_sparse_shape(shape: Self::MinimalShape) -> Self {
        Self::with_sparse_shaped_capacity(shape, SparseIndex::ZERO)
    }

    fn with_sparse_shaped_capacity(
        (number_of_rows, number_of_columns): Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        Self {
            data: vec![Vec::new(); number_of_rows.into_usize()],
            number_of_defined_values: number_of_values,
            number_of_columns,
            number_of_rows,
            number_of_non_empty_rows: RowIndex::ZERO,
        }
    }
}
