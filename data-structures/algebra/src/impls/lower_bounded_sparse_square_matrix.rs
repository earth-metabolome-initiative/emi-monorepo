//! A sliced sparse square matrix, where only rows and columns greater or equal
//! to the provided index are available. All other defined values with
//! coordinates less than the provided index are considered undefined.

use std::fmt::Debug;

use super::MutabilityError;
use crate::traits::SquareMatrix;

mod lower_bounded_sparse_row;
mod matrix;
mod sparse_matrix;

/// A sliced square matrix.
pub struct LowerBoundedSquareMatrix<M: SquareMatrix> {
    /// The matrix to be sliced.
    matrix: M,
    /// The row id of the slice.
    index: M::Index,
}

impl<M: SquareMatrix + Debug> Debug for LowerBoundedSquareMatrix<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LowerBoundedSquareMatrix")
            .field("matrix", &self.matrix)
            .field("index", &self.index)
            .finish()
    }
}

impl<M: SquareMatrix> LowerBoundedSquareMatrix<M> {
    /// Creates a new sliced square matrix.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The matrix to be sliced.
    /// * `index` - The row id of the slice.
    ///
    /// # Errors
    ///
    /// * If the index is greater than or equal to the order of the matrix, an
    ///   `OutOfBounds` error is returned.
    pub fn new(matrix: M, index: M::Index) -> Result<Self, MutabilityError<M>> {
        if index >= matrix.order() {
            return Err(MutabilityError::OutOfBounds((index, index)));
        }
        Ok(Self { matrix, index })
    }
}
