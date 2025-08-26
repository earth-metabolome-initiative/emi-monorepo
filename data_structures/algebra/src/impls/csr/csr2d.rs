//! Submodule providing a definition of a CSR matrix.
use core::{fmt::Debug, iter::repeat_n};

use multi_ranged::Step;
use num_traits::ConstZero;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash)]
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

impl<SparseIndex: ConstZero, RowIndex: ConstZero, ColumnIndex: ConstZero> Default
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
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
> SparseMatrixMut for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: SparseMatrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex, SparseIndex = SparseIndex>,
{
    type MinimalShape = Self::Coordinates;

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self::with_sparse_shaped_capacity((RowIndex::ZERO, ColumnIndex::ZERO), number_of_values)
    }

    fn with_sparse_shape((number_of_rows, number_of_columns): Self::MinimalShape) -> Self {
        Self::with_sparse_shaped_capacity((number_of_rows, number_of_columns), SparseIndex::ZERO)
    }

    fn with_sparse_shaped_capacity(
        (number_of_rows, number_of_columns): Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        let mut offsets = Vec::with_capacity(number_of_rows.into_usize() + 1);
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
> Matrix for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    type Coordinates = (RowIndex, ColumnIndex);

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows.into_usize(), self.number_of_columns.into_usize()]
    }
}

impl<
    SparseIndex,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize,
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
            self.number_of_rows.into_usize(),
            self.offsets.len()
        );
        self.number_of_rows
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.number_of_columns
    }
}

