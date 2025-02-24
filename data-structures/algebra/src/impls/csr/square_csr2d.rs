//! Submodule providing a definition of a CSR matrix.
use core::fmt::Debug;
use crate::prelude::*;

#[derive(Clone)]
/// A compressed sparse row matrix.
pub struct SquareCSR2D<SparseIndex, Idx> {
    /// The underlying CSR matrix.
    pub(super) csr: CSR2D<SparseIndex, Idx, Idx>,
    /// The number of values in the diagonal.
    pub(super) number_of_diagonal_values: Idx,
}

impl<SparseIndex: Debug, Idx: Debug> Debug for SquareCSR2D<SparseIndex, Idx> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SquareCSR2D")
            .field("csr", &self.csr)
            .field("number_of_diagonal_values", &self.number_of_diagonal_values)
            .finish()
    }
}

impl<SparseIndex, Idx: IntoUsize + PositiveInteger> SquareMatrix for SquareCSR2D<SparseIndex, Idx>
where
    CSR2D<SparseIndex, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Index = Idx;

    fn order(&self) -> Self::Index {
        debug_assert_eq!(self.csr.number_of_columns(), self.csr.number_of_rows(), "The matrix is not square.");
        self.csr.number_of_rows()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: IntoUsize + PositiveInteger> SparseSquareMatrix
    for SquareCSR2D<SparseIndex, Idx>
where
    Self: SquareMatrix<Index = Idx>,
    CSR2D<SparseIndex, Idx, Idx>:
        SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        self.number_of_diagonal_values
    }
}

impl<SparseIndex, Idx> AsRef<CSR2D<SparseIndex, Idx, Idx>> for SquareCSR2D<SparseIndex, Idx> {
    fn as_ref(&self) -> &CSR2D<SparseIndex, Idx, Idx> {
        &self.csr
    }
}

impl<SparseIndex: Zero, Idx: Zero> Default for SquareCSR2D<SparseIndex, Idx> {
    fn default() -> Self {
        Self { csr: CSR2D::default(), number_of_diagonal_values: Idx::ZERO }
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

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self {
            csr: CSR2D::with_sparse_capacity(number_of_values),
            number_of_diagonal_values: Idx::ZERO,
        }
    }

    fn with_sparse_shaped_capacity(
        order: Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        Self {
            csr: CSR2D::with_sparse_shaped_capacity((order, order), number_of_values),
            number_of_diagonal_values: Idx::ZERO,
        }
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
    type SparseRowSizes<'a> = <CSR2D<SparseIndex, Idx, Idx> as SparseMatrix2D>::SparseRowSizes<'a>
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

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.csr.sparse_row_sizes()
    }

    /// Returns the rank for the provided row.
    fn rank(&self, row: Idx) -> Self::SparseIndex {
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
        self.csr.add((row, column))?;
        // Since the matrix is square, the number of columns is equal to the number of row,
        // and if the user happens to provide a row that is greater than the number of columns,
        // we need to update the number of columns so as to keep the matrix square.
        self.csr.number_of_columns = self.csr.number_of_columns.max(row + Idx::ONE);
        self.csr.number_of_rows = self.csr.number_of_rows.max(column + Idx::ONE);
        self.number_of_diagonal_values += Idx::from(row == column);

        Ok(())
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    > TransposableMatrix2D for SquareCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    CSR2D<SparseIndex, Idx, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Transposed = Self;

    fn transpose(&self) -> Self::Transposed {
        Self {
            csr: self.csr.transpose(),
            number_of_diagonal_values: self.number_of_diagonal_values,
        }
    }
}
