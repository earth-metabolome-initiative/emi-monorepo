//! Submodule providing traits and structs commonly used in the context of
//! algorithms dealing with weighted assignments.

pub mod hungarian_algorithm;

use algebra::prelude::{Number, PositiveInteger};
pub use hungarian_algorithm::HungarianAlgorithm;

/// Trait providing an assignment.
pub trait WeightedAssignment: Default {
    /// The left node identifier.
    type LeftNodeId;
    /// The right node identifier.
    type RightNodeId;
    /// The weight of the assignment.
    type Weight;

    /// Returns the assigned left node to the provided right node.
    fn predecessor(
        &self,
        right_node_node_id: Self::RightNodeId,
    ) -> Option<(Self::LeftNodeId, Self::Weight)>;

    /// Returns the assigned right node to the provided left node.
    fn successor(
        &self,
        left_node_node_id: Self::LeftNodeId,
    ) -> Option<(Self::RightNodeId, Self::Weight)>;

    /// Returns the total cost of the assignment.
    fn cost(&self) -> Self::Weight;

    /// Returns the number of assigned nodes.
    fn number_of_assigned_nodes(&self) -> usize;
}

/// Implements the Assignment trait for the `HashMap` data structure.
impl<K, V, W, S: std::hash::BuildHasher + Default> WeightedAssignment
    for std::collections::HashMap<K, (V, W), S>
where
    K: PositiveInteger,
    V: PositiveInteger,
    W: Number,
{
    type LeftNodeId = K;
    type RightNodeId = V;
    type Weight = W;

    /// Returns the assigned left node to the provided right node.
    fn predecessor(
        &self,
        right_node_node_id: Self::RightNodeId,
    ) -> Option<(Self::LeftNodeId, Self::Weight)> {
        self.iter().find_map(|(left_node_id, (assigned_right_node_id, weight))| {
            if *assigned_right_node_id == right_node_node_id {
                Some((*left_node_id, *weight))
            } else {
                None
            }
        })
    }

    /// Returns the assigned right node to the provided left node.
    fn successor(
        &self,
        left_node_node_id: Self::LeftNodeId,
    ) -> Option<(Self::RightNodeId, Self::Weight)> {
        self.get(&left_node_node_id).map(|(right_node_id, weight)| (*right_node_id, *weight))
    }

    /// Returns the total cost of the assignment.
    fn cost(&self) -> Self::Weight {
        self.iter().map(|(_, (_, weight))| *weight).sum()
    }

    /// Returns the number of assigned nodes.
    fn number_of_assigned_nodes(&self) -> usize {
        self.len()
    }
}

/// Implements the Assignment trait for the `Vec` data structure.
impl<K, V, W> WeightedAssignment for Vec<(K, V, W)>
where
    K: PositiveInteger,
    V: PositiveInteger,
    W: Number,
{
    type LeftNodeId = K;
    type RightNodeId = V;
    type Weight = W;

    /// Returns the assigned left node to the provided right node.
    fn predecessor(
        &self,
        right_node_node_id: Self::RightNodeId,
    ) -> Option<(Self::LeftNodeId, Self::Weight)> {
        self.iter().find_map(|(left_node_id, assigned_right_node_id, weight)| {
            if *assigned_right_node_id == right_node_node_id {
                Some((*left_node_id, *weight))
            } else {
                None
            }
        })
    }

    /// Returns the assigned right node to the provided left node.
    fn successor(
        &self,
        left_node_node_id: Self::LeftNodeId,
    ) -> Option<(Self::RightNodeId, Self::Weight)> {
        self.iter().find_map(|(assigned_left_node_id, right_node_id, weight)| {
            if *assigned_left_node_id == left_node_node_id {
                Some((*right_node_id, *weight))
            } else {
                None
            }
        })
    }

    /// Returns the total cost of the assignment.
    fn cost(&self) -> Self::Weight {
        self.iter().map(|(_, _, weight)| *weight).sum()
    }

    /// Returns the number of assigned nodes.
    fn number_of_assigned_nodes(&self) -> usize {
        self.len()
    }
}
