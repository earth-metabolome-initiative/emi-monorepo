//! Trait defining a matrix.

mod matrix2d;
mod matrix_mut;
mod square_matrix;
mod triangular_matrix;
mod valued_matrix2d;

pub use matrix2d::*;
pub use matrix_mut::*;
pub use square_matrix::*;
pub use triangular_matrix::*;
pub use valued_matrix2d::*;

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

/// Trait defining a matrix with values that can be computed on the fly.
pub trait ImplicitValuedMatrix: Matrix {
    /// Type of the values of the matrix.
    type Value;

    /// Returns the value at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - Coordinates of the value to get.
    ///
    /// # Returns
    ///
    /// The value at the given coordinates.
    fn value(&self, coordinates: &Self::Coordinates) -> Self::Value;
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
