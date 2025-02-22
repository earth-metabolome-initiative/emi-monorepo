//! Submodule providing a definition of a CSR matrix.
use crate::prelude::*;

/// A compressed sparse row matrix.
pub struct SquareCSR2D<Offset, Idx> {
    /// The underlying CSR matrix.
    pub(super) csr: CSR2D<Offset, Idx, Idx>,
}

impl<Offset, Idx: IntoUsize + PositiveInteger> SquareMatrix for SquareCSR2D<Offset, Idx>
where
    CSR2D<Offset, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Index = Idx;

    fn order(&self) -> Self::Index {
        self.csr.number_of_rows()
    }
}

impl<Offset, Idx> AsRef<CSR2D<Offset, Idx, Idx>> for SquareCSR2D<Offset, Idx> {
    fn as_ref(&self) -> &CSR2D<Offset, Idx, Idx> {
        &self.csr
    }
}

impl<Offset: Zero, Idx: Zero> Default for SquareCSR2D<Offset, Idx> {
    fn default() -> Self {
        Self { csr: CSR2D::default() }
    }
}

impl<Offset: IntoUsize, Idx: PositiveInteger + IntoUsize + Zero> SquareCSR2D<Offset, Idx> {
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

impl<Offset: IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix for SquareCSR2D<Offset, Idx>
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
    for SquareCSR2D<Offset, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<Offset, Idx, Idx>: SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type SparseRowColumns<'a>
        = <CSR2D<Offset, Idx, Idx> as SparseMatrix2D>::SparseRowColumns<'a>
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

    fn row_sparse_columns(&self, row: Self::RowIndex) -> Self::SparseRowColumns<'_> {
        self.csr.row_sparse_columns(row)
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
    for SquareCSR2D<Offset, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<Offset, Idx, Idx>: MatrixMut<Entry = Self::Coordinates, Error = MutabilityError<CSR2D<Offset, Idx, Idx>>>
        + Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Entry = Self::Coordinates;
    type Error = super::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        Ok(self.csr.add((row, column))?)
    }
}

impl<Offset: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> TransposableMatrix2D
    for SquareCSR2D<Offset, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<Offset, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Transposed = CSR2D<Offset, Idx, Idx>;

    fn transpose(&self) -> Self::Transposed {
        self.csr.transpose()
    }
}
