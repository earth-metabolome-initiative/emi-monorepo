//! Submodule for the [`SquareMatrix`] trait.

use multi_ranged::Step;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger};

use super::{Matrix2D, SparseMatrix2D, SymmetricMatrix2D};

/// Trait defining a square matrix.
pub trait SquareMatrix: Matrix2D<RowIndex = Self::Index, ColumnIndex = Self::Index> {
    /// Type of the index for this matrix.
    type Index: Step + PositiveInteger + IntoUsize;

    /// Returns the order of the matrix.
    fn order(&self) -> Self::Index;
}

impl<M: SquareMatrix> SquareMatrix for &M {
    type Index = M::Index;

    fn order(&self) -> Self::Index {
        (*self).order()
    }
}

/// Trait defining a sparse square matrix.
pub trait SparseSquareMatrix: SquareMatrix + SparseMatrix2D {
    /// Returns the number of defined values in the main diagonal.
    fn number_of_defined_diagonal_values(&self) -> Self::Index;
}

impl<M: SparseSquareMatrix> SparseSquareMatrix for &M {
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        (*self).number_of_defined_diagonal_values()
    }
}

/// Trait defining a matrix that can be symmetrized.
pub trait Symmetrize<M: SymmetricMatrix2D>: SquareMatrix {
    /// Returns the symmetrized version of the matrix.
    fn symmetrize(&self) -> M;
}
