//! Submodule providing the support dual edges to be used in the context of the
//! `Dual` struct of the Hungarian algorithm.

mod dual_matrix;
use dual_matrix::DualMatrix;
mod edges_traits;

use crate::traits::WeightedEdges;

pub struct DualEdges<'edges, E: WeightedEdges + ?Sized> {
    /// The reference to the underlying matrix.
    matrix: DualMatrix<'edges, E::WeightedMatrix>,
}

impl<E: WeightedEdges + ?Sized> DualEdges<'_, E> {
    /// Returns an iterator over the successors of a given left node
    /// which are characterized by having a zero weight.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The identifier of the left node.
    pub fn zero_weight_successors(
        &self,
        left_node_id: E::SourceNodeId,
    ) -> impl Iterator<Item = E::DestinationNodeId> + '_ {
        self.matrix.zero_weight_columns(left_node_id)
    }

    /// Increases the weight of the left node by the provided value.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The identifier of the left node.
    /// * `value`: The value to increase the weight by.
    pub fn increase_left_node_weight(&mut self, left_node_id: E::SourceNodeId, value: E::Weight) {
        self.matrix.increase_left_node_weight(left_node_id, value);
    }

    /// Reduces the weight of the right node by the provided value.
    ///
    /// # Arguments
    ///
    /// * `right_node_id`: The identifier of the right node.
    /// * `value`: The value to reduce the weight by.
    pub fn decrease_right_node_weight(
        &mut self,
        right_node_id: E::DestinationNodeId,
        value: E::Weight,
    ) {
        self.matrix.decrease_right_node_weight(right_node_id, value);
    }
}
