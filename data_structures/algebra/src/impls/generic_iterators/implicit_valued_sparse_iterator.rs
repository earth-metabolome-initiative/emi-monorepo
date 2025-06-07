//! Submodule providing the Implicit Valued Sparse Iterator.

use crate::traits::ImplicitValuedSparseMatrix;

/// Iterator over the values of a matrix.
pub struct ImplicitValuedSparseIterator<'matrix, M: ImplicitValuedSparseMatrix> {
    iter: M::SparseCoordinates<'matrix>,
    matrix: &'matrix M,
}

impl<'matrix, M: ImplicitValuedSparseMatrix> From<&'matrix M>
    for ImplicitValuedSparseIterator<'matrix, M>
{
    #[inline]
    fn from(matrix: &'matrix M) -> Self {
        let iter = matrix.sparse_coordinates();
        Self { iter, matrix }
    }
}

impl<M: ImplicitValuedSparseMatrix> Iterator for ImplicitValuedSparseIterator<'_, M> {
    type Item = M::Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|coordinates| self.matrix.implicit_value(&coordinates))
    }
}

impl<'matrix, M: ImplicitValuedSparseMatrix> ExactSizeIterator
    for ImplicitValuedSparseIterator<'matrix, M>
where
    M::SparseCoordinates<'matrix>: ExactSizeIterator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<M: ImplicitValuedSparseMatrix> DoubleEndedIterator for ImplicitValuedSparseIterator<'_, M> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|coordinates| self.matrix.implicit_value(&coordinates))
    }
}
