//! Submodule providing the `Kahn` trait and its blanket implementation for
//! sparse matrices, which provides the Kahn's algorithm for topological
//! sorting, plus an additional method to attempt to convert an arbitrary matrix
//! into an upper triangular matrix, which would fail if the matrix contains
//! cycles.

use crate::traits::{IntoUsize, One, SparseSquareMatrix, Zero};

#[derive(Debug, Clone, PartialEq)]
/// Error enumeration for Kahn's algorithm.
pub enum KahnError {
    /// Error when the graph contains a cycle.
    Cycle,
}

/// Kahn's algorithm for topological sorting.
pub trait Kahn: SparseSquareMatrix {
    /// Returns the indices to rearrange the rows of the matrix in a topological
    /// order.
    fn kahn(&self) -> Result<Vec<Self::Index>, KahnError> {
        let mut in_degree = vec![Self::Index::ZERO; self.order().into_usize()];
        let mut frontier = Vec::new();
        let mut temporary_frontier = Vec::new();
        let mut number_of_visited_nodes = Self::Index::ZERO;
        let mut topological_order = vec![Self::Index::ZERO; self.order().into_usize()];

        for row_id in self.row_indices() {
            for successor_id in self.sparse_row(row_id) {
                in_degree[successor_id.into_usize()] += Self::Index::ONE;
            }
        }

        frontier.extend(
            self.row_indices().filter(|row_id| in_degree[row_id.into_usize()] == Self::Index::ZERO),
        );

        while !frontier.is_empty() {
            temporary_frontier.clear();

            for row_id in frontier.drain(..) {
                topological_order[row_id.into_usize()] = number_of_visited_nodes;
                number_of_visited_nodes += Self::Index::ONE;
                temporary_frontier.extend(self.sparse_row(row_id).filter_map(|successor_id| {
                    in_degree[successor_id.into_usize()] -= Self::Index::ONE;
                    (in_degree[successor_id.into_usize()] == Self::Index::ZERO)
                        .then(|| successor_id)
                }));
            }

            core::mem::swap(&mut frontier, &mut temporary_frontier);
        }

        if number_of_visited_nodes != self.order() {
            return Err(KahnError::Cycle);
        }

        Ok(topological_order)
    }
}

impl<G: SparseSquareMatrix> Kahn for G {}
