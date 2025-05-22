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
            .field("number_of_defined_values", &self.number_of_defined_values)
            .field("number_of_columns", &self.number_of_columns)
            .field("number_of_rows", &self.number_of_rows)
            .field("column_indices", &self.ranges)
            .field("number_of_non_empty_rows", &self.number_of_non_empty_rows)
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
    RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
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

impl<SparseIndex, RowIndex: PositiveInteger + IntoUsize + TryFromUsize, R: Ranged> Matrix
    for RangedCSR2D<SparseIndex, RowIndex, R>
{
    type Coordinates = (RowIndex, R::Step);

    #[inline]
    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows.into_usize(), self.number_of_columns.into_usize()]
    }
}

impl<SparseIndex, RowIndex: PositiveInteger + IntoUsize + TryFromUsize, R: Ranged> Matrix2D
    for RangedCSR2D<SparseIndex, RowIndex, R>
{
    type RowIndex = RowIndex;
    type ColumnIndex = R::Step;

    #[inline]
    fn number_of_rows(&self) -> Self::RowIndex {
        self.number_of_rows
    }

    #[inline]
    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.number_of_columns
    }
}

impl<SparseIndex, RowIndex: PositiveInteger + IntoUsize + TryFromUsize, R: Ranged> Matrix2DRef
    for RangedCSR2D<SparseIndex, RowIndex, R>
{
    #[inline]
    fn number_of_columns_ref(&self) -> &Self::ColumnIndex {
        &self.number_of_columns
    }

    #[inline]
    fn number_of_rows_ref(&self) -> &Self::RowIndex {
        &self.number_of_rows
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize,
    RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
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

    #[inline]
    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        if self.is_empty() {
            return None;
        }
        let last_row_index = RowIndex::try_from_usize(self.ranges.len() - 1)
            .expect("The matrix is in a valid state.");
        let last_row_with_values = self.ranges.last().expect("The matrix should not be empty.");
        let last_column =
            last_row_with_values.clone().last().expect("The last row should not be empty.");
        Some((last_row_index, last_column))
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.number_of_defined_values == SparseIndex::ZERO
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize,
    RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
    R: Ranged,
> SizedSparseMatrix for RangedCSR2D<SparseIndex, RowIndex, R>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = R::Step>,
{
    #[inline]
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.number_of_defined_values
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize,
    RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
    R: Ranged,
> SparseMatrix2D for RangedCSR2D<SparseIndex, RowIndex, R>
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
        = crate::impls::CSR2DSizedRows<'a, Self>
    where
        Self: 'a;

    #[inline]
    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.ranges[row.into_usize()].clone()
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

impl<
    SparseIndex: PositiveInteger + IntoUsize,
    RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
    R: Ranged,
> EmptyRows for RangedCSR2D<SparseIndex, RowIndex, R>
{
    type EmptyRowIndices<'a>
        = crate::impls::CSR2DEmptyRowIndices<'a, Self>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = crate::impls::CSR2DNonEmptyRowIndices<'a, Self>
    where
        Self: 'a;
    #[inline]
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.into()
    }

    #[inline]
    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.into()
    }

    #[inline]
    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.number_of_rows() - self.number_of_non_empty_rows()
    }

    #[inline]
    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.number_of_non_empty_rows
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize,
    RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
    R: Ranged,
> SizedRowsSparseMatrix2D for RangedCSR2D<SparseIndex, RowIndex, R>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = R::Step>,
{
    type SparseRowSizes<'a>
        = crate::impls::CSR2DSizedRowsizes<'a, Self>
    where
        Self: 'a;

    #[inline]
    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.into()
    }

    #[inline]
    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.ranges[row.into_usize()].number_of_elements()
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize,
    RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
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
                    return Err(MutabilityError::DuplicatedEntry((row, column)));
                }
                RangedError::OutOfRange => {
                    return Err(MutabilityError::UnorderedCoordinate((row, column)));
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

    fn increase_shape(&mut self, shape: Self::Coordinates) -> Result<(), Self::Error> {
        if shape.0 < self.number_of_rows || shape.1 < self.number_of_columns {
            return Err(MutabilityError::IncompatibleShape);
        }

        self.ranges.extend(repeat_n(R::default(), shape.0.into_usize() - self.ranges.len()));

        self.number_of_rows = shape.0;
        self.number_of_columns = shape.1;

        Ok(())
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, R1: Ranged, R2: Ranged>
    TransposableMatrix2D<RangedCSR2D<SparseIndex, R1::Step, R2>>
    for RangedCSR2D<SparseIndex, R2::Step, R1>
where
    Self: Matrix2D<RowIndex = R2::Step, ColumnIndex = R1::Step>,
    RangedCSR2D<SparseIndex, R1::Step, R2>: Matrix2D<RowIndex = R1::Step, ColumnIndex = R2::Step>,
    R1::Step: TryFromUsize,
    R2::Step: TryFromUsize,
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
