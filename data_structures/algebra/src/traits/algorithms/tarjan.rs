//! Submodule providing the `Tarjan` trait and its blanket implementation for
//! sparse matrices, which provides the Tarjan's algorithm for strongly
//! connected components.

use multi_ranged::SimpleRange;
use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::IntoUsize;

use crate::traits::{SparseMatrix2D, SquareMatrix};

/// Iterator over the strongly connected components of a sparse matrix.
pub struct TarjanIterator<'matrix, M: SquareMatrix + SparseMatrix2D + ?Sized> {
    /// The matrix to be analyzed.
    matrix: &'matrix M,
    /// The lowlink values for each node.
    lowlink: Vec<Option<M::Index>>,
    /// The indices for each node.
    indices: Vec<Option<M::Index>>,
    /// The stack of nodes.
    stack: Vec<M::Index>,
    /// Mask to mark nodes as on the stack.
    on_stack: Vec<bool>,
    /// Sparse row iterator stack.
    sparse_row_stack: Vec<M::SparseRow<'matrix>>,
    /// The number of strongly connected components found.
    number_of_strongly_connected_components: M::Index,
    /// Iterator over the row indices.
    row_indices: SimpleRange<M::Index>,
}

impl<'matrix, M: SquareMatrix + SparseMatrix2D + ?Sized> From<&'matrix M>
    for TarjanIterator<'matrix, M>
{
    fn from(matrix: &'matrix M) -> Self {
        Self {
            lowlink: vec![None; matrix.order().into_usize()],
            indices: vec![None; matrix.order().into_usize()],
            on_stack: vec![false; matrix.order().into_usize()],
            stack: Vec::new(),
            sparse_row_stack: Vec::new(),
            number_of_strongly_connected_components: M::Index::ZERO,
            row_indices: matrix.row_indices(),
            matrix,
        }
    }
}

impl<M: SquareMatrix + SparseMatrix2D + ?Sized> TarjanIterator<'_, M> {
    fn register_new_scc_search(&mut self, row_id: M::Index) {
        self.indices[row_id.into_usize()] = Some(self.number_of_strongly_connected_components);
        self.lowlink[row_id.into_usize()] = Some(self.number_of_strongly_connected_components);
        self.number_of_strongly_connected_components += M::Index::ONE;
        self.stack.push(row_id);
        self.sparse_row_stack.push(self.matrix.sparse_row(row_id));
        self.on_stack[row_id.into_usize()] = true;
    }

    fn last_scc_row_id(&self) -> M::Index {
        let scc_index = self.sparse_row_stack.len();
        self.stack[scc_index - 1]
    }

    fn complete_last_scc_search(&mut self) -> Option<Vec<M::Index>> {
        let row_id = self.last_scc_row_id();
        let sparse_row = self.sparse_row_stack.pop()?;
        debug_assert_eq!(
            sparse_row.collect::<Vec<_>>(),
            Vec::new(),
            "The sparse row stack should be empty when completing the last SCC for node {row_id}. {:?}, {:?}",
            self.matrix.sparse_coordinates().collect::<Vec<_>>(),
            self.matrix.shape()
        );
        if self.lowlink[row_id.into_usize()] == self.indices[row_id.into_usize()] {
            let mut component = Vec::new();
            loop {
                let column_id = self.stack.pop().unwrap();
                self.on_stack[column_id.into_usize()] = false;
                component.push(column_id);
                if column_id == row_id {
                    break;
                }
            }
            Some(component)
        } else {
            None
        }
    }

    fn last_scc_next_column_id(&mut self) -> Option<M::Index> {
        self.sparse_row_stack.last_mut().and_then(Iterator::next)
    }
}

impl<M: SquareMatrix + SparseMatrix2D> Iterator for TarjanIterator<'_, M> {
    type Item = Vec<M::Index>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sparse_row_stack.is_empty() {
            let root_id = self.row_indices.next()?;
            if self.indices[root_id.into_usize()].is_none() {
                self.register_new_scc_search(root_id);
            } else {
                return self.next();
            }
        }
        if let Some(column_id) = self.last_scc_next_column_id() {
            if self.indices[column_id.into_usize()].is_none() {
                self.register_new_scc_search(column_id);
            } else if self.on_stack[column_id.into_usize()] {
                let root_id = self.last_scc_row_id();
                self.lowlink[root_id.into_usize()] = self.lowlink[root_id.into_usize()]
                    .zip(self.indices[column_id.into_usize()])
                    .map(|(left_scc_id, right_scc_id)| left_scc_id.min(right_scc_id));
            }
            self.next()
        } else {
            // If the last sparse coordinates are exhausted, we need to determine whether it
            // completes the last strongly connected component or not.
            let column_id = self.last_scc_row_id();
            let maybe_scc = self.complete_last_scc_search();
            // If the rows iterator is exhausted, we no longer have a next row to process.
            if !self.stack.is_empty() {
                let root_id = self.last_scc_row_id();
                self.lowlink[root_id.into_usize()] = self.lowlink[root_id.into_usize()]
                    .zip(self.lowlink[column_id.into_usize()])
                    .map(|(left_scc_id, right_scc_id)| left_scc_id.min(right_scc_id));
            }

            maybe_scc.or_else(|| self.next())
        }
    }
}

/// Tarjan's algorithm for strongly connected components.
pub trait Tarjan: SquareMatrix + SparseMatrix2D {
    /// Returns the strongly connected components of the graph.
    fn tarjan(&self) -> TarjanIterator<'_, Self> {
        TarjanIterator::from(self)
    }
}

impl<M: SquareMatrix + SparseMatrix2D> Tarjan for M {}
