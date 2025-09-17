//! Submodule providing a definition of a CSR matrix.
use core::fmt::Debug;

use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger};

#[cfg(feature = "arbitrary")]
mod arbitrary_impl;

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
/// A compressed sparse row matrix.
pub struct SquareCSR2D<M: Matrix2D> {
    /// The underlying matrix.
    pub(super) matrix: M,
    /// The number of values in the diagonal.
    pub(super) number_of_diagonal_values: M::RowIndex,
}

impl<M: Matrix2D + Debug> Debug for SquareCSR2D<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SquareCSR2D")
            .field("matrix", &self.matrix)
            .field("number_of_diagonal_values", &self.number_of_diagonal_values)
            .finish()
    }
}

impl<M: Matrix2D> Matrix for SquareCSR2D<M>
where
    M: Matrix2D,
    M::RowIndex: IntoUsize + PositiveInteger,
    M::ColumnIndex: IntoUsize + PositiveInteger,
{
    type Coordinates = (M::RowIndex, M::ColumnIndex);

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<M> Matrix2D for SquareCSR2D<M>
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

impl<M> SquareMatrix for SquareCSR2D<M>
where
    M: Matrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
{
    type Index = M::RowIndex;

    fn order(&self) -> Self::Index {
        debug_assert_eq!(
            self.matrix.number_of_columns(),
            self.matrix.number_of_rows(),
            "The matrix is not square."
        );
        self.matrix.number_of_rows()
    }
}

impl<M> SparseSquareMatrix for SquareCSR2D<M>
where
    M: SparseMatrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
{
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        self.number_of_diagonal_values
    }
}

impl<M: Matrix2D> AsRef<M> for SquareCSR2D<M> {
    fn as_ref(&self) -> &M {
        &self.matrix
    }
}

impl<M: Default + Matrix2D> Default for SquareCSR2D<M> {
    fn default() -> Self {
        Self { matrix: M::default(), number_of_diagonal_values: M::RowIndex::ZERO }
    }
}

impl<M> SparseMatrixMut for SquareCSR2D<M>
where
    M: SparseMatrixMut<
            MinimalShape = Self::Coordinates,
            Entry = Self::Coordinates,
            Error = MutabilityError<M>,
        > + Matrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type MinimalShape = M::RowIndex;

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self {
            matrix: M::with_sparse_capacity(number_of_values),
            number_of_diagonal_values: M::RowIndex::ZERO,
        }
    }

    fn with_sparse_shape(order: Self::MinimalShape) -> Self {
        Self::with_sparse_shaped_capacity(order, M::SparseIndex::ZERO)
    }

    fn with_sparse_shaped_capacity(
        order: Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        Self {
            matrix: M::with_sparse_shaped_capacity((order, order), number_of_values),
            number_of_diagonal_values: M::RowIndex::ZERO,
        }
    }
}

impl<M> SparseMatrix for SquareCSR2D<M>
where
    M: Matrix2D + SparseMatrix<Coordinates = Self::Coordinates>,
{
    type SparseIndex = <M as SparseMatrix>::SparseIndex;
    type SparseCoordinates<'a>
        = <M as SparseMatrix>::SparseCoordinates<'a>
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

impl<M> SizedSparseMatrix for SquareCSR2D<M>
where
    M: SizedSparseMatrix<Coordinates = Self::Coordinates> + Matrix2D,
{
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.matrix.number_of_defined_values()
    }
}

impl<M> RankSelectSparseMatrix for SquareCSR2D<M>
where
    M: RankSelectSparseMatrix<Coordinates = Self::Coordinates> + Matrix2D,
{
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        self.matrix.rank(coordinates)
    }

    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        self.matrix.select(sparse_index)
    }
}

