//! Submodule providing a definition of a CSR matrix.
use core::marker::PhantomData;

use crate::prelude::*;

/// A compressed sparse row matrix.
pub struct CSR2D<Offset, RowIndex, ColumnIndex> {
    /// The row pointers.
    pub(super) offsets: Vec<Offset>,
    /// The number of columns.
    pub(super) number_of_columns: ColumnIndex,
    /// The column indices.
    pub(super) column_indices: Vec<ColumnIndex>,
    /// The row indices.
    pub(super) _row_indices: PhantomData<RowIndex>,
}

impl<Offset: Zero, RowIndex, ColumnIndex: Zero> Default for CSR2D<Offset, RowIndex, ColumnIndex> {
    fn default() -> Self {
        Self {
            offsets: vec![Offset::ZERO],
            number_of_columns: ColumnIndex::ZERO,
            column_indices: Vec::new(),
            _row_indices: PhantomData,
        }
    }
}

impl<Offset: IntoUsize, RowIndex: PositiveInteger + IntoUsize, ColumnIndex: Zero>
    CSR2D<Offset, RowIndex, ColumnIndex>
{
    /// Creates a new CSR matrix with the provided number of rows and columns.
    ///
    /// # Arguments
    ///
    /// * `number_of_rows`: The number of rows.
    /// * `number_of_columns`: The number of columns.
    /// * `number_of_values`: The number of values.
    ///
    /// # Returns
    ///
    /// A new CSR matrix with the provided number of rows and columns.
    pub fn with_capacity(
        number_of_rows: RowIndex,
        number_of_columns: ColumnIndex,
        number_of_values: Offset,
    ) -> Self {
        Self {
            offsets: Vec::with_capacity((number_of_rows + RowIndex::ONE).into_usize()),
            number_of_columns,
            column_indices: Vec::with_capacity(number_of_values.into_usize()),
            _row_indices: PhantomData,
        }
    }
}

impl<
        Offset,
        RowIndex: PositiveInteger + IntoUsize + TryFromUsize,
        ColumnIndex: PositiveInteger + IntoUsize,
    > Matrix2D for CSR2D<Offset, RowIndex, ColumnIndex>
{
    type RowIndex = RowIndex;
    type ColumnIndex = ColumnIndex;

    fn number_of_rows(&self) -> Self::RowIndex {
        if let Ok(value) = RowIndex::try_from_usize(self.offsets.len() - 1) {
            value
        } else {
            unreachable!()
        }
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.number_of_columns
    }
}

impl<
        Offset: IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize,
    > SparseMatrix for CSR2D<Offset, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type SparseCoordinates<'a>
        = super::CSR2DView<'a, Self>
    where
        Self: 'a;

    fn number_of_defined_values(&self) -> usize {
        self.column_indices.len()
    }

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.into()
    }
}

impl<
        Offset: IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize,
    > SparseMatrix2D for CSR2D<Offset, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type SparseRowColumns<'a>
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

    fn row_sparse_columns(&self, row: Self::RowIndex) -> Self::SparseRowColumns<'_> {
        let start = self.offsets[row.into_usize()].into_usize();
        let end = self.offsets[row.into_usize() + 1].into_usize();
        self.column_indices[start..end].iter().copied()
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.column_indices.iter().copied()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.into()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> usize {
        let start = self.offsets[row.into_usize()].into_usize();
        let end = self.offsets[row.into_usize() + 1].into_usize();
        end - start
    }

    /// Returns the rank for the provided row.
    fn rank(&self, row: RowIndex) -> usize {
        self.offsets[row.into_usize()].into_usize()
    }
}

impl<
        Offset: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize,
    > MatrixMut for CSR2D<Offset, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
{
    type Entry = Self::Coordinates;
    type Error = super::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        if !self.is_empty() && row == self.number_of_rows() - <Self as Matrix2D>::RowIndex::ONE {
            // We check that the provided column is not repeated.
            if self.row_sparse_columns(row).last().is_some_and(|last| last == column) {
                return Err(MutabilityError::DuplicatedEntry((row, column)));
            }
            // We check that the provided column is provided in sorted order.
            if self.row_sparse_columns(row).last().is_some_and(|last| last > column) {
                return Err(MutabilityError::UnorderedColumnIndex(column));
            }
            // If the row is the last row, we can add the entry at the end of the column indices.
            self.column_indices.push(column);
            self.number_of_columns = self.number_of_columns.max(column + ColumnIndex::ONE);
            if let Some(offset) = self.offsets.last_mut() {
                *offset += Offset::ONE;
            } else {
                unreachable!()
            }
            Ok(())
        } else if row >= self.number_of_rows() {
            // If the row is the next row, we can add the entry at the end of the column indices.
            while self.number_of_rows() < row {
                self.offsets.push(self.offsets.last().copied().unwrap_or(Offset::ZERO));
            }
            self.column_indices.push(column);
            self.number_of_columns = self.number_of_columns.max(column + ColumnIndex::ONE);
            self.offsets.push(self.offsets.last().copied().unwrap_or(Offset::ZERO) + Offset::ONE);
            Ok(())
        } else {
            Err(MutabilityError::UnorderedRowIndex(row))
        }
    }
}

impl<
        Offset: PositiveInteger + IntoUsize,
        RowIndex: PositiveInteger + IntoUsize,
        ColumnIndex: PositiveInteger + IntoUsize,
    > TransposableMatrix2D for CSR2D<Offset, RowIndex, ColumnIndex>
where
    Self: Matrix2D<RowIndex = RowIndex, ColumnIndex = ColumnIndex>,
    CSR2D<Offset, ColumnIndex, RowIndex>: Matrix2D<RowIndex = ColumnIndex, ColumnIndex = RowIndex>,
{
    type Transposed = CSR2D<Offset, ColumnIndex, RowIndex>;

    fn transpose(&self) -> Self::Transposed {
        // We initialize the transposed matrix.
        let mut transposed: Self::Transposed = Self::Transposed {
            offsets: vec![Offset::ZERO; self.number_of_columns.into_usize() + 1],
            number_of_columns: self.number_of_rows(),
            column_indices: vec![RowIndex::ZERO; self.number_of_defined_values()],
            _row_indices: PhantomData,
        };

        // First, we proceed to compute the number of elements in each column.
        for column in self.column_indices.iter().copied() {
            transposed.offsets[column.into_usize() + 1] += Offset::ONE;
        }

        // Then, we compute the prefix sum of the degrees to get the offsets.
        let mut prefix_sum = Offset::ZERO;
        for offset in &mut transposed.offsets {
            let current = *offset;
            *offset = prefix_sum;
            prefix_sum += current;
        }

        // Finally, we fill the column indices.
        let mut degree = vec![Offset::ZERO; self.number_of_columns.into_usize()];
        for (row, column) in self.sparse_coordinates() {
            let current_degree: &mut Offset = &mut degree[column.into_usize()];
            let index = *current_degree + transposed.offsets[column.into_usize()];
            transposed.column_indices[index.into_usize()] = row;
            *current_degree += Offset::ONE;
        }

        transposed
    }
}