impl<
    SparseIndex,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize,
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
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
> SparseMatrix for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type SparseIndex = SparseIndex;
    type SparseCoordinates<'a>
        = super::CSR2DView<'a, Self>
    where
        Self: 'a;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        if self.is_empty() {
            return None;
        }
        let last_row = self
            .offsets
            .len()
            .checked_sub(2)
            .and_then(|x| RowIndex::try_from_usize(x).ok())
            .expect("The offsets should always have at least one element.");
        debug_assert!(
            self.number_of_defined_values_in_row(last_row) > ColumnIndex::ZERO,
            "The last row stores in the offsets should always have at least one column, as all subsequent empty rows should be left implicit and represented by the `number_of_rows` field."
        );
        let last_column = self
            .column_indices
            .last()
            .copied()
            .expect("The column indices cannot be empty if the matrix is not empty.");
        Some((last_row, last_column))
    }

    fn is_empty(&self) -> bool {
        self.number_of_defined_values() == SparseIndex::ZERO
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
> SizedSparseMatrix for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.offsets.last().copied().unwrap_or(SparseIndex::ZERO)
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
> RankSelectSparseMatrix for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        (self.select_row(sparse_index), self.select_column(sparse_index))
    }

    fn rank(&self, &(row_index, column_index): &Self::Coordinates) -> Self::SparseIndex {
        let start = self.rank_row(row_index);
        let end = self.rank_row(row_index + RowIndex::ONE);
        let Ok(relative_column_index) =
            self.column_indices[start.into_usize()..end.into_usize()].binary_search(&column_index)
        else {
            panic!("The column index {column_index} is not present in the row {row_index}.");
        };

        start + Self::SparseIndex::try_from_usize(relative_column_index)
            .unwrap_or_else(|_| {
                unreachable!(
                    "The Matrix is in an illegal state where a sparse index is greater than the number of defined values."
                )
            })
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
> SparseMatrix2D for CSR2D<SparseIndex, RowIndex, ColumnIndex>
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
        = CSR2DSizedRows<'a, Self>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        let start = self.rank_row(row).into_usize();
        let end = self.rank_row(row + RowIndex::ONE).into_usize();
        self.column_indices[start..end].iter().copied()
    }

    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        let start = self.rank_row(row).into_usize();
        let end = self.rank_row(row + RowIndex::ONE).into_usize();
        self.column_indices[start..end].binary_search(&column).is_ok()
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.column_indices.iter().copied()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.into()
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
> EmptyRows for CSR2D<SparseIndex, RowIndex, ColumnIndex>
{
    type EmptyRowIndices<'a>
        = CSR2DEmptyRowIndices<'a, Self>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = CSR2DNonEmptyRowIndices<'a, Self>
    where
        Self: 'a;
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
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
> SizedSparseMatrix2D for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    fn rank_row(&self, row: RowIndex) -> SparseIndex {
        if self.offsets.len() <= row.into_usize() && row <= self.number_of_rows() {
            return self.number_of_defined_values();
        }
        assert!(
            row <= self.number_of_rows(),
            "The matrix is in an illegal state where the row index {row} is greater than the number of rows {}, with number of columns {}, with offset size {}.",
            self.number_of_rows(),
            self.number_of_columns(),
            self.offsets.len()
        );
        self.offsets[row.into_usize()]
    }

    fn select_row(&self, sparse_index: Self::SparseIndex) -> Self::RowIndex {
        Self::RowIndex::try_from_usize(
            self.offsets.binary_search(&sparse_index).unwrap_or_else(|x| x),
        ).unwrap_or_else(|_| {
            unreachable!(
                "The Matrix is in an illegal state where a sparse index is greater than the number of defined values."
            )
        })
    }

    fn select_column(&self, sparse_index: Self::SparseIndex) -> Self::ColumnIndex {
        self.column_indices[sparse_index.into_usize()]
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
> SizedRowsSparseMatrix2D for CSR2D<SparseIndex, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type SparseRowSizes<'a>
        = CSR2DSizedRowsizes<'a, Self>
    where
        Self: 'a;

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.into()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        if let Ok(out_degree) = (self.rank_row(row + RowIndex::ONE) - self.rank_row(row)).try_into()
        {
            out_degree
        } else {
            unreachable!(
                "The Matrix is in an illegal state where a sparse row has a number of defined columns greater than the data type of the columns allows for."
            )
        }
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
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
                return Err(MutabilityError::UnorderedCoordinate((row, column)));
            }

            if column == ColumnIndex::MAX {
                return Err(MutabilityError::MaxedOutColumnIndex);
            }

            if let Some(offset) = self.offsets.last_mut() {
                if *offset == SparseIndex::MAX {
                    return Err(MutabilityError::MaxedOutSparseIndex);
                }
                *offset += SparseIndex::ONE;
            } else {
                unreachable!()
            }

            // If the row is the last row, we can add the entry at the end of the column
            // indices.
            self.column_indices.push(column);
            self.number_of_columns = self.number_of_columns.max(column + ColumnIndex::ONE);

            debug_assert_eq!(
                self.sparse_row(row).last(),
                Some(column),
                "The last column of the row {row} should be equal to the column {column}."
            );

            Ok(())
        } else if row.into_usize() >= self.offsets.len() - 1 {
            if self.number_of_non_empty_rows == RowIndex::MAX {
                return Err(MutabilityError::MaxedOutRowIndex);
            }
            if column == ColumnIndex::MAX {
                return Err(MutabilityError::MaxedOutColumnIndex);
            }
            if row == RowIndex::MAX {
                return Err(MutabilityError::MaxedOutSparseIndex);
            }
            let last_offset = self.offsets.last().copied().unwrap_or(SparseIndex::ZERO);
            if last_offset == SparseIndex::MAX {
                return Err(MutabilityError::MaxedOutSparseIndex);
            }
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
            self.offsets.push(last_offset + SparseIndex::ONE);

            debug_assert_eq!(
                self.sparse_row(row).last(),
                Some(column),
                "The last column of the row {row} should be equal to the column {column}."
            );

            Ok(())
        } else {
            Err(MutabilityError::UnorderedCoordinate((row, column)))
        }
    }

    fn increase_shape(
        &mut self,
        (number_of_rows, number_of_columns): Self::Coordinates,
    ) -> Result<(), Self::Error> {
        if number_of_rows < self.number_of_rows() || number_of_columns < self.number_of_columns() {
            return Err(MutabilityError::IncompatibleShape);
        }
        self.number_of_rows = self.number_of_rows.max(number_of_rows);
        self.number_of_columns = self.number_of_columns.max(number_of_columns);
        Ok(())
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    RowIndex: Step + PositiveInteger + IntoUsize + TryFromUsize,
    ColumnIndex: Step + PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
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
