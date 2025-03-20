//! Submodule providing an implementation of the augmenting alternating path to
//! be used in the context of the Hungarian algorithm.

use algebra::prelude::{IntoUsize, TotalOrd, Zero};

use super::{Dual, PartialAssignment};
use crate::traits::{BipartiteGraph, BipartiteWeightedMonoplexGraph, MonoplexBipartiteGraph, WeightedMonoplexGraph};

/// Struct employed in the context of the Hungarian Algorithm
/// to represent the augmenting alternating path.
pub(super) struct AugmentingAlternatingPath<G: BipartiteWeightedMonoplexGraph + ?Sized> {
    /// The labels of the left nodes.
    successor_labels: Vec<Option<SuccessorMarker<G>>>,
    /// The labels of the right nodes.
    predecessor_labels: Vec<Option<G::LeftNodeId>>,
    /// The left node labels to explore.
    left_nodes_queue: Vec<G::LeftNodeId>,
    /// The right node labels to explore.
    right_nodes_queue: Vec<G::RightNodeId>,
    /// The path minimum costs.
    path_costs: Vec<Option<(G::Weight, G::LeftNodeId)>>,
}

#[derive(Copy, Debug, PartialEq)]
/// Marker for the successor.
pub(super) enum SuccessorMarker<G: BipartiteWeightedMonoplexGraph + ?Sized> {
    Successor(G::RightNodeId),
    Source,
}

impl<G: BipartiteWeightedMonoplexGraph + ?Sized> Clone for SuccessorMarker<G> {
    fn clone(&self) -> Self {
        match self {
            SuccessorMarker::Successor(right_node_id) => SuccessorMarker::Successor(*right_node_id),
            SuccessorMarker::Source => SuccessorMarker::Source,
        }
    }
}

impl<'graph, G: BipartiteWeightedMonoplexGraph + ?Sized> AugmentingAlternatingPath<G> {
    pub(super) fn new(
        dual: &'graph Dual<'graph, G>,
        partial_assignment: &PartialAssignment<G>,
    ) -> Self {
        let mut successor_labels = vec![None; dual.number_of_left_nodes()];
        let mut left_nodes_queue = Vec::new();
        let mut path_costs = vec![None; dual.number_of_right_nodes()];

        for left_node_id in dual.left_partition_non_singleton_ids() {
            if partial_assignment.has_successor(left_node_id) {
                continue;
            }
            successor_labels[left_node_id.into_usize()] = Some(SuccessorMarker::Source);
            left_nodes_queue.push(left_node_id);
            let (minimum_cost, right_node_id) =
                dual.min_successor_weight_and_id(left_node_id).expect(
                    &format!("The minimum cost and the corresponding right node identifier should exist for the provided left node identifier {left_node_id}"),
                );
            path_costs[right_node_id.into_usize()] = Some((minimum_cost, left_node_id));
        }

        Self {
            successor_labels,
            predecessor_labels: vec![None; dual.number_of_right_nodes()],
            left_nodes_queue,
            right_nodes_queue: Vec::new(),
            path_costs,
        }
    }

    /// Returns whether has labels yet to be propagated.
    pub(super) fn has_unpropagated_labels(&self) -> bool {
        !self.left_nodes_queue.is_empty() || !self.right_nodes_queue.is_empty()
    }

    /// Returns the minimum unlabelled path cost.
    pub(super) fn minimum_unlabelled_path_cost(&self) -> Option<G::Weight> {
        self.path_costs
            .iter()
            .zip(self.predecessor_labels.iter())
            .filter(|(_, predecessor_label)| predecessor_label.is_none())
            .flat_map(|(path_cost, _)| path_cost)
            .map(|(cost, _)| *cost)
            .min_by(G::Weight::total_cmp)
    }

    /// Reduces the path cost of the provided right node by the provided value.
    ///
    /// # Arguments
    ///
    /// * `right_node_id`: The right node identifier.
    /// * `value`: The value to reduce the path cost by.
    pub(super) fn reduce_path_cost(&mut self, right_node_id: G::RightNodeId, value: G::Weight) {
        if let Some((cost, _))=  self.path_costs[right_node_id.into_usize()].as_mut() {
            // We only update the cost if it is known, as when it is `None` it is representing
            // an infinite cost, and therefore `infinity - x = infinity`.
            *cost -= value;
        } 
    }

    /// Returns whether the provided right node has a zero path cost.
    pub(super) fn has_zero_path_cost(&self, right_node_id: G::RightNodeId) -> bool {
        self.path_costs[right_node_id.into_usize()]
            .map(|(cost, _)| cost == G::Weight::ZERO)
            .unwrap_or(false)
    }

    /// Returns whether the provided right node has a predecessor label.
    ///
    /// # Arguments
    ///
    /// * `right_node_id`: The right node identifier.
    pub(super) fn has_predecessor_label(&self, right_node_id: G::RightNodeId) -> bool {
        self.predecessor_labels[right_node_id.into_usize()].is_some()
    }

    /// Returns the predecessor label of the provided right node.
    ///
    /// # Arguments
    ///
    /// * `right_node_id`: The right node identifier.
    pub(super) fn predecessor(&self, right_node_id: G::RightNodeId) -> Option<G::LeftNodeId> {
        self.predecessor_labels[right_node_id.into_usize()]
    }

    /// Returns whether the provided left node has a successor label.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The left node identifier.
    pub(super) fn has_successor_label(&self, left_node_id: G::LeftNodeId) -> bool {
        self.successor(left_node_id).is_some()
    }

    /// Returns the successor label of the provided left node.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The left node identifier.
    pub(super) fn successor(&self, left_node_id: G::LeftNodeId) -> Option<SuccessorMarker<G>> {
        self.successor_labels[left_node_id.into_usize()].clone()
    }

    /// Labels the predecessor of the provided right node.
    ///
    /// # Arguments
    ///
    /// * `right_node_id`: The right node identifier.
    /// * `predecessor_label`: The predecessor label.
    pub(super) fn predecessor_label(
        &mut self,
        right_node_id: G::RightNodeId,
        predecessor_label: G::LeftNodeId,
    ) {
        self.predecessor_labels[right_node_id.into_usize()] = Some(predecessor_label);
    }

    /// Returns the source of the path associated to the given right node.
    pub(super) fn path_source(&self, right_node_id: G::RightNodeId) -> G::LeftNodeId {
        self.path_costs[right_node_id.into_usize()]
            .expect("The path cost should exist for the provided right node identifier.")
            .1
    }

    /// Labels the successor of the provided left node.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The left node identifier.
    /// * `successor_label`: The successor label.
    fn successor_label(&mut self, left_node_id: G::LeftNodeId, successor_label: G::RightNodeId) {
        self.successor_labels[left_node_id.into_usize()] =
            Some(SuccessorMarker::Successor(successor_label));
    }

    /// Adds the provided left node to the queue.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The left node identifier.
    fn add_left_node_to_queue(&mut self, left_node_id: G::LeftNodeId) {
        self.left_nodes_queue.push(left_node_id);
    }

    /// Adds the provided right node to the queue.
    ///
    /// # Arguments
    ///
    /// * `right_node_id`: The right node identifier.
    pub(super) fn add_right_node_to_queue(&mut self, right_node_id: G::RightNodeId) {
        self.right_nodes_queue.push(right_node_id);
    }
}

