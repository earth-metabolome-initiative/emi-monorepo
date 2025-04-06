//! Submodule providing the Hopcroft-Karp algorithm for the assignment problem.

mod partial_assignment;

use partial_assignment::PartialAssignment;

use crate::traits::SparseMatrix2D;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enumeration of the errors that might occur during the Hopcroft-Karp
/// algorithm.
pub enum HopcroftKarpError {
    /// The provided distance type is not large enough to be used in the
    /// algorithm for the provided graph.
    InsufficientDistanceType,
}

/// Trait providing the Hopcroft-Karp algorithm for the assignment problem.
pub trait HopcroftKarp: SparseMatrix2D {
    /// Return the assignment as assigned by the Hopcroft-Karp algorithm.
    fn hopcroft_karp(&self) -> Result<Vec<(Self::RowIndex, Self::ColumnIndex)>, HopcroftKarpError> {
        let mut partial_assignment: PartialAssignment<'_, Self, u32> =
            PartialAssignment::from(self);
        while partial_assignment.bfs()? {
            for left_node_id in self.row_indices() {
                if !partial_assignment.has_successor(left_node_id) {
                    partial_assignment.dfs(Some(left_node_id));
                }
            }
        }

        Ok(partial_assignment.into())
    }
}

impl<M> HopcroftKarp for M where M: SparseMatrix2D {}
