//! Trait defining a matrix.

mod matrix2d;
mod square_matrix;
mod triangular_matrix;
mod matrix_mut;

pub use matrix2d::*;
pub use square_matrix::*;
pub use triangular_matrix::*;
pub use matrix_mut::*;

use super::{Coordinates, IntoUsize, PositiveInteger, Zero};

/// Trait defining a matrix.
pub trait Matrix {
    /// Type of the coordinate for this matrix.
    type Coordinates: Coordinates;

    #[must_use]
    /// Returns the number of dimensions of this matrix.
    fn dimensions() -> usize {
        Self::Coordinates::dimensions()
    }
}

/// Trait defining a matrix with values.
pub trait ValuedMatrix: Matrix {
    /// Type of the values of the matrix.
    type Value;
}

/// Trait defining a sparse matrix.
pub trait SparseMatrix: Matrix {
    /// Type defining the numeric index for the sparse values in the matrix.
    type SparseIndex: PositiveInteger + IntoUsize;

    /// Iterator of the sparse coordinates of the matrix.
    type SparseCoordinates<'a>: ExactSizeIterator<Item = Self::Coordinates>
        + DoubleEndedIterator<Item = Self::Coordinates>
    where
        Self: 'a;

    /// Returns the number of defined elements in the matrix.
    fn number_of_defined_values(&self) -> Self::SparseIndex;

    /// Returns an iterator of the sparse coordinates of the matrix.
    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_>;

    /// Returns whether the matrix is empty.
    fn is_empty(&self) -> bool {
        self.number_of_defined_values() == Self::SparseIndex::ZERO
    }
}
