//! Submodule providing the `Johnson` trait and its blanket implementation for
//! sparse matrices, which provides the Johnson's algorithm for finding all
//! cycles in a sparse matrix.

use lender::prelude::{Lender, Lending};
use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::IntoUsize;

use crate::{
    impls::{LowerBoundedSquareMatrix, SubsetSquareMatrix},
    traits::{SparseMatrix2D, SquareMatrix, Tarjan},
};

#[allow(clippy::type_complexity)]
/// Iterator for Johnson's algorithm.
struct CircuitSearch<'lend, 'matrix, M: SquareMatrix + SparseMatrix2D>
{
    /// The matrix to find cycles in.
    data: &'lend mut Data<&'matrix M>,
    /// The current root node.
    current_root_id: M::Index,
    /// The current component matrix.
    current_component: &'lend SubsetSquareMatrix<LowerBoundedSquareMatrix<&'matrix M>, Vec<M::Index>>,
    /// The row iterators for the current component.
    row_iterators: Vec<<SubsetSquareMatrix<LowerBoundedSquareMatrix<&'matrix M>, Vec<M::Index>> as SparseMatrix2D>::SparseRow<'lend>>,
}

impl<'lend, 'matrix, M: SquareMatrix + SparseMatrix2D>
    From<&'lend mut InnerJohnsonIterator<'matrix, M>> for CircuitSearch<'lend, 'matrix, M>
{
    fn from(parent: &'lend mut InnerJohnsonIterator<'matrix, M>) -> Self {
        let mut circuit_search = Self {
            current_component: parent.current_component.as_ref().unwrap(),
            current_root_id: parent.data.current_root_id,
            data: &mut parent.data,
            row_iterators: Vec::new(),
        };
        debug_assert!(
            circuit_search.data.stack.is_empty(),
            "Stack should be empty at the start of the circuit search"
        );
        circuit_search.data.found_circuit = false;
        circuit_search.data.current_root_id += M::Index::ONE;
        circuit_search.register_circuit_search(circuit_search.current_root_id);
        circuit_search
    }
}

impl<M: SquareMatrix + SparseMatrix2D> CircuitSearch<'_, '_, M> {
    fn unblock(&mut self, row_id: M::Index) {
        self.data.blocked[row_id.into_usize()] = false;
        let row_block = core::mem::take(&mut self.data.block_map[row_id.into_usize()]);
        for column_id in row_block {
            if self.data.blocked[column_id.into_usize()] {
                self.unblock(column_id);
            }
        }
    }

    fn is_blocked(&self, row_id: M::Index) -> bool {
        self.data.blocked[row_id.into_usize()]
    }

    fn next_circuit(&mut self) -> Option<&[M::Index]> {
        while let Some(column_id) = self.last_circuit_next_column() {
            if column_id == self.current_root_id {
                self.data.found_circuit = true;
                return Some(self.data.stack.as_slice());
            }

            if !self.is_blocked(column_id) {
                self.register_circuit_search(column_id);
                return self.next_circuit();
            }
        }

        self.remove_last_circuit_search();
        None
    }

    fn search_circuit(&mut self) -> Option<&[M::Index]> {
        while !self.row_iterators.is_empty() {
            while let Some(column_id) = self.last_circuit_next_column() {
                if column_id == self.current_root_id {
                    self.data.found_circuit = true;
                    return Some(self.data.stack.as_slice());
                }

                if !self.is_blocked(column_id) {
                    self.register_circuit_search(column_id);
                    return self.next_circuit();
                }
            }

            self.remove_last_circuit_search();
        }
        debug_assert!(
            self.data.stack.is_empty(),
            "Stack should be empty after removing last circuit search"
        );
        debug_assert!(
            self.row_iterators.is_empty(),
            "Row iterators should be empty after removing last circuit search"
        );
        None
    }

    fn last_circuit_next_column(&mut self) -> Option<M::Index> {
        self.row_iterators.last_mut().and_then(Iterator::next)
    }

    fn register_circuit_search(&mut self, row_id: M::Index) {
        debug_assert!(
            self.data.stack.len() < self.data.block_map.len(),
            "Stack length {} should be less than block map length {}",
            self.data.stack.len(),
            self.data.block_map.len()
        );
        debug_assert!(
            self.row_iterators.len() < self.data.block_map.len(),
            "Row iterators length {} should be less than block map length {}",
            self.row_iterators.len(),
            self.data.block_map.len()
        );
        self.data.stack.push(row_id);
        self.row_iterators.push(self.current_component.sparse_row(row_id));
        self.data.blocked[row_id.into_usize()] = true;
    }

    fn remove_last_circuit_search(&mut self) {
        let row_id = self.data.stack.pop().unwrap();
        let mut row_iterator = self.row_iterators.pop().unwrap();
        debug_assert!(row_iterator.next().is_none(), "Row iterator should be empty after popping");
        if self.data.found_circuit {
            self.unblock(row_id);
        } else {
            for column_id in self.current_component.sparse_row(row_id) {
                if self.data.block_map[column_id.into_usize()].contains(&row_id) {
                    self.data.block_map[column_id.into_usize()].push(row_id);
                }
            }
        }
    }
}

impl<'lend2, M: SquareMatrix + SparseMatrix2D> Lending<'lend2> for CircuitSearch<'_, '_, M> {
    type Lend = &'lend2 [M::Index];
}

