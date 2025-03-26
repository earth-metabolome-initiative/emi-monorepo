//! Submodule providing a definition of a CSR matrix.
use core::{fmt::Debug, iter::repeat_n};

use crate::prelude::*;

#[derive(Clone)]
/// A compressed sparse row matrix.
pub struct CSR2D<SparseIndex, RowIndex, ColumnIndex> {
    /// The row pointers.
    pub(super) offsets: Vec<SparseIndex>,
    /// The number of columns.
    pub(super) number_of_columns: ColumnIndex,
    /// The number of rows.
    pub(super) number_of_rows: RowIndex,
    /// The column indices.
    pub(super) column_indices: Vec<ColumnIndex>,
    /// The number of non-empty rows.
    pub(super) number_of_non_empty_rows: RowIndex,
}

impl<SparseIndex: Debug, RowIndex: Debug, ColumnIndex: Debug> Debug
    for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CSR2D")
            .field("offsets", &self.offsets)
            .field("number_of_columns", &self.number_of_columns)
            .field("number_of_rows", &self.number_of_rows)
            .field("column_indices", &self.column_indices)
            .field("number_of_non_empty_rows", &self.number_of_non_empty_rows)
            .finish()
    }
}

impl<SparseIndex: Zero, RowIndex: Zero, ColumnIndex: Zero> Default
    for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    fn default() -> Self {
        Self {
            offsets: vec![SparseIndex::ZERO],
            number_of_columns: ColumnIndex::ZERO,
            number_of_rows: RowIndex::ZERO,
            column_indices: Vec::new(),
            number_of_non_empty_rows: RowIndex::ZERO,
        }
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    > SparseMatrixMut for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: SparseMatrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex, SparseIndex = SparseIndex>,
{
    type MinimalShape = Self::Coordinates;

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self::with_sparse_shaped_capacity((RowIndex::ZERO, ColumnIndex::ZERO), number_of_values)
    }

    fn with_sparse_shaped_capacity(
        (number_of_rows, number_of_columns): Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        let mut offsets = Vec::with_capacity((number_of_rows + RowIndex::ONE).into_usize());
        offsets.push(SparseIndex::ZERO);
        Self {
            offsets,
            number_of_columns,
            number_of_rows,
            column_indices: Vec::with_capacity(number_of_values.into_usize()),
            number_of_non_empty_rows: RowIndex::ZERO,
        }
    }
}

impl<
        SparseIndex,
        RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
        ColumnIndex: PositiveInteger + IntoUsize,
    > Matrix2D for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    type RowIndex = RowIndex;
    type ColumnIndex = ColumnIndex;

    fn number_of_rows(&self) -> Self::RowIndex {
        debug_assert!(
            !self.offsets.is_empty(),
            "The offsets should always have at least one element."
        );
        debug_assert!(
            self.offsets.len() - 1 <= self.number_of_rows.into_usize(),
            "The matrix is in an illegal state where the number of rows {} is less than the number of rows in the offsets {}.",
            self.number_of_rows.into_usize(), self.offsets.len()
        );
        self.number_of_rows
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.number_of_columns
    }
}

