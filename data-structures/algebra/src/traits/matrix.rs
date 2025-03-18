//! Trait defining a matrix.

mod matrix2d;
mod matrix_mut;
mod square_matrix;
mod transposed_valued_matrix2d;
mod triangular_matrix;
mod valued_matrix2d;

pub use matrix2d::*;
pub use matrix_mut::*;
pub use square_matrix::*;
pub use transposed_valued_matrix2d::*;
pub use triangular_matrix::*;
pub use valued_matrix2d::*;

use super::{Coordinates, IntoUsize, PositiveInteger, TotalOrd, Zero};
use crate::impls::ImplicitValuedSparseIteraror;

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

/// Trait defining a matrix with values that can be computed on the fly.
pub trait ImplicitValuedMatrix: ValuedMatrix {
    /// Returns the implicit value at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - Coordinates of the value to get.
    ///
    /// # Returns
    ///
    /// The value at the given coordinates.
    fn implicit_value(&self, coordinates: &Self::Coordinates) -> Self::Value;
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

/// Trait defining a sparse valued matrix.
pub trait SparseValuedMatrix: SparseMatrix + ValuedMatrix {
    /// Iterator over the sparse values of the matrix.
    type SparseValues<'a>: ExactSizeIterator<Item = Self::Value>
        + DoubleEndedIterator<Item = Self::Value>
    where
        Self: 'a;

    /// Returns the largest value in the matrix.
    fn max_value(&self) -> Option<Self::Value>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_values().max_by(TotalOrd::total_cmp)
    }

    /// Returns the smallest value in the matrix.
    fn min_value(&self) -> Option<Self::Value>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_values().min_by(TotalOrd::total_cmp)
    }

    /// Returns an iterator of the sparse coordinates of the matrix.
    fn sparse_values(&self) -> Self::SparseValues<'_>;
}

/// Trait defining a bidimensional matrix.
pub trait ImplicitValuedSparseMatrix: SparseValuedMatrix + ImplicitValuedMatrix {
    /// Iterator over the sparse columns of a row.
    type SparseImplicitValues<'a>: ExactSizeIterator<Item = Self::Value>
        + DoubleEndedIterator<Item = Self::Value>
    where
        Self: 'a;

    /// Returns an iterator over the values of the matrix.
    fn sparse_implicit_values(&self) -> Self::SparseImplicitValues<'_>;
}

impl<M> ImplicitValuedSparseMatrix for M
where
    M: SparseValuedMatrix + ImplicitValuedMatrix,
{
    type SparseImplicitValues<'a>
        = ImplicitValuedSparseIteraror<'a, M>
    where
        M: 'a;

    fn sparse_implicit_values(&self) -> Self::SparseImplicitValues<'_> {
        self.into()
    }
}
