//! Submodule providing a definition of a CSR matrix.
use core::marker::PhantomData;

use crate::prelude::*;

#[derive(Clone)]
/// A compressed sparse row matrix.
pub struct UpperTriangularCSR2D<SparseIndex, Idx> {
    /// The underlying CSR matrix.
    csr: CSR2D<SparseIndex, Idx, Idx>,
    /// The number of values in the diagonal.
    number_of_diagonal_values: Idx,
}

impl<SparseIndex, Idx: IntoUsize + PositiveInteger> SquareMatrix
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    CSR2D<SparseIndex, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Index = Idx;

    fn order(&self) -> Self::Index {
        self.csr.number_of_rows()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: IntoUsize + PositiveInteger> SparseSquareMatrix
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: SquareMatrix<Index = Idx>,
    CSR2D<SparseIndex, Idx, Idx>:
        SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        self.number_of_diagonal_values
    }
}

impl<SparseIndex, Idx> AsRef<CSR2D<SparseIndex, Idx, Idx>>
    for UpperTriangularCSR2D<SparseIndex, Idx>
{
    fn as_ref(&self) -> &CSR2D<SparseIndex, Idx, Idx> {
        &self.csr
    }
}

impl<SparseIndex: Zero, Idx: Zero> Default for UpperTriangularCSR2D<SparseIndex, Idx> {
    fn default() -> Self {
        Self { csr: CSR2D::default(), number_of_diagonal_values: Idx::ZERO }
    }
}

impl<SparseIndex: IntoUsize, Idx: PositiveInteger + IntoUsize + Zero> SparseMatrixMut
    for UpperTriangularCSR2D<SparseIndex, Idx>
where 
    Self: SparseMatrix<SparseIndex = SparseIndex, Coordinates = (Idx, Idx)>
        + MatrixMut<Entry = Self::Coordinates>,
    CSR2D<SparseIndex, Idx, Idx>:
        SparseMatrixMut<SparseIndex = SparseIndex, MinimalShape = Self::Coordinates>,
{
    type MinimalShape = Idx;

    /// Creates a new CSR matrix with the provided number of rows and columns.
    ///
    /// # Arguments
    ///
    /// * `order`: The number of rows and columns.
    /// * `number_of_values`: The number of values.
    ///
    /// # Returns
    ///
    /// A new CSR matrix with the provided number of rows and columns.
    fn with_sparse_capacity(order: Idx, number_of_values: SparseIndex) -> Self {
        Self {
            csr: CSR2D::with_sparse_capacity((order, order), number_of_values),
            number_of_diagonal_values: Idx::ZERO,
        }
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<SparseIndex, Idx, Idx>:
        SparseMatrix<Coordinates = Self::Coordinates, SparseIndex = SparseIndex>,
{
    type SparseIndex = <CSR2D<SparseIndex, Idx, Idx> as SparseMatrix>::SparseIndex;
    type SparseCoordinates<'a>
        = <CSR2D<SparseIndex, Idx, Idx> as SparseMatrix>::SparseCoordinates<'a>
    where
        Self: 'a;

    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.csr.number_of_defined_values()
    }

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.as_ref().sparse_coordinates()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix2D
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<SparseIndex, Idx, Idx>:
        SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    type SparseRow<'a>
        = <CSR2D<SparseIndex, Idx, Idx> as SparseMatrix2D>::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = <CSR2D<SparseIndex, Idx, Idx> as SparseMatrix2D>::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = <CSR2D<SparseIndex, Idx, Idx> as SparseMatrix2D>::SparseRows<'a>
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

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.csr.number_of_defined_values_in_row(row)
    }

    /// Returns the rank for the provided row.
    fn rank(&self, row: Idx) -> usize {
        self.csr.rank(row)
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> MatrixMut
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<SparseIndex, Idx, Idx>: MatrixMut<Entry = Self::Coordinates, Error = MutabilityError<CSR2D<SparseIndex, Idx, Idx>>>
        + Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Entry = Self::Coordinates;
    type Error = super::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        if row > column {
            return Err(MutabilityError::OutOfBounds((row, column)));
        }
        self.csr.add((row, column))?;
        self.number_of_diagonal_values += Idx::ONE;

        Ok(())
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + IntoUsize + From<SparseIndex>,
    > TransposableMatrix2D for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<SparseIndex, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Transposed = CSR2D<SparseIndex, Idx, Idx>;

    fn transpose(&self) -> Self::Transposed {
        self.csr.transpose()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize>
    Symmetrize<SymmetricCSR2D<SparseIndex, Idx>> for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: SparseSquareMatrix<Index = Idx, SparseIndex = SparseIndex>,
    CSR2D<SparseIndex, Idx, Idx>:
        SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    fn symmetrize(&self) -> SymmetricCSR2D<SparseIndex, Idx> {
        // We initialize the transposed matrix.
        let mut symmetric: CSR2D<SparseIndex, Idx, Idx> = CSR2D {
            offsets: vec![SparseIndex::ZERO; self.order().into_usize() + 1],
            number_of_columns: self.order(),
            column_indices: vec![
                Idx::ZERO;
                (self.number_of_defined_values().into_usize()
                    - self.number_of_defined_diagonal_values().into_usize())
                    * 2
                    + self.number_of_defined_diagonal_values().into_usize()
            ],
            _row_indices: PhantomData,
        };

        // First, we proceed to compute the number of elements in each column.
        for (row, column) in self.sparse_coordinates() {
            symmetric.offsets[row.into_usize() + 1] += SparseIndex::ONE;
            symmetric.offsets[column.into_usize() + 1] += SparseIndex::ONE;
        }

        // Then, we compute the prefix sum of the degrees to get the offsets.
        let mut prefix_sum = SparseIndex::ZERO;
        for offset in &mut symmetric.offsets {
            let current = *offset;
            *offset = prefix_sum;
            prefix_sum += current;
        }

        // Finally, we fill the column indices.
        let mut degree = vec![SparseIndex::ZERO; self.order().into_usize()];
        for (row, column) in self.sparse_coordinates() {
            for (i, j) in [(row, column), (column, row)] {
                let current_degree: &mut SparseIndex = &mut degree[i.into_usize()];
                let index = *current_degree + symmetric.offsets[i.into_usize()];
                symmetric.column_indices[index.into_usize()] = j;
                *current_degree += SparseIndex::ONE;
            }
        }

        SymmetricCSR2D { csr: SquareCSR2D { csr: symmetric } }
    }
}