impl<G: BipartiteWeightedMonoplexGraph + ?Sized> AugmentingAlternatingPath<G> {
    /// Executes the propagation of the labels.
    ///
    /// # Arguments
    ///
    /// * `partial_assignment`: The partial assignment.
    /// * `dual`: The dual graph.
    ///
    /// # Returns
    ///
    /// The identifier of the right node if the propagation is successful.
    pub(super) fn propagate_labels(
        &mut self,
        partial_assignment: &PartialAssignment<G>,
        dual: &Dual<G>,
    ) -> Option<G::RightNodeId> {
        while let Some(left_node_node_id) = self.left_nodes_queue.pop() {
            self.forward_label_propagation(left_node_node_id, dual);
        }
        while let Some(right_node_id) = self.right_nodes_queue.pop() {
            if let Some(assigned_left_node_id) = partial_assignment.predecessor(right_node_id) {
                self.backward_label_propagation(&right_node_id, assigned_left_node_id, dual);
            } else {
                return Some(right_node_id);
            }
        }
        None
    }

    /// Forward label propagation.
    fn forward_label_propagation(&mut self, k: G::LeftNodeId, dual: &Dual<G>) {
        for right_node_id in dual.zero_weight_successors(k) {
            if self.has_predecessor_label(right_node_id) {
                continue;
            }
            self.right_nodes_queue.push(right_node_id);
            self.predecessor_labels[right_node_id.into_usize()] = Some(k);
        }
    }

    /// Backward label propagation.
    fn backward_label_propagation(
        &mut self,
        k: &G::RightNodeId,
        assigned_left_node_id: G::LeftNodeId,
        dual: &Dual<'_, G>,
    ) {
        if self.has_successor_label(assigned_left_node_id) {
            return;
        }

        self.successor_label(assigned_left_node_id, *k);
        self.add_left_node_to_queue(assigned_left_node_id);

        // We search the minimum cost of the successors of the assigned left node.
        let (minimum_cost, right_node_id) = dual
            .min_successor_weight_and_id(assigned_left_node_id)
            .expect("The minimum cost and the corresponding right node identifier should exist.");
        self.path_costs[right_node_id.into_usize()] = Some((minimum_cost, assigned_left_node_id));
    }
}