impl<M> SparseMatrix2D for SquareCSR2D<M>
where
    M: SparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type SparseRow<'a>
        = <M as SparseMatrix2D>::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = <M as SparseMatrix2D>::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = <M as SparseMatrix2D>::SparseRows<'a>
    where
        Self: 'a;

    #[inline]
    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.matrix.sparse_row(row)
    }

    #[inline]
    fn has_entry(&self, row: Self::RowIndex, column: Self::ColumnIndex) -> bool {
        self.matrix.has_entry(row, column)
    }

    #[inline]
    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.matrix.sparse_columns()
    }

    #[inline]
    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.matrix.sparse_rows()
    }
}

impl<M> EmptyRows for SquareCSR2D<M>
where
    M: EmptyRows<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type EmptyRowIndices<'a>
        = <M as EmptyRows>::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = <M as EmptyRows>::NonEmptyRowIndices<'a>
    where
        Self: 'a;
    #[inline]
    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.matrix.empty_row_indices()
    }

    #[inline]
    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.matrix.non_empty_row_indices()
    }

    #[inline]
    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_empty_rows()
    }

    #[inline]
    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_non_empty_rows()
    }
}

impl<M> SizedRowsSparseMatrix2D for SquareCSR2D<M>
where
    M: SizedRowsSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type SparseRowSizes<'a>
        = <M as SizedRowsSparseMatrix2D>::SparseRowSizes<'a>
    where
        Self: 'a;
    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        debug_assert_eq!(
            self.number_of_rows(),
            self.number_of_columns(),
            "The matrix is not square."
        );
        self.matrix.number_of_defined_values_in_row(row)
    }

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.matrix.sparse_row_sizes()
    }
}

impl<M> SizedSparseMatrix2D for SquareCSR2D<M>
where
    M: SizedSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    fn rank_row(&self, row: M::RowIndex) -> Self::SparseIndex {
        self.matrix.rank_row(row)
    }

    fn select_row(&self, sparse_index: Self::SparseIndex) -> Self::RowIndex {
        self.matrix.select_row(sparse_index)
    }

    fn select_column(&self, sparse_index: Self::SparseIndex) -> Self::ColumnIndex {
        self.matrix.select_column(sparse_index)
    }
}

impl<M> MatrixMut for SquareCSR2D<M>
where
    M: MatrixMut<Entry = Self::Coordinates, Error = MutabilityError<M>>
        + Matrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type Entry = Self::Coordinates;
    type Error = crate::error::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        self.matrix.add((row, column))?;
        // Since the matrix is square, the number of columns is equal to the number of
        // row, and if the user happens to provide a row that is greater than
        // the number of columns, we need to update the number of columns so as
        // to keep the matrix square.
        let side =
            self.matrix.number_of_rows().max(row + M::RowIndex::ONE).max(column + M::RowIndex::ONE);
        self.matrix.increase_shape((side, side))?;

        debug_assert!(
            row < self.number_of_rows(),
            "The matrix is in an illegal state where the row index {row} is greater than the number of rows {}.",
            self.number_of_rows()
        );
        debug_assert!(
            column < self.number_of_columns(),
            "The matrix is in an illegal state where the column index {column} is greater than the number of columns {}.",
            self.number_of_columns()
        );
        debug_assert_eq!(
            self.number_of_rows(),
            self.number_of_columns(),
            "The matrix is not square."
        );

        self.number_of_diagonal_values += M::RowIndex::from(row == column);

        Ok(())
    }

    fn increase_shape(
        &mut self,
        (number_of_rows, number_of_columns): Self::Coordinates,
    ) -> Result<(), Self::Error> {
        if number_of_rows != number_of_columns {
            return Err(MutabilityError::IncompatibleShape);
        }
        self.matrix.increase_shape((number_of_rows, number_of_columns))?;
        Ok(())
    }
}

impl<M> TransposableMatrix2D<Self> for SquareCSR2D<M>
where
    M: TransposableMatrix2D<M, ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    fn transpose(&self) -> Self {
        Self {
            matrix: self.matrix.transpose(),
            number_of_diagonal_values: self.number_of_diagonal_values,
        }
    }
}
