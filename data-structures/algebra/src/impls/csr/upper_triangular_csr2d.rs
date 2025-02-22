//! Submodule providing a definition of a CSR matrix.
use core::marker::PhantomData;

use crate::prelude::*;

/// A compressed sparse row matrix.
pub struct UpperTriangularCSR2D<Offset, Idx> {
    /// The underlying CSR matrix.
    csr: CSR2D<Offset, Idx, Idx>,
}

impl<Offset, Idx: IntoUsize + PositiveInteger> SquareMatrix for UpperTriangularCSR2D<Offset, Idx>
where
    CSR2D<Offset, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Index = Idx;

    fn order(&self) -> Self::Index {
        self.csr.number_of_rows()
    }
}

impl<Offset, Idx> AsRef<CSR2D<Offset, Idx, Idx>> for UpperTriangularCSR2D<Offset, Idx> {
    fn as_ref(&self) -> &CSR2D<Offset, Idx, Idx> {
        &self.csr
    }
}

impl<Offset: Zero, Idx: Zero> Default for UpperTriangularCSR2D<Offset, Idx> {
    fn default() -> Self {
        Self { csr: CSR2D::default() }
    }
}

impl<Offset: IntoUsize, Idx: PositiveInteger + IntoUsize + Zero> UpperTriangularCSR2D<Offset, Idx> {
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
        number_of_rows: Idx,
        number_of_columns: Idx,
        number_of_values: Offset,
    ) -> Self {
        Self { csr: CSR2D::with_capacity(number_of_rows, number_of_columns, number_of_values) }
    }
}

impl<Offset: IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix
    for UpperTriangularCSR2D<Offset, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<Offset, Idx, Idx>: SparseMatrix<Coordinates = Self::Coordinates>,
{
    type SparseCoordinates<'a>
        = <CSR2D<Offset, Idx, Idx> as SparseMatrix>::SparseCoordinates<'a>
    where
        Self: 'a;

    fn number_of_defined_values(&self) -> usize {
        self.csr.number_of_defined_values()
    }

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.as_ref().sparse_coordinates()
    }
}

impl<Offset: IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix2D
    for UpperTriangularCSR2D<Offset, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<Offset, Idx, Idx>: SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type SparseRow<'a>
        = <CSR2D<Offset, Idx, Idx> as SparseMatrix2D>::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = <CSR2D<Offset, Idx, Idx> as SparseMatrix2D>::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = <CSR2D<Offset, Idx, Idx> as SparseMatrix2D>::SparseRows<'a>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.csr.sparse_row(row)
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.csr.sparse_columns()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.csr.sparse_rows()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> usize {
        self.csr.number_of_defined_values_in_row(row)
    }

    /// Returns the rank for the provided row.
    fn rank(&self, row: Idx) -> usize {
        self.csr.rank(row)
    }
}

impl<Offset: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> MatrixMut
    for UpperTriangularCSR2D<Offset, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<Offset, Idx, Idx>: MatrixMut<Entry = Self::Coordinates, Error = MutabilityError<CSR2D<Offset, Idx, Idx>>>
        + Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Entry = Self::Coordinates;
    type Error = super::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        if row > column {
            return Err(MutabilityError::OutOfBounds((row, column)));
        }
        Ok(self.csr.add((row, column))?)
    }
}

impl<Offset: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> TransposableMatrix2D
    for UpperTriangularCSR2D<Offset, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<Offset, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Transposed = CSR2D<Offset, Idx, Idx>;

    fn transpose(&self) -> Self::Transposed {
        self.csr.transpose()
    }
}

impl<Offset: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize>
    Symmetrize<SquareCSR2D<Offset, Idx>> for UpperTriangularCSR2D<Offset, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<Offset, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn symmetrize(&self) -> SquareCSR2D<Offset, Idx> {
        // We initialize the transposed matrix.
        let mut symmetric: CSR2D<Offset, Idx, Idx> = CSR2D {
            offsets: vec![Offset::ZERO; self.order().into_usize() + 1],
            number_of_columns: self.order(),
            column_indices: vec![Idx::ZERO; self.number_of_defined_values() * 2],
            _row_indices: PhantomData,
        };

        // First, we proceed to compute the number of elements in each column.
        for (row, column) in self.sparse_coordinates() {
            symmetric.offsets[row.into_usize() + 1] += Offset::ONE;
            symmetric.offsets[column.into_usize() + 1] += Offset::ONE;
        }

        // Then, we compute the prefix sum of the degrees to get the offsets.
        let mut prefix_sum = Offset::ZERO;
        for offset in &mut symmetric.offsets {
            let current = *offset;
            *offset = prefix_sum;
            prefix_sum += current;
        }

        // Finally, we fill the column indices.
        let mut degree = vec![Offset::ZERO; self.order().into_usize()];
        for (row, column) in self.sparse_coordinates() {
            for (i, j) in [(row, column), (column, row)] {
                let current_degree: &mut Offset = &mut degree[i.into_usize()];
                let index = *current_degree + symmetric.offsets[i.into_usize()];
                symmetric.column_indices[index.into_usize()] = j;
                *current_degree += Offset::ONE;
            }
        }

        SquareCSR2D { csr: symmetric }
    }
}
