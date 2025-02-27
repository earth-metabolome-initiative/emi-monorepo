//! Submodule for the [`SquareMatrix`] trait.

use super::{Matrix2D, SparseMatrix2D, SymmetricMatrix2D};
use crate::traits::{IntoUsize, PositiveInteger};

/// Trait defining a square matrix.
pub trait SquareMatrix: Matrix2D<RowIndex = Self::Index, ColumnIndex = Self::Index> {
    /// Type of the index for this matrix.
    type Index: PositiveInteger + IntoUsize;

    /// Returns the order of the matrix.
    fn order(&self) -> Self::Index;
}

/// Trait defining a sparse square matrix.
pub trait SparseSquareMatrix: SquareMatrix + SparseMatrix2D {
    /// Returns the number of defined values in the main diagonal.
    fn number_of_defined_diagonal_values(&self) -> Self::Index;
}

/// Trait defining a matrix that can be symmetrized.
pub trait Symmetrize<M: SymmetricMatrix2D>: SquareMatrix {
    /// Returns the symmetrized version of the matrix.
    fn symmetrize(&self) -> M;
}

impl<M: SquareMatrix> Matrix2D for M {
    type RowIndex = M::Index;
    type ColumnIndex = M::Index;

    fn number_of_rows(&self) -> Self::RowIndex {
        self.order()
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.order()
    }
}
