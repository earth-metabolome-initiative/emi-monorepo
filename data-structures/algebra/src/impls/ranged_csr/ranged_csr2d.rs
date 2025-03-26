//! Submodule providing a definition of a CSR matrix.
use core::{fmt::Debug, iter::repeat_n};

use super::ranged::RangedError;
use crate::prelude::*;

#[derive(Clone)]
/// A compressed sparse row matrix.
pub struct RangedCSR2D<SparseIndex, RowIndex, R: Ranged> {
    /// The number of elements in the matrix.
    pub(super) number_of_defined_values: SparseIndex,
    /// The number of columns.
    pub(super) number_of_columns: R::Item,
    /// The number of rows.
    pub(super) number_of_rows: RowIndex,
    /// The destination ranges.
    pub(super) ranges: Vec<R>,
    /// The number of non-empty rows.
    pub(super) number_of_non_empty_rows: RowIndex,
}

impl<SparseIndex: Debug, RowIndex: Debug, R: Ranged> Debug
    for RangedCSR2D<SparseIndex, RowIndex, R>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RangedCSR2D")
            .field("number_of_elements", &self.number_of_defined_values)
            .field("number_of_columns", &self.number_of_columns)
            .field("number_of_rows", &self.number_of_rows)
            .field("column_indices", &self.ranges)
            .finish()
    }
}

impl<SparseIndex: Zero, RowIndex: Zero, R: Ranged> Default
    for RangedCSR2D<SparseIndex, RowIndex, R>
{
    fn default() -> Self {
        Self {
            number_of_defined_values: SparseIndex::ZERO,
            number_of_columns: R::Step::ZERO,
            number_of_rows: RowIndex::ZERO,
            ranges: Vec::new(),
            number_of_non_empty_rows: RowIndex::ZERO,
        }
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        R: Ranged,
    > SparseMatrixMut for RangedCSR2D<SparseIndex, RowIndex, R>
where
    Self: SparseMatrix2D<RowIndex = RowIndex, ColumnIndex = R::Step, SparseIndex = SparseIndex>,
{
    type MinimalShape = Self::Coordinates;

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self::with_sparse_shaped_capacity((RowIndex::ZERO, R::Step::ZERO), number_of_values)
    }

    fn with_sparse_shaped_capacity(
        (number_of_rows, number_of_columns): Self::MinimalShape,
        _number_of_values: Self::SparseIndex,
    ) -> Self {
        Self {
            number_of_defined_values: SparseIndex::ZERO,
            number_of_columns,
            number_of_rows,
            ranges: Vec::with_capacity(number_of_rows.into_usize()),
            number_of_non_empty_rows: RowIndex::ZERO,
        }
    }
}

impl<SparseIndex, RowIndex: PositiveInteger + IntoUsize + TryFromUsize, R: Ranged> Matrix2D
    for RangedCSR2D<SparseIndex, RowIndex, R>
{
    type RowIndex = RowIndex;
    type ColumnIndex = R::Step;

    fn number_of_rows(&self) -> Self::RowIndex {
        self.number_of_rows
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.number_of_columns
    }
}

impl<SparseIndex, RowIndex: PositiveInteger + IntoUsize + TryFromUsize, R: Ranged> Matrix2DRef
    for RangedCSR2D<SparseIndex, RowIndex, R>
{
    fn number_of_columns_ref(&self) -> &Self::ColumnIndex {
        &self.number_of_columns
    }

    fn number_of_rows_ref(&self) -> &Self::RowIndex {
        &self.number_of_rows
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        R: Ranged,
    > SparseMatrix for RangedCSR2D<SparseIndex, RowIndex, R>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = R::Step>,
{
    type SparseIndex = SparseIndex;
    type SparseCoordinates<'a>
        = crate::impls::CSR2DView<'a, Self>
    where
        Self: 'a;

    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.number_of_defined_values
    }

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        R: Ranged,
    > SparseMatrix2D for RangedCSR2D<SparseIndex, RowIndex, R>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = R::Step>,
{
    type SparseRow<'a>
        = R
    where
        Self: 'a;
    type SparseColumns<'a>
        = crate::impls::CSR2DColumns<'a, Self>
    where
        Self: 'a;
    type SparseRows<'a>
        = crate::impls::CSR2DRows<'a, Self>
    where
        Self: 'a;
    type SparseRowSizes<'a>
        = crate::impls::CSR2DRowSizes<'a, Self>
    where
        Self: 'a;
    type EmptyRowIndices<'a>
        = crate::impls::CSR2DEmptyRowIndices<'a, Self>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = crate::impls::CSR2DNonEmptyRowIndices<'a, Self>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.ranges[row.into_usize()].clone()
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.into()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.into()
    }

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.into()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.ranges[row.into_usize()].number_of_elements()
    }

    fn rank(&self, _row: RowIndex) -> SparseIndex {
        unimplemented!()
    }

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

    fn number_of_empty_columns(&self) -> Self::ColumnIndex {
        self.number_of_columns() - self.number_of_non_empty_columns()
    }

    fn number_of_non_empty_columns(&self) -> Self::ColumnIndex {
        let mut non_empty_columns = vec![false; self.number_of_columns().into_usize()];
        let mut number_of_non_empty_columns = Self::ColumnIndex::ZERO;
        for column in self.sparse_columns() {
            if !non_empty_columns[column.into_usize()] {
                number_of_non_empty_columns += Self::ColumnIndex::ONE;
                non_empty_columns[column.into_usize()] = true;
            }
        }
        number_of_non_empty_columns
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        R: Ranged,
    > MatrixMut for RangedCSR2D<SparseIndex, RowIndex, R>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = R::Step>,
{
    type Entry = Self::Coordinates;
    type Error = MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        if row.into_usize() >= self.ranges.len() {
            self.ranges.extend(repeat_n(R::default(), row.into_usize() - self.ranges.len() + 1));
        }

        let range = &mut self.ranges[row.into_usize()];

        if let Err(err) = range.add(column) {
            match err {
                RangedError::DuplicateElement => {
                    return Err(MutabilityError::DuplicatedEntry((row, column)))
                }
                RangedError::OutOfRange => {
                    return Err(MutabilityError::UnorderedColumnIndex(column))
                }
            }
        }

        self.number_of_defined_values += SparseIndex::ONE;
        self.number_of_columns = self.number_of_columns.max(column + R::Step::ONE);
        self.number_of_rows = self.number_of_rows.max(row + RowIndex::ONE);

        if range.number_of_elements() == R::Step::ONE {
            self.number_of_non_empty_rows += RowIndex::ONE;
        }

        Ok(())
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, R1: Ranged, R2: Ranged>
    TransposableMatrix2D<RangedCSR2D<SparseIndex, R1::Step, R2>>
    for RangedCSR2D<SparseIndex, R2::Step, R1>
where
    Self: Matrix2D<RowIndex = R2::Step, ColumnIndex = R1::Step>,
    RangedCSR2D<SparseIndex, R1::Step, R2>: Matrix2D<RowIndex = R1::Step, ColumnIndex = R2::Step>,
{
    fn transpose(&self) -> RangedCSR2D<SparseIndex, R1::Step, R2> {
        // We initialize the transposed matrix.
        let mut transposed: RangedCSR2D<SparseIndex, R1::Step, R2> =
            RangedCSR2D::with_sparse_shaped_capacity(
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