impl<
        SparseIndex,
        RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
        ColumnIndex: PositiveInteger + IntoUsize,
    > Matrix2DRef for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    fn number_of_rows_ref(&self) -> &Self::RowIndex {
        &self.number_of_rows
    }

    fn number_of_columns_ref(&self) -> &Self::ColumnIndex {
        &self.number_of_columns
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    > SparseMatrix for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type SparseIndex = SparseIndex;
    type SparseCoordinates<'a>
        = super::CSR2DView<'a, Self>
    where
        Self: 'a;

    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.offsets.last().copied().unwrap_or(SparseIndex::ZERO)
    }

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    > SparseMatrix2D for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type SparseRow<'a>
        = core::iter::Copied<core::slice::Iter<'a, Self::ColumnIndex>>
    where
        Self: 'a;
    type SparseColumns<'a>
        = core::iter::Copied<core::slice::Iter<'a, Self::ColumnIndex>>
    where
        Self: 'a;
    type SparseRows<'a>
        = CSR2DRows<'a, Self>
    where
        Self: 'a;
    type SparseRowSizes<'a>
        = CSR2DRowSizes<'a, Self>
    where
        Self: 'a;
    type EmptyRowIndices<'a>
        = CSR2DEmptyRowIndices<'a, Self>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = CSR2DNonEmptyRowIndices<'a, Self>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        let start = self.rank(row).into_usize();
        let end = self.rank(row + RowIndex::ONE).into_usize();
        self.column_indices[start..end].iter().copied()
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.column_indices.iter().copied()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.into()
    }

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.into()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        if let Ok(out_degree) = (self.rank(row + RowIndex::ONE) - self.rank(row)).try_into() {
            out_degree
        } else {
            unreachable!(
                "The Matrix is in an illegal state where a sparse row has a number of defined columns greated than the data type of the columns allows for."
            )
        }
    }

    fn rank(&self, row: RowIndex) -> SparseIndex {
        if self.offsets.len() <= row.into_usize() && row <= self.number_of_rows() {
            return self.number_of_defined_values();
        }
        self.offsets[row.into_usize()]
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.number_of_non_empty_rows
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.number_of_rows() - self.number_of_non_empty_rows()
    }

    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.into()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.into()
    }

    fn number_of_empty_columns(&self) -> Self::ColumnIndex {
        self.number_of_columns() - self.number_of_non_empty_columns()
    }

    fn number_of_non_empty_columns(&self) -> Self::ColumnIndex {
        let mut non_empty_columns = vec![false; self.number_of_columns().into_usize()];
        let mut number_of_non_empty_columns = ColumnIndex::ZERO;
        for column in self.sparse_columns() {
            if !non_empty_columns[column.into_usize()] {
                number_of_non_empty_columns += ColumnIndex::ONE;
            }
            non_empty_columns[column.into_usize()] = true;
        }
        number_of_non_empty_columns
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    > MatrixMut for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type Entry = Self::Coordinates;
    type Error = crate::error::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        if !self.is_empty() && row.into_usize() == self.offsets.len() - 2 {
            // We check that the provided column is not repeated.
            if self.sparse_row(row).last().is_some_and(|last| last == column) {
                return Err(MutabilityError::DuplicatedEntry((row, column)));
            }
            // We check that the provided column is provided in sorted order.
            if self.sparse_row(row).last().is_some_and(|last| last > column) {
                return Err(MutabilityError::UnorderedColumnIndex(column));
            }
            // If the row is the last row, we can add the entry at the end of the column
            // indices.
            self.column_indices.push(column);
            self.number_of_columns = self.number_of_columns.max(column + ColumnIndex::ONE);
            if let Some(offset) = self.offsets.last_mut() {
                *offset += SparseIndex::ONE;
            } else {
                unreachable!()
            }
            Ok(())
        } else if row.into_usize() >= self.offsets.len() - 1 {
            // If the row is the next row, we can add the entry at the end of the column
            // indices.
            self.offsets.extend(repeat_n(
                self.number_of_defined_values(),
                (row.into_usize() + 1) - self.offsets.len(),
            ));
            self.number_of_non_empty_rows += RowIndex::ONE;
            self.column_indices.push(column);
            self.number_of_columns = self.number_of_columns.max(column + ColumnIndex::ONE);
            self.number_of_rows = self.number_of_rows.max(row + RowIndex::ONE);
            self.offsets
                .push(self.offsets.last().copied().unwrap_or(SparseIndex::ZERO) + SparseIndex::ONE);
            Ok(())
        } else {
            Err(MutabilityError::UnorderedRowIndex(row))
        }
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    > TransposableMatrix2D<CSR2D<SparseIndex, ColumnIndex, RowIndex>>
    for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
    CSR2D<SparseIndex, ColumnIndex, RowIndex>:
        Matrix2D<RowIndex = ColumnIndex, ColumnIndex = RowIndex>,
{
    fn transpose(&self) -> CSR2D<SparseIndex, ColumnIndex, RowIndex> {
        // We initialize the transposed matrix.
        let mut transposed: CSR2D<SparseIndex, ColumnIndex, RowIndex> = CSR2D {
            offsets: vec![SparseIndex::ZERO; self.number_of_columns().into_usize() + 1],
            number_of_columns: self.number_of_rows(),
            number_of_rows: self.number_of_columns(),
            column_indices: vec![RowIndex::ZERO; self.number_of_defined_values().into_usize()],
            number_of_non_empty_rows: self.number_of_columns(),
        };

        // First, we proceed to compute the number of elements in each column.
        for column in self.column_indices.iter().copied() {
            transposed.offsets[column.into_usize() + 1] += SparseIndex::ONE;
        }

        // Then, we compute the prefix sum of the degrees to get the offsets.
        let mut prefix_sum = SparseIndex::ZERO;
        for offset in &mut transposed.offsets {
            prefix_sum += *offset;
            transposed.number_of_non_empty_rows +=
                if *offset > SparseIndex::ZERO { ColumnIndex::ONE } else { ColumnIndex::ZERO };
            *offset = prefix_sum;
        }

        // Finally, we fill the column indices.
        let mut degree = vec![SparseIndex::ZERO; self.number_of_columns.into_usize()];
        for (row, column) in self.sparse_coordinates() {
            let current_degree: &mut SparseIndex = &mut degree[column.into_usize()];
            let index = *current_degree + transposed.offsets[column.into_usize()];
            transposed.column_indices[index.into_usize()] = row;
            *current_degree += SparseIndex::ONE;
        }

        transposed
    }
}
