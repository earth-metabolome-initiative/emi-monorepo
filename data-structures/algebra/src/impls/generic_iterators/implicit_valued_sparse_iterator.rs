//! Submodule providing the Implicit Valued Sparse Iterator.

use crate::traits::ImplicitValuedSparseMatrix;

/// Iterator over the values of a matrix.
pub struct ImplicitValuedSparseIteraror<'matrix, M: ImplicitValuedSparseMatrix> {
    iter: M::SparseCoordinates<'matrix>,
    matrix: &'matrix M,
}

impl<'matrix, M: ImplicitValuedSparseMatrix> From<&'matrix M>
    for ImplicitValuedSparseIteraror<'matrix, M>
{
    fn from(matrix: &'matrix M) -> Self {
        let iter = matrix.sparse_coordinates();
        Self { iter, matrix }
    }
}

impl<M: ImplicitValuedSparseMatrix> Iterator for ImplicitValuedSparseIteraror<'_, M> {
    type Item = M::Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|coordinates| self.matrix.implicit_value(&coordinates))
    }
}

impl<M: ImplicitValuedSparseMatrix> ExactSizeIterator
    for ImplicitValuedSparseIteraror<'_, M>
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<M: ImplicitValuedSparseMatrix> DoubleEndedIterator
    for ImplicitValuedSparseIteraror<'_, M>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|coordinates| self.matrix.implicit_value(&coordinates))
    }
}
