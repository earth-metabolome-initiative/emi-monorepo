//! Submodule providing a definition of a CSR matrix.
use core::fmt::Debug;

use crate::prelude::*;

#[derive(Clone)]
/// A compressed sparse row matrix.
pub struct SymmetricCSR2D<SparseIndex, Idx> {
    /// The underlying CSR matrix.
    pub(super) csr: SquareCSR2D<SparseIndex, Idx>,
}

impl<SparseIndex: Debug, Idx: Debug> Debug for SymmetricCSR2D<SparseIndex, Idx> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SymmetricCSR2D").field("csr", &self.csr).finish()
    }
}

impl<SparseIndex, Idx: IntoUsize + PositiveInteger> SquareMatrix
    for SymmetricCSR2D<SparseIndex, Idx>
where
    SquareCSR2D<SparseIndex, Idx>: SquareMatrix<Index = Idx>,
{
    type Index = Idx;

    fn order(&self) -> Self::Index {
        self.csr.order()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: IntoUsize + PositiveInteger> SparseSquareMatrix
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: SquareMatrix<Index = Idx>,
    CSR2D<SparseIndex, Idx, Idx>:
        SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        self.csr.number_of_defined_diagonal_values()
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

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self { csr: SquareCSR2D::with_sparse_capacity(number_of_values) }
    }

    fn with_sparse_shaped_capacity(
        order: Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        Self { csr: SquareCSR2D::with_sparse_shaped_capacity(order, number_of_values) }
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
    type SparseRowSizes<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::SparseRowSizes<'a>
    where
        Self: 'a;
    type EmptyRowIndices<'a> = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::EmptyRowIndices<'a> where Self: 'a;
    type NonEmptyRowIndices<'a> = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::NonEmptyRowIndices<'a> where Self: 'a;

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

    fn rank(&self, row: Idx) -> Self::SparseIndex {
        self.csr.rank(row)
    }

    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.csr.empty_row_indices()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.csr.non_empty_row_indices()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.csr.number_of_empty_rows()
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.csr.number_of_non_empty_rows()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize>
    TransposableMatrix2D<Self> for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    fn transpose(&self) -> Self {
        self.clone()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> Symmetrize<Self>
    for SymmetricCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: SquareMatrix<Index = Idx>,
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
