//! Trait defining a matrix.

mod dense_matrix;
mod dense_matrix2d;
mod matrix2d;
mod matrix_mut;
mod sparse_matrix2d;
mod square_matrix;
mod transposed_valued_matrix2d;
mod triangular_matrix;
mod valued_matrix2d;

use common_traits::prelude::TotalOrd;
pub use dense_matrix::*;
pub use dense_matrix2d::*;
pub use matrix_mut::*;
pub use matrix2d::*;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger};
pub use sparse_matrix2d::*;
pub use square_matrix::*;
pub use transposed_valued_matrix2d::*;
pub use triangular_matrix::*;
pub use valued_matrix2d::*;

use super::Coordinates;
use crate::impls::ImplicitValuedSparseIterator;

/// Trait defining a matrix.
pub trait Matrix {
    /// Type of the coordinate for this matrix.
    type Coordinates: Coordinates;

    #[must_use]
    /// Returns the number of dimensions of this matrix.
    fn dimensions() -> usize {
        Self::Coordinates::dimensions()
    }

    #[must_use]
    /// Returns the shape of the matrix.
    fn shape(&self) -> Vec<usize>;

    #[must_use]
    /// Returns the total number of values in the matrix.
    fn total_values(&self) -> usize {
        self.shape().iter().product()
    }
}

impl<M: Matrix> Matrix for &M {
    type Coordinates = M::Coordinates;

    fn dimensions() -> usize {
        M::dimensions()
    }

    fn shape(&self) -> Vec<usize> {
        (*self).shape()
    }

    fn total_values(&self) -> usize {
        (*self).total_values()
    }
}

/// Trait defining a matrix with values.
pub trait ValuedMatrix: Matrix {
    /// Type of the values of the matrix.
    type Value;
}

impl<M: ValuedMatrix> ValuedMatrix for &M {
    type Value = M::Value;
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
    type SparseCoordinates<'a>: Iterator<Item = Self::Coordinates>
        + DoubleEndedIterator<Item = Self::Coordinates>
    where
        Self: 'a;

    /// Returns an iterator of the sparse coordinates of the matrix.
    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_>;

    /// Returns the last sparse entry in the matrix.
    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates>;

    /// Returns whether the matrix is empty.
    fn is_empty(&self) -> bool;
}

impl<M: SparseMatrix> SparseMatrix for &M {
    type SparseIndex = M::SparseIndex;
    type SparseCoordinates<'a>
        = M::SparseCoordinates<'a>
    where
        Self: 'a;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        (*self).sparse_coordinates()
    }

    fn last_sparse_coordinates(&self) -> Option<Self::Coordinates> {
        (*self).last_sparse_coordinates()
    }

    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}

/// Trait defining a sized sparse matrix.
pub trait SizedSparseMatrix: SparseMatrix {
    #[allow(clippy::cast_precision_loss)]
    /// Returns the density of the matrix.
    fn density(&self) -> f64 {
        let defined_values = self.number_of_defined_values().into_usize();
        let total_values = self.total_values();
        defined_values as f64 / total_values as f64
    }

    /// Returns the number of defined elements in the matrix.
    fn number_of_defined_values(&self) -> Self::SparseIndex;
}

impl<M: SizedSparseMatrix> SizedSparseMatrix for &M {
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        (*self).number_of_defined_values()
    }
}

/// Trait defining a sized sparse matrix.
pub trait RankSelectSparseMatrix: SizedSparseMatrix {
    /// Returns the rank of the provided coordinates.
    ///
    /// # Arguments
    ///
    /// * `coordinates`: The coordinates of the rank to get.
    ///
    /// # Panics
    ///
    /// Panics if the coordinates are out of bounds.
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex;

    /// Returns the coordinates associated to the provided sparse index.
    ///
    /// # Arguments
    ///
    /// * `sparse_index`: The sparse index of the coordinates to get.
    ///
    /// # Panics
    ///
    /// Panics if the sparse index is out of bounds.
    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates;
}

impl<M: RankSelectSparseMatrix> RankSelectSparseMatrix for &M {
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        (*self).rank(coordinates)
    }

    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        (*self).select(sparse_index)
    }
}

/// Trait defining a dense valued matrix.
pub trait DenseValuedMatrix: DenseMatrix + ValuedMatrix {
    /// Iterator over the dense values of the matrix.
    type Values<'a>: Iterator<Item = Self::Value> + DoubleEndedIterator<Item = Self::Value>
    where
        Self: 'a;

    /// Returns the largest value in the matrix.
    fn max_value(&self) -> Option<Self::Value>
    where
        Self::Value: TotalOrd,
    {
        self.values().max_by(TotalOrd::total_cmp)
    }

    /// Returns the smallest value in the matrix.
    fn min_value(&self) -> Option<Self::Value>
    where
        Self::Value: TotalOrd,
    {
        self.values().min_by(TotalOrd::total_cmp)
    }

    /// Returns the value associated to the provided coordinates.
    fn value(&self, coordinates: Self::Coordinates) -> Self::Value;

    /// Returns an iterator of the values of the matrix.
    fn values(&self) -> Self::Values<'_>;
}

impl<M: DenseValuedMatrix> DenseValuedMatrix for &M {
    type Values<'a>
        = M::Values<'a>
    where
        Self: 'a;

    fn value(&self, coordinates: Self::Coordinates) -> Self::Value {
        (*self).value(coordinates)
    }

    fn values(&self) -> Self::Values<'_> {
        (*self).values()
    }
}

/// Trait defining a sparse valued matrix.
pub trait SparseValuedMatrix: SparseMatrix + ValuedMatrix {
    /// Iterator over the sparse values of the matrix.
    type SparseValues<'a>: Iterator<Item = Self::Value> + DoubleEndedIterator<Item = Self::Value>
    where
        Self: 'a;

    /// Returns the largest value in the matrix.
    fn max_sparse_value(&self) -> Option<Self::Value>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_values().max_by(TotalOrd::total_cmp)
    }

    /// Returns the smallest value in the matrix.
    fn min_sparse_value(&self) -> Option<Self::Value>
    where
        Self::Value: TotalOrd,
    {
        self.sparse_values().min_by(TotalOrd::total_cmp)
    }

    /// Returns an iterator of the sparse values of the matrix.
    fn sparse_values(&self) -> Self::SparseValues<'_>;
}

impl<M: SparseValuedMatrix> SparseValuedMatrix for &M {
    type SparseValues<'a>
        = M::SparseValues<'a>
    where
        Self: 'a;

    fn sparse_values(&self) -> Self::SparseValues<'_> {
        (*self).sparse_values()
    }
}

/// Trait defining a sparse valued matrix.
pub trait SizedSparseValuedMatrix: SizedSparseMatrix + SparseValuedMatrix {
    /// Returns the value associated to the provided sparse index.
    ///
    /// # Arguments
    ///
    /// * `sparse_index`: The sparse index of the value to get.
    ///
    /// # Panics
    ///
    /// Panics if the sparse index is out of bounds.
    fn select_value(&self, sparse_index: Self::SparseIndex) -> Self::Value;
}

/// Trait defining a bi-dimensional matrix.
pub trait ImplicitValuedSparseMatrix: SparseValuedMatrix + ImplicitValuedMatrix {
    /// Iterator over the sparse columns of a row.
    type SparseImplicitValues<'a>: Iterator<Item = Self::Value>
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
        = ImplicitValuedSparseIterator<'a, M>
    where
        M: 'a;

    fn sparse_implicit_values(&self) -> Self::SparseImplicitValues<'_> {
        self.into()
    }
}
