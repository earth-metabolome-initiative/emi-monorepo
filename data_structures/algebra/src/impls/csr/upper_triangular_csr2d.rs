//! Submodule providing a definition of a CSR matrix.
use multi_ranged::Step;
use num_traits::ConstZero;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use crate::prelude::*;

#[derive(Clone, Debug)]
/// A compressed sparse row matrix.
pub struct UpperTriangularCSR2D<M: Matrix2D> {
    /// The underlying matrix.
    matrix: SquareCSR2D<M>,
}

impl<M> Matrix for UpperTriangularCSR2D<M>
where
    M: Matrix2D,
{
    type Coordinates = (M::RowIndex, M::ColumnIndex);

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<M> Matrix2D for UpperTriangularCSR2D<M>
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

impl<M> SquareMatrix for UpperTriangularCSR2D<M>
where
    M: Matrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
{
    type Index = M::RowIndex;

    fn order(&self) -> Self::Index {
        self.matrix.order()
    }
}

impl<M> SparseSquareMatrix for UpperTriangularCSR2D<M>
where
    M: SparseMatrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>,
{
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        self.matrix.number_of_defined_diagonal_values()
    }
}

impl<M> AsRef<M> for UpperTriangularCSR2D<M>
where
    M: Matrix2D,
{
    fn as_ref(&self) -> &M {
        self.matrix.as_ref()
    }
}

impl<M> Default for UpperTriangularCSR2D<M>
where
    M: Matrix2D + Default,
{
    fn default() -> Self {
        Self { matrix: SquareCSR2D::default() }
    }
}

impl<M> SparseMatrixMut for UpperTriangularCSR2D<M>
where
    M: SparseMatrixMut<
            MinimalShape = Self::Coordinates,
            Entry = Self::Coordinates,
            Error = MutabilityError<M>,
        > + SparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type MinimalShape = M::RowIndex;

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self { matrix: SquareCSR2D::with_sparse_capacity(number_of_values) }
    }

    fn with_sparse_shape(shape: Self::MinimalShape) -> Self {
        Self::with_sparse_shaped_capacity(shape, M::SparseIndex::ZERO)
    }

    fn with_sparse_shaped_capacity(
        shape: Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        Self { matrix: SquareCSR2D::with_sparse_shaped_capacity(shape, number_of_values) }
    }
}

impl<M> SparseMatrix for UpperTriangularCSR2D<M>
where
    M: SparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type SparseIndex = <SquareCSR2D<M> as SparseMatrix>::SparseIndex;
    type SparseCoordinates<'a>
        = <SquareCSR2D<M> as SparseMatrix>::SparseCoordinates<'a>
    where
        Self: 'a;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.matrix.sparse_coordinates()
    }

    fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        self.matrix.last_sparse_coordinates()
    }
}

impl<M> SizedSparseMatrix for UpperTriangularCSR2D<M>
where
    M: SizedSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.matrix.number_of_defined_values()
    }
}

impl<M> RankSelectSparseMatrix for UpperTriangularCSR2D<M>
where
    M: SizedSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex> + RankSelectSparseMatrix,
{
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        self.matrix.rank(coordinates)
    }

    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        self.matrix.select(sparse_index)
    }
}

impl<M> SparseMatrix2D for UpperTriangularCSR2D<M>
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

impl<M> EmptyRows for UpperTriangularCSR2D<M>
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

impl<M> SizedRowsSparseMatrix2D for UpperTriangularCSR2D<M>
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

impl<M> SizedSparseMatrix2D for UpperTriangularCSR2D<M>
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

impl<M> MatrixMut for UpperTriangularCSR2D<M>
where
    M: MatrixMut<Entry = Self::Coordinates, Error = MutabilityError<M>>
        + Matrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    type Entry = Self::Coordinates;
    type Error = crate::error::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        if row > column {
            return Err(MutabilityError::OutOfBounds((row, column)));
        }
        self.matrix.add((row, column))?;

        Ok(())
    }

    fn increase_shape(&mut self, shape: Self::Coordinates) -> Result<(), Self::Error> {
        Ok(self.matrix.increase_shape(shape)?)
    }
}

impl<M> TransposableMatrix2D<SquareCSR2D<M>> for UpperTriangularCSR2D<M>
where
    M: TransposableMatrix2D<M, ColumnIndex = <Self as Matrix2D>::RowIndex>,
{
    fn transpose(&self) -> SquareCSR2D<M> {
        self.matrix.transpose()
    }
}

impl<SparseIndex, Idx> Symmetrize<SymmetricCSR2D<CSR2D<SparseIndex, Idx, Idx>>>
    for UpperTriangularCSR2D<CSR2D<SparseIndex, Idx, Idx>>
where
    Idx: Step + PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
{
    fn symmetrize(&self) -> SymmetricCSR2D<CSR2D<SparseIndex, Idx, Idx>> {
        // We initialize the transposed matrix.
        let number_of_expected_column_indices = (self.number_of_defined_values().into_usize()
            - self.number_of_defined_diagonal_values().into_usize())
            * 2
            + self.number_of_defined_diagonal_values().into_usize();

        let mut symmetric: CSR2D<SparseIndex, Idx, Idx> = CSR2D {
            offsets: vec![SparseIndex::ZERO; self.order().into_usize() + 1],
            number_of_columns: self.order(),
            number_of_rows: self.order(),
            column_indices: vec![Idx::ZERO; number_of_expected_column_indices],
            number_of_non_empty_rows: Idx::ZERO,
        };

        // First, we proceed to compute the number of elements in each column.
        for (row, column) in self.sparse_coordinates() {
            // TODO! IF YOU INITIALIZE OFFSETS WITH THE OUT BOUND DEGREES, THERE IS NO NEED
            // FOR ALL OF THE SPARSE ROW ACCESSES!
            symmetric.offsets[row.into_usize() + 1] += SparseIndex::ONE;
            symmetric.offsets[column.into_usize() + 1] += SparseIndex::from(row != column);
        }

        // Then, we compute the prefix sum of the degrees to get the offsets.
        let mut prefix_sum = SparseIndex::ZERO;
        for offset in &mut symmetric.offsets {
            prefix_sum += *offset;
            symmetric.number_of_non_empty_rows +=
                if *offset > SparseIndex::ZERO { Idx::ONE } else { Idx::ZERO };
            *offset = prefix_sum;
        }

        // Finally, we fill the column indices.
        let mut degree = vec![SparseIndex::ZERO; self.order().into_usize()];
        for (row, column) in self.sparse_coordinates() {
            let edges: Vec<(Idx, Idx)> = if row == column {
                vec![(row, column)]
            } else {
                vec![(row, column), (column, row)]
            };
            for (i, j) in edges {
                let current_degree: &mut SparseIndex = &mut degree[i.into_usize()];
                let index = *current_degree + symmetric.offsets[i.into_usize()];
                symmetric.column_indices[index.into_usize()] = j;
                *current_degree += SparseIndex::ONE;
            }
        }

        debug_assert_eq!(
            symmetric.number_of_defined_values().into_usize(),
            number_of_expected_column_indices,
            "The number of inserted values is not the expected one. Original number of values: {}. Diagonals: {}",
            self.number_of_defined_values(),
            self.number_of_defined_diagonal_values()
        );

        SymmetricCSR2D {
            matrix: SquareCSR2D {
                matrix: symmetric,
                number_of_diagonal_values: self.number_of_defined_diagonal_values(),
            },
        }
    }
}
