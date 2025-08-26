//! Submodule providing a definition of a CSR matrix.
use core::fmt::Debug;

use numeric_common_traits::prelude::IntoUsize;

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
/// A symmetric compressed sparse row matrix.
pub struct SymmetricCSR2D<M: Matrix2D> {
    /// The underlying matrix.
    pub(super) matrix: SquareCSR2D<M>,
}

impl<M: Matrix2D + Debug> Debug for SymmetricCSR2D<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SymmetricCSR2D").field("matrix", &self.matrix).finish()
    }
}

impl<M> Matrix for SymmetricCSR2D<M>
where
    M: Matrix2D,
{
    type Coordinates = (M::RowIndex, M::ColumnIndex);

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<M> Matrix2D for SymmetricCSR2D<M>
where
    M: Matrix2D,
{
    type RowIndex = M::RowIndex;
    type ColumnIndex = M::ColumnIndex;

    fn number_of_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_rows()
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_columns()
    }
}

impl<M> SquareMatrix for SymmetricCSR2D<M>
where
    M: Matrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
{
    type Index = M::RowIndex;

    fn order(&self) -> Self::Index {
        self.matrix.order()
    }
}

impl<M> SparseSquareMatrix for SymmetricCSR2D<M>
where
    M: SparseMatrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
{
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        self.matrix.number_of_defined_diagonal_values()
    }
}

impl<M> AsRef<SquareCSR2D<M>> for SymmetricCSR2D<M>
where
    M: Matrix2D,
{
    fn as_ref(&self) -> &SquareCSR2D<M> {
        &self.matrix
    }
}

impl<M> Default for SymmetricCSR2D<M>
where
    M: Matrix2D + Default,
{
    fn default() -> Self {
        Self { matrix: SquareCSR2D::default() }
    }
}

impl<M> SparseMatrix for SymmetricCSR2D<M>
where
    M: SparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type SparseIndex = M::SparseIndex;
    type SparseCoordinates<'a>
        = <SquareCSR2D<M> as SparseMatrix>::SparseCoordinates<'a>
    where
        Self: 'a;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.as_ref().sparse_coordinates()
    }

    fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        self.matrix.last_sparse_coordinates()
    }
}

impl<M> SizedSparseMatrix for SymmetricCSR2D<M>
where
    M: SizedSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.matrix.number_of_defined_values()
    }
}

impl<M> RankSelectSparseMatrix for SymmetricCSR2D<M>
where
    SquareCSR2D<M>:
        RankSelectSparseMatrix<Coordinates = Self::Coordinates, SparseIndex = Self::SparseIndex>,
    M: SizedSparseMatrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
{
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        self.matrix.rank(coordinates)
    }

    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        self.matrix.select(sparse_index)
    }
}

impl<M> SparseMatrix2D for SymmetricCSR2D<M>
where
    M: SparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type SparseRow<'a>
        = <SquareCSR2D<M> as SparseMatrix2D>::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = <SquareCSR2D<M> as SparseMatrix2D>::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = <SquareCSR2D<M> as SparseMatrix2D>::SparseRows<'a>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.matrix.sparse_row(row)
    }

    #[inline]
    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        self.matrix.has_entry(row, column)
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.matrix.sparse_columns()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.matrix.sparse_rows()
    }
}

impl<M> EmptyRows for SymmetricCSR2D<M>
where
    M: EmptyRows<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type EmptyRowIndices<'a>
        = <SquareCSR2D<M> as EmptyRows>::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = <SquareCSR2D<M> as EmptyRows>::NonEmptyRowIndices<'a>
    where
        Self: 'a;

    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.matrix.empty_row_indices()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.matrix.non_empty_row_indices()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_empty_rows()
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_non_empty_rows()
    }
}

impl<M> SizedRowsSparseMatrix2D for SymmetricCSR2D<M>
where
    M: SizedSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type SparseRowSizes<'a>
        = <SquareCSR2D<M> as SizedRowsSparseMatrix2D>::SparseRowSizes<'a>
    where
        Self: 'a;
    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.matrix.number_of_defined_values_in_row(row)
    }

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.matrix.sparse_row_sizes()
    }
}

impl<M> SizedSparseMatrix2D for SymmetricCSR2D<M>
where
    M: SizedSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    fn rank_row(&self, row: M::RowIndex) -> Self::SparseIndex {
        self.matrix.rank_row(row)
    }

    fn select_column(&self, sparse_index: Self::SparseIndex) -> Self::ColumnIndex {
        self.matrix.select_column(sparse_index)
    }

    fn select_row(&self, sparse_index: Self::SparseIndex) -> Self::RowIndex {
        self.matrix.select_row(sparse_index)
    }
}

impl<M> TransposableMatrix2D<Self> for SymmetricCSR2D<M>
where
    M: Matrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex> + Clone,
{
    fn transpose(&self) -> Self {
        self.clone()
    }
}

impl<M> Symmetrize<Self> for SymmetricCSR2D<M>
where
    M: Matrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex> + Clone,
{
    fn symmetrize(&self) -> Self {
        self.clone()
    }
}

impl<M> BiMatrix2D for SymmetricCSR2D<M>
where
    M: Matrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type Matrix = SquareCSR2D<M>;
    type TransposedMatrix = SquareCSR2D<M>;

    fn matrix(&self) -> &Self::Matrix {
        &self.matrix
    }

    fn transposed(&self) -> &Self::TransposedMatrix {
        &self.matrix
    }
}
