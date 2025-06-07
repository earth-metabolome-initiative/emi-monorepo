//! A subset sparse square matrix, where only the existing coordinates that have
//! both row and column ids are within a provided subset.

use num_traits::ConstZero;

use super::MutabilityError;
use crate::traits::SquareMatrix;

mod matrix;
mod sparse_matrix;

/// A sliced sparse square matrix, where only the existing coordinates that have
/// both row and column ids are within a provided subset.
pub struct SubsetSquareMatrix<M: SquareMatrix, I> {
    /// The matrix to be sliced.
    matrix: M,
    /// The indices of the subset.
    indices: I,
}

impl<M: SquareMatrix + Default, I> Default for SubsetSquareMatrix<M, I>
where
    I: Default,
{
    fn default() -> Self {
        Self { matrix: M::default(), indices: I::default() }
    }
}

impl<M: SquareMatrix, I> SubsetSquareMatrix<M, I> {
    /// Creates a new subset sparse square matrix from unsorted indices.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The matrix to be sliced.
    /// * `unsorted_indices` - The indices of the subset.
    ///
    /// # Errors
    ///
    /// * `MutabilityError::OutOfBounds` - If any of the indices are out of
    ///   bounds.
    pub fn with_unsorted_indices(
        matrix: M,
        unsorted_indices: I,
    ) -> Result<SubsetSquareMatrix<M, Vec<M::Index>>, MutabilityError<M>>
    where
        I: Iterator<Item = M::Index> + ExactSizeIterator,
    {
        let mut sorted_indices: Vec<M::Index> = vec![M::Index::ZERO; unsorted_indices.len()];
        for (unsorted_index, sorted_index) in unsorted_indices.zip(sorted_indices.iter_mut()) {
            if unsorted_index >= matrix.order() {
                return Err(MutabilityError::OutOfBounds((unsorted_index, unsorted_index)));
            }
            *sorted_index = unsorted_index;
        }
        sorted_indices.sort_unstable();
        Ok(SubsetSquareMatrix { matrix, indices: sorted_indices })
    }

    /// Creates a new subset sparse square matrix from sorted indices.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The matrix to be sliced.
    /// * `sorted_indices` - The indices of the subset.
    ///
    /// # Panics in debug mode
    ///
    /// * `MutabilityError::UnorderedCoordinate` - If any of the indices are out
    ///   of order.
    /// * `MutabilityError::OutOfBounds` - If any of the indices are out of
    ///   bounds.
    pub fn with_sorted_indices(matrix: M, sorted_indices: I) -> SubsetSquareMatrix<M, I>
    where
        I: AsRef<[M::Index]>,
    {
        debug_assert!(
            sorted_indices.as_ref().is_sorted(),
            "The indices are not sorted. This is a bug in the code."
        );
        debug_assert!(
            sorted_indices.as_ref().iter().all(|&index| index < matrix.order()),
            "The indices are out of bounds. This is a bug in the code."
        );
        SubsetSquareMatrix { matrix, indices: sorted_indices }
    }
}
