//! Submodule providing an implementation of the Dual Graph to be used in the
//! context of the Hungarian algorithm.

use dual_edges::DualEdges;

use super::AugmentingAlternatingPath;
use crate::traits::BipartiteWeightedMonoplexGraph;

mod dual_edges;
mod graph_traits;

/// Struct employed in the context of the Hungarian Algorithm
/// to represent the dual weighted graph.
pub(super) struct Dual<'graph, G: BipartiteWeightedMonoplexGraph + ?Sized> {
    /// The associated dual graph.
    graph: &'graph G,
    /// The underlying edges.
    edges: DualEdges<'graph, G::Edges>,
}

impl<'graph, G: BipartiteWeightedMonoplexGraph + ?Sized> From<&'graph G> for Dual<'graph, G> {
    fn from(graph: &'graph G) -> Self {
        let edges = DualEdges::from(graph.edges());
        Dual { graph, edges }
    }
}

impl<G: BipartiteWeightedMonoplexGraph + ?Sized> Dual<'_, G> {
    /// Returns an iterator over the successors of a given left node
    /// which are characterized by having a zero weight.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The identifier of the left node.
    pub(super) fn zero_weight_successors(
        &self,
        left_node_id: G::LeftNodeId,
    ) -> impl Iterator<Item = G::RightNodeId> + '_ {
        self.edges.zero_weight_successors(left_node_id)
    }

    /// Returns the reconstructed weight for the provided node tuple.
    pub(super) fn reconstructed_weight(
        &self,
        left_node_id: G::LeftNodeId,
        right_node_id: G::RightNodeId,
    ) -> Option<G::Weight> {
        if let (Some(left_node_weight), Some(right_node_weight)) =
            (self.edges.left_node_weight(left_node_id), self.edges.right_node_weight(right_node_id))
        {
            Some(left_node_weight + right_node_weight)
        } else {
            None
        }
    }

    /// Executes an iteration of the dual algorithm.
    pub(super) fn update(
        &mut self,
        augmenting_alternating_path: &mut AugmentingAlternatingPath<G>,
    ) -> bool {
        let Some(delta) = augmenting_alternating_path.minimum_unlabelled_path_cost() else {
            return false;
        };

        for left_node_id in self.graph.left_node_ids() {
            if augmenting_alternating_path.has_successor_label(left_node_id) {
                self.edges.increase_left_node_weight(left_node_id, delta);
            }
        }

        for right_node_id in self.graph.right_node_ids() {
            if augmenting_alternating_path.has_predecessor_label(right_node_id) {
                self.edges.decrease_right_node_weight(right_node_id, delta);
            } else {
                augmenting_alternating_path.reduce_path_cost(right_node_id, delta);
                if augmenting_alternating_path.has_zero_path_cost(right_node_id) {
                    // Changes the cost of the path associated to
                    // `right_node_id`.
                    let left_node_id = augmenting_alternating_path.path_source(right_node_id);
                    let path_weight = self.reconstructed_weight(left_node_id, right_node_id).map(
                        |reconstructed_weight| {
                            reconstructed_weight
                                + augmenting_alternating_path.path_cost(right_node_id)
                        },
                    );
                    augmenting_alternating_path.predecessor_label(
                        right_node_id,
                        left_node_id,
                        path_weight,
                    );
                    augmenting_alternating_path.add_right_node_to_queue(right_node_id);
                }
            }
        }

        true
    }
}