impl<M: SquareMatrix + SparseMatrix2D> Lender for CircuitSearch<'_, '_, M> {
    fn next(&mut self) -> Option<<Self as Lending<'_>>::Lend> {
        self.search_circuit()
    }
}

struct Data<M: SquareMatrix + SparseMatrix2D> {
    /// The current root node.
    current_root_id: M::Index,
    /// The blocked nodes.
    blocked: Vec<bool>,
    /// The stack of nodes in the current circuit.
    stack: Vec<M::Index>,
    /// Whether a circuit has been found.
    found_circuit: bool,
    /// The block map for the current component.
    block_map: Vec<Vec<M::Index>>,
}

impl<M: SquareMatrix + SparseMatrix2D> From<M> for Data<M> {
    fn from(matrix: M) -> Self {
        let order = matrix.order();
        let blocked = vec![false; order.into_usize()];
        let block_map = vec![Vec::new(); order.into_usize()];
        Self {
            current_root_id: M::Index::ZERO,
            blocked,
            stack: Vec::new(),
            found_circuit: false,
            block_map,
        }
    }
}

/// Iterator for Johnson's algorithm.
struct InnerJohnsonIterator<'matrix, M: SquareMatrix + SparseMatrix2D> {
    /// The matrix to find cycles in.
    matrix: &'matrix M,
    /// The underlying data structure for the algorithm.
    data: Data<&'matrix M>,
    /// The current component matrix.
    current_component:
        Option<SubsetSquareMatrix<LowerBoundedSquareMatrix<&'matrix M>, Vec<M::Index>>>,
}

impl<'lend, 'matrix, M: SquareMatrix + SparseMatrix2D> Lending<'lend>
    for InnerJohnsonIterator<'matrix, M>
{
    type Lend = CircuitSearch<'lend, 'matrix, M>;
}

impl<M: SquareMatrix + SparseMatrix2D> Lender for InnerJohnsonIterator<'_, M> {
    fn next(&mut self) -> Option<<Self as Lending<'_>>::Lend> {
        if self.data.current_root_id < self.matrix.order() {
            let bounded_matrix =
                LowerBoundedSquareMatrix::new(self.matrix, self.data.current_root_id).unwrap();
            let Some((new_root_id, mut strongly_connected_component_with_smallest_node)): Option<
                (M::Index, Vec<M::Index>),
            > = bounded_matrix
                .tarjan()
                // We skip the singletons, as they are not cycles.
                .filter(|scc| scc.len() > 1)
                .map(|scc| (scc.iter().min().copied().unwrap(), scc))
                .min_by_key(|(node_id, _)| *node_id)
            else {
                // No strongly connected components left, so we can
                // break out of the loop.
                self.data.current_root_id = self.matrix.order();
                return None;
            };

            debug_assert!(
                self.data.current_root_id <= new_root_id,
                "current_root_id {} should be less than or equal to new_root_id {new_root_id} in scc {strongly_connected_component_with_smallest_node:?}",
                self.data.current_root_id
            );

            self.data.current_root_id = new_root_id;
            for row_id in &strongly_connected_component_with_smallest_node {
                self.data.blocked[row_id.into_usize()] = false;
                self.data.block_map[row_id.into_usize()].clear();
            }

            strongly_connected_component_with_smallest_node.sort_unstable();
            self.current_component = Some(SubsetSquareMatrix::with_sorted_indices(
                bounded_matrix,
                strongly_connected_component_with_smallest_node,
            ));

            // The bug is currently documented here: https://github.com/WanderLanz/Lender/pull/8
            // Once the bug is fixed, we can remove this clear.
            self.data.stack.clear();
            debug_assert!(
                self.data.stack.is_empty(),
                "Stack at address {} should be empty at the start of the circuit search, but in parent is {:?}",
                self.data.stack.as_ptr() as usize,
                self.data.stack
            );

            return Some(CircuitSearch::from(self));
        }

        None
    }
}

impl<'matrix, M: SquareMatrix + SparseMatrix2D> From<&'matrix M>
    for InnerJohnsonIterator<'matrix, M>
{
    fn from(matrix: &'matrix M) -> Self {
        Self { matrix, data: Data::from(matrix), current_component: None }
    }
}

/// Johnson's algorithm for finding all cycles in a sparse matrix.
pub struct JohnsonIterator<'matrix, M: SquareMatrix + SparseMatrix2D> {
    /// The underlying iterator.
    inner: lender::Flatten<'matrix, InnerJohnsonIterator<'matrix, M>>,
}

impl<'matrix, M: SquareMatrix + SparseMatrix2D> From<&'matrix M> for JohnsonIterator<'matrix, M> {
    fn from(matrix: &'matrix M) -> Self {
        Self { inner: InnerJohnsonIterator::from(matrix).flatten() }
    }
}

impl<M: SquareMatrix + SparseMatrix2D> Iterator for JohnsonIterator<'_, M> {
    type Item = Vec<M::Index>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(<[M::Index]>::to_vec)
    }
}

/// Johnson's algorithm for finding all cycles in a sparse matrix.
pub trait Johnson: SquareMatrix + SparseMatrix2D + Sized {
    /// Finds all cycles in a sparse matrix.
    fn johnson(&self) -> JohnsonIterator<'_, Self> {
        JohnsonIterator::from(self)
    }
}

impl<M: SquareMatrix + SparseMatrix2D> Johnson for M {}
