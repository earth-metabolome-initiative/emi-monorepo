//! Submodule providing the `LowerBoundedSparseRow` struct and the Iterator
//! trait implementation for it.

use super::LowerBoundedSquareMatrix;
use crate::traits::{SparseMatrix2D, SquareMatrix};

#[derive(Debug)]
/// Iterator of the sparse coordinates of the `LowerBoundedSquareMatrix`.
pub struct LowerBoundedSparseRow<'matrix, M: SquareMatrix + SparseMatrix2D> {
    /// The matrix.
    matrix: &'matrix LowerBoundedSquareMatrix<M>,
    /// The row iterator associated with the index at the beginning of the
    /// iteration.
    row: Option<M::SparseRow<'matrix>>,
}

impl<M: SquareMatrix + SparseMatrix2D> Clone for LowerBoundedSparseRow<'_, M> {
    fn clone(&self) -> Self {
        Self { matrix: self.matrix, row: self.row.clone() }
    }
}

impl<'matrix, M: SquareMatrix + SparseMatrix2D> LowerBoundedSparseRow<'matrix, M> {
    /// Creates a new `LowerBoundedSparseRow` iterator.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The matrix to be iterated.
    /// * `row` - The row index of the matrix.
    pub(super) fn new(matrix: &'matrix LowerBoundedSquareMatrix<M>, row: M::Index) -> Self {
        Self { matrix, row: (row >= matrix.index).then(|| matrix.matrix.sparse_row(row)) }
    }
}

impl<M: SquareMatrix + SparseMatrix2D> Iterator for LowerBoundedSparseRow<'_, M> {
    type Item = M::Index;

    fn next(&mut self) -> Option<Self::Item> {
        self.row.as_mut()?.find(|&column| column >= self.matrix.index)
    }
}

impl<M: SquareMatrix + SparseMatrix2D> DoubleEndedIterator for LowerBoundedSparseRow<'_, M> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let column = self.row.as_mut()?.next_back()?;
        if column >= self.matrix.index {
            Some(column)
        } else {
            // If the column is less than the threshold index, we are done since
            // the row is sorted in ascending order and we are iterating from the
            // back.
            self.row = None;
            None
        }
    }
}
