//! Submodule providing a definition of a CSR matrix.
use crate::prelude::*;

#[derive(Clone)]
/// A compressed sparse row matrix.
pub struct SquareCSR2D<SparseIndex, Idx> {
    /// The underlying CSR matrix.
    pub(super) csr: CSR2D<SparseIndex, Idx, Idx>,
}

impl<SparseIndex, Idx: IntoUsize + PositiveInteger> SquareMatrix for SquareCSR2D<SparseIndex, Idx>
where
    CSR2D<SparseIndex, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Index = Idx;

    fn order(&self) -> Self::Index {
        self.csr.number_of_rows()
    }
}

impl<SparseIndex, Idx> AsRef<CSR2D<SparseIndex, Idx, Idx>> for SquareCSR2D<SparseIndex, Idx> {
    fn as_ref(&self) -> &CSR2D<SparseIndex, Idx, Idx> {
        &self.csr
    }
}

impl<SparseIndex: Zero, Idx: Zero> Default for SquareCSR2D<SparseIndex, Idx> {
    fn default() -> Self {
        Self { csr: CSR2D::default() }
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize + Zero>
    SparseMatrixMut for SquareCSR2D<SparseIndex, Idx>
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
        Self { csr: CSR2D::with_sparse_capacity((order, order), number_of_values) }
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix
    for SquareCSR2D<SparseIndex, Idx>
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
    for SquareCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<SparseIndex, Idx, Idx>:
        SparseMatrix2D<SparseIndex = SparseIndex, RowIndex = Idx, ColumnIndex = Idx>,
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
    for SquareCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<SparseIndex, Idx, Idx>: MatrixMut<Entry = Self::Coordinates, Error = MutabilityError<CSR2D<SparseIndex, Idx, Idx>>>
        + Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Entry = Self::Coordinates;
    type Error = super::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        Ok(self.csr.add((row, column))?)
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + IntoUsize + From<SparseIndex>,
    > TransposableMatrix2D for SquareCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<SparseIndex, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Transposed = CSR2D<SparseIndex, Idx, Idx>;

    fn transpose(&self) -> Self::Transposed {
        self.csr.transpose()
    }
}
