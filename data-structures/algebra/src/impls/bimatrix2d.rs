//! [`BiMatrix`] data structure, which provides a wrapper to a matrix and its transposed version.

use crate::prelude::*;

/// [`BiMatrix`] data structure, which provides a wrapper to a matrix and its transposed version.
pub struct BiMatrix2D<M: TransposableMatrix2D> {
    /// The matrix.
    matrix: M,
    /// The transposed matrix.
    transposed: M::Transposed,
}

impl<M: TransposableMatrix2D> BiMatrix2D<M> {
    /// Creates a new instance of `BiMatrix2D`.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The matrix.
    ///
    pub fn new(matrix: M) -> Self {
        let transposed = matrix.transpose();
        Self { matrix, transposed }
    }
}

impl<M> Matrix2D for BiMatrix2D<M>
where
    M: TransposableMatrix2D,
    M: Matrix2D,
{
    type ColumnIndex = M::ColumnIndex;
    type RowIndex = M::RowIndex;

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_columns()
    }

    fn number_of_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_rows()
    }
}

impl<M> SparseMatrix for BiMatrix2D<M>
where
    M: TransposableMatrix2D,
    M: SparseMatrix,
{
    type SparseCoordinates<'a>
        = M::SparseCoordinates<'a>
    where
        Self: 'a;

    fn number_of_defined_values(&self) -> usize {
        self.matrix.number_of_defined_values()
    }

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.matrix.sparse_coordinates()
    }
}

impl<M> SparseMatrix2D for BiMatrix2D<M>
where
    M: TransposableMatrix2D,
    M: SparseMatrix2D,
{
    type SparseRow<'a>
        = M::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = M::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = M::SparseRows<'a>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.matrix.sparse_row(row)
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.matrix.sparse_columns()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.matrix.sparse_rows()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> usize {
        self.matrix.number_of_defined_values_in_row(row)
    }

    fn rank(&self, row: Self::RowIndex) -> usize {
        self.matrix.rank(row)
    }
}

impl<M> TransposableMatrix2D for BiMatrix2D<M>
where
    M: TransposableMatrix2D,
    M::Transposed: Clone,
{
    /// The type of the transposed matrix.
    type Transposed = M::Transposed;

    fn transpose(&self) -> Self::Transposed {
        self.transposed.clone()
    }
}

impl<M> SparseBiMatrix2D for BiMatrix2D<M>
where
    M: TransposableMatrix2D + SparseMatrix2D,
    M::Transposed: SparseMatrix2D<RowIndex = M::ColumnIndex, ColumnIndex = M::RowIndex>,
{
    type Matrix = M;
    type TransposedMatrix = M::Transposed;

    fn matrix(&self) -> &Self::Matrix {
        &self.matrix
    }

    fn transposed(&self) -> &<Self::Matrix as TransposableMatrix2D>::Transposed {
        &self.transposed
    }
}
