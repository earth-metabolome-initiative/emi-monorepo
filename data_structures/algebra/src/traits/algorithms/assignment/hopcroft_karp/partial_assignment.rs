//! Partial assignment for Hopcroft-Karp algorithm.

use numeric_common_traits::prelude::{IntoUsize, Number};

use super::HopcroftKarpError;
use crate::traits::SparseMatrix2D;

/// Struct representing a partial assignment.
pub struct PartialAssignment<'a, M: SparseMatrix2D + ?Sized, Distance = u16> {
    predecessors: Vec<Option<M::RowIndex>>,
    successors: Vec<Option<M::ColumnIndex>>,
    left_distances: Vec<Distance>,
    null_distance: Distance,
    matrix: &'a M,
}

impl<'a, M: SparseMatrix2D + ?Sized, Distance: Number> From<PartialAssignment<'a, M, Distance>>
    for Vec<(M::RowIndex, M::ColumnIndex)>
{
    fn from(assignment: PartialAssignment<'a, M, Distance>) -> Self {
        assignment
            .successors
            .iter()
            .copied()
            .filter_map(|right_node_id: Option<M::ColumnIndex>| {
                let right_node_id = right_node_id?;
                let row_index = assignment.predecessors[right_node_id.into_usize()]?;
                Some((row_index, right_node_id))
            })
            .collect()
    }
}

impl<'a, M: SparseMatrix2D + ?Sized, Distance: Number> From<&'a M>
    for PartialAssignment<'a, M, Distance>
{
    fn from(matrix: &'a M) -> Self {
        let predecessors = vec![None; matrix.number_of_columns().into_usize()];
        let successors = vec![None; matrix.number_of_rows().into_usize()];
        let left_distances = vec![Distance::MAX; matrix.number_of_rows().into_usize()];
        PartialAssignment {
            predecessors,
            successors,
            left_distances,
            matrix,
            null_distance: Distance::MAX,
        }
    }
}

impl<M: SparseMatrix2D + ?Sized, Distance: Number> PartialAssignment<'_, M, Distance> {
    /// Returns whether the provided left node id has a successor.
    pub(super) fn has_successor(&self, row_index: M::RowIndex) -> bool {
        self.successors[row_index.into_usize()].is_some()
    }

    pub(super) fn bfs(&mut self) -> Result<bool, HopcroftKarpError> {
        let mut frontier = Vec::new();
        for row_index in self.matrix.row_indices() {
            if self.has_successor(row_index) {
                self.left_distances[row_index.into_usize()] = Distance::MAX;
            } else {
                self.left_distances[row_index.into_usize()] = Distance::ZERO;
                frontier.push(row_index);
            }
        }

        self.null_distance = Distance::MAX;

        while !frontier.is_empty() {
            let mut tmp_frontier = Vec::new();
            for row_index in frontier {
                let left_distance = self.left_distances[row_index.into_usize()];
                if left_distance < self.null_distance {
                    if left_distance == Distance::MAX - Distance::ONE {
                        return Err(HopcroftKarpError::InsufficientDistanceType);
                    }
                    for right_node_id in self.matrix.sparse_row(row_index) {
                        let maybe_predecessor_id = self.predecessors[right_node_id.into_usize()];
                        let predecessor_distance = self.left_distance_mut(maybe_predecessor_id);
                        if *predecessor_distance == Distance::MAX {
                            *predecessor_distance = left_distance + Distance::ONE;
                            tmp_frontier.extend(maybe_predecessor_id);
                        }
                    }
                }
            }
            frontier = tmp_frontier;
        }

        Ok(self.null_distance != Distance::MAX)
    }

    /// Returns the distance of the provided left node id.
    ///
    /// # Arguments
    ///
    /// * `row_index`: The identifier of the left node.
    fn left_distance(&self, row_index: Option<M::RowIndex>) -> Distance {
        let Some(row_index) = row_index else {
            return self.null_distance;
        };
        self.left_distances[row_index.into_usize()]
    }

    /// Returns a mutable reference to the distance of the provided left node
    /// id.
    ///
    /// # Arguments
    ///
    /// * `row_index`: The identifier of the left node.
    fn left_distance_mut(&mut self, row_index: Option<M::RowIndex>) -> &mut Distance {
        let Some(row_index) = row_index else {
            return &mut self.null_distance;
        };
        &mut self.left_distances[row_index.into_usize()]
    }

    pub(super) fn dfs(&mut self, row_index: Option<M::RowIndex>) -> bool {
        let Some(row_index) = row_index else {
            return true;
        };
        let left_distance = self.left_distances[row_index.into_usize()];
        for successor_id in self.matrix.sparse_row(row_index) {
            let maybe_predecessor_id = self.predecessors[successor_id.into_usize()];
            if self.left_distance(maybe_predecessor_id) == left_distance + Distance::ONE
                && self.dfs(maybe_predecessor_id)
            {
                self.successors[row_index.into_usize()] = Some(successor_id);
                self.predecessors[successor_id.into_usize()] = Some(row_index);
                return true;
            }
        }

        self.left_distances[row_index.into_usize()] = Distance::MAX;
        false
    }
}
