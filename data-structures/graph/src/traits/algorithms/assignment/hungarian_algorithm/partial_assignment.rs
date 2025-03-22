//! Submodule providing the structs to represent the state of the partial
//! assignment in the context of the Hungarian algorithm.

use std::collections::HashMap;

use algebra::prelude::{IntoUsize, Zero};

use super::{augmenting_alternating_path::SuccessorMarker, AugmentingAlternatingPath, Dual};
use crate::traits::{
    BipartiteGraph, BipartiteWeightedMonoplexGraph, MonoplexGraph, WeightedMonoplexGraph,
};

pub struct PartialAssignment<G: BipartiteWeightedMonoplexGraph + ?Sized> {
    /// The left nodes assigned to the right nodes.
    pub(super) successors: Vec<Option<G::RightNodeId>>,
    /// The right nodes assigned to the left nodes.
    pub(super) predecessors: Vec<Option<(G::LeftNodeId, Option<G::Weight>)>>,
    /// The number of assigned nodes.
    pub(super) number_of_assigned_nodes: usize,
}

impl<G: BipartiteWeightedMonoplexGraph + ?Sized, S: core::hash::BuildHasher + Default>
    From<PartialAssignment<G>> for HashMap<G::LeftNodeId, (G::RightNodeId, G::Weight), S>
{
    fn from(value: PartialAssignment<G>) -> Self {
        let mut map = HashMap::default();
        value.predecessors.iter().flatten().copied().for_each(|( left_node_id, weight)| {
            let Some(weight) = weight else {
                return;
            };
            map.insert(
                left_node_id,
                (value.successors[left_node_id.into_usize()].expect(
                    "The predecessor of the right node should always be defined when converting a partial assignment to a HashMap.",
                ), weight),
            );
        });
        map
    }
}

impl<G: BipartiteWeightedMonoplexGraph + ?Sized>
    From<PartialAssignment<G>> for Vec<(G::LeftNodeId, G::RightNodeId, G::Weight)>
{
    fn from(value: PartialAssignment<G>) -> Self {
        value.predecessors.iter().flatten().copied().map(|(left_node_id, weight)| {
            let Some(weight) = weight else {
                return None;
            };
            Some((
                left_node_id,
                value.successors[left_node_id.into_usize()].expect(
                    "The predecessor of the right node should always be defined when converting a partial assignment to a Vec.",
                ),
                weight,
            ))
        }).flatten().collect()
    }
}

impl<'graph, G: BipartiteWeightedMonoplexGraph + ?Sized> From<&Dual<'graph, G>>
    for PartialAssignment<G>
{
    /// Greedily assigns all of the available admissible cells according to the
    /// provided dual solution.
    ///
    /// # Arguments
    ///
    /// * `dual`: The dual solution.
    ///
    /// # Implementative details
    ///
    /// A tuple `(i, j)` is admissible if the following conditions are met:
    ///
    /// * The right node `i` is not assigned.
    /// * The left node `j` is not assigned.
    /// * The weight of the edge `(i, j)` is equal to the sum of the dual
    ///   weights of the right node i and the left node `j`, i.e. the associated
    ///   value in the dual graph is zero.
    fn from(dual: &Dual<'graph, G>) -> Self {
        let mut assignment = PartialAssignment {
            successors: vec![None; dual.number_of_left_nodes()],
            predecessors: vec![None; dual.number_of_right_nodes()],
            number_of_assigned_nodes: 0,
        };

        // We iterate over the right nodes.
        dual.sparse_weights().zip(dual.sparse_coordinates()).for_each(
            |(weight, (source_id, destination_id)): (
                G::Weight,
                (G::LeftNodeId, G::RightNodeId),
            )| {
                if assignment.has_no_predecessor(destination_id)
                    && assignment.has_no_successor(source_id)
                    && weight == G::Weight::ZERO
                {
                    assignment.predecessors[destination_id.into_usize()] =
                        Some((source_id, dual.reconstructed_weight(source_id, destination_id)));
                    assignment.successors[source_id.into_usize()] = Some(destination_id);
                    assignment.number_of_assigned_nodes += 1;
                }
            },
        );

        assignment
    }
}

impl<G: BipartiteWeightedMonoplexGraph + ?Sized> PartialAssignment<G> {
    /// Returns whether the assignment is complete.
    pub(super) fn is_complete(&self, graph: &G) -> bool {
        self.number_of_assigned_nodes
            == graph
                .number_of_non_singletons_in_left_partition()
                .into_usize()
                .min(self.predecessors.len())
    }

    /// Returns whether the provided right now has no predecessor.
    pub(super) fn has_no_predecessor(&self, right_node_node_id: G::RightNodeId) -> bool {
        self.predecessor(right_node_node_id).is_none()
    }

    /// Returns the assigned left node to the provided right node.
    pub(super) fn predecessor(&self, right_node_node_id: G::RightNodeId) -> Option<G::LeftNodeId> {
        self.predecessors
            .get(right_node_node_id.into_usize())
            .copied()
            .flatten()
            .map(|(predecessor, _)| predecessor)
    }

    /// Returns whether the provided left node is assigned.
    pub(super) fn has_successor(&self, left_node_node_id: G::LeftNodeId) -> bool {
        self.successor(left_node_node_id).is_some()
    }

    /// Returns whether the provided left now has no successor.
    pub(super) fn has_no_successor(&self, left_node_node_id: G::LeftNodeId) -> bool {
        self.successor(left_node_node_id).is_none()
    }

    /// Returns the assigned right node to the provided left node.
    pub(super) fn successor(&self, left_node_node_id: G::LeftNodeId) -> Option<G::RightNodeId> {
        self.successors.get(left_node_node_id.into_usize()).copied().flatten()
    }

    /// Executes a new primal iteration.
    pub(super) fn update(
        &mut self,
        mut path_end: G::RightNodeId,
        augmenting_path: &AugmentingAlternatingPath<G>,
    ) {
        loop {
            let (predecessor, weight) = augmenting_path.predecessor(path_end).expect(
                "The predecessor of the path end should always be defined when updating the assignment.",
            );
            self.successors[predecessor.into_usize()] = Some(path_end);
            self.predecessors[path_end.into_usize()] = Some((predecessor, weight));
            self.number_of_assigned_nodes += 1;

            match augmenting_path.successor(predecessor).expect(
                "The successor of the predecessor should always be defined when updating the assignment.",
            ) {
                SuccessorMarker::Successor(successor) => {
                    path_end = successor;
                    self.number_of_assigned_nodes -= 1;
                }
                SuccessorMarker::Source => {
                    break;
                }
            }
        }
    }
}
