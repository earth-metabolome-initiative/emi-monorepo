//! Submodule providing a definition of a CSR matrix.
use crate::prelude::*;

#[derive(Clone)]
/// A compressed sparse row matrix.
pub struct SymmetricCSR2D<SparseIndex, Idx> {
    /// The underlying CSR matrix.
    pub(super) csr: SquareCSR2D<SparseIndex, Idx>,
}

impl<SparseIndex, Idx: IntoUsize + PositiveInteger> SquareMatrix
    for SymmetricCSR2D<SparseIndex, Idx>
where
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Index = Idx;

    fn order(&self) -> Self::Index {
        self.csr.number_of_rows()
    }
}

impl<SparseIndex, Idx> AsRef<SquareCSR2D<SparseIndex, Idx>> for SymmetricCSR2D<SparseIndex, Idx> {
    fn as_ref(&self) -> &SquareCSR2D<SparseIndex, Idx> {
        &self.csr
    }
}

impl<SparseIndex: Zero, Idx: Zero> Default for SymmetricCSR2D<SparseIndex, Idx> {
    fn default() -> Self {
        Self { csr: SquareCSR2D::default() }
    }
}

impl<SparseIndex: IntoUsize, Idx: PositiveInteger + IntoUsize + Zero> SparseMatrixMut
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: SparseMatrix<SparseIndex = SparseIndex, Coordinates = (Idx, Idx)>
        + MatrixMut<Entry = Self::Coordinates>,
    SquareCSR2D<SparseIndex, Idx>: SparseMatrixMut<SparseIndex = SparseIndex, MinimalShape = Idx>,
{
    type MinimalShape = Idx;

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
    fn with_sparse_capacity(order: Idx, number_of_values: SparseIndex) -> Self {
        Self { csr: SquareCSR2D::with_sparse_capacity(order, number_of_values) }
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        SparseMatrix<Coordinates = Self::Coordinates, SparseIndex = SparseIndex>,
{
    type SparseIndex = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix>::SparseIndex;
    type SparseCoordinates<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix>::SparseCoordinates<'a>
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
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    type SparseRow<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::SparseRows<'a>
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

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize>
    TransposableMatrix2D for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Transposed = Self;

    fn transpose(&self) -> Self::Transposed {
        self.clone()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> Symmetrize<Self>
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn symmetrize(&self) -> Self {
        self.clone()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> BiMatrix2D
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Matrix = SquareCSR2D<SparseIndex, Idx>;
    type TransposedMatrix = SquareCSR2D<SparseIndex, Idx>;

    fn matrix(&self) -> &Self::Matrix {
        &self.csr
    }

    fn transposed(&self) -> &Self::TransposedMatrix {
        &self.csr
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SparseBiMatrix2D
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type SparseMatrix = SquareCSR2D<SparseIndex, Idx>;
    type SparseTransposedMatrix = SquareCSR2D<SparseIndex, Idx>;
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SymmetricMatrix2D
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type SymmetricMatrix = SquareCSR2D<SparseIndex, Idx>;
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize>
    SparseSymmetricMatrix2D for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type SymmetricSparseMatrix = SquareCSR2D<SparseIndex, Idx>;
}
