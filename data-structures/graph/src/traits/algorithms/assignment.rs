//! Submodule providing traits and structs commonly used in the context of
//! algorithms dealing with unweighted assignments.

pub mod hopcroft_karp;
use algebra::prelude::PositiveInteger;
pub use hopcroft_karp::HopcroftKarp;

/// Trait providing an assignment.
pub trait Assignment {
    /// The left node id
    type LeftNodeId;
    /// The right node id
    type RightNodeId;

    /// Returns the assigned left node to the provided right node.
    fn predecessor(&self, right_node_node_id: Self::RightNodeId) -> Option<Self::LeftNodeId>;

    /// Returns the assigned right node to the provided left node.
    fn successor(&self, left_node_node_id: Self::LeftNodeId) -> Option<Self::RightNodeId>;

    /// Returns the number of assigned nodes.
    fn number_of_assigned_nodes(&self) -> usize;
}

impl<K: PositiveInteger, V: PositiveInteger, S: std::hash::BuildHasher + Default> Assignment
    for std::collections::HashMap<K, V, S>
{
    type LeftNodeId = K;
    type RightNodeId = V;

    /// Returns the assigned left node to the provided right node.
    fn predecessor(&self, right_node_node_id: Self::RightNodeId) -> Option<Self::LeftNodeId> {
        self.iter().find_map(|(left_node_id, assigned_right_node_id)| {
            if *assigned_right_node_id == right_node_node_id {
                Some(*left_node_id)
            } else {
                None
            }
        })
    }

    /// Returns the assigned right node to the provided left node.
    fn successor(&self, left_node_node_id: Self::LeftNodeId) -> Option<Self::RightNodeId> {
        self.get(&left_node_node_id).copied()
    }

    /// Returns the number of assigned nodes.
    fn number_of_assigned_nodes(&self) -> usize {
        self.len()
    }
}

impl<K, V> Assignment for Vec<(K, V)>
where
    K: PositiveInteger,
    V: PositiveInteger,
{
    type LeftNodeId = K;
    type RightNodeId = V;

    fn predecessor(&self, right_node_node_id: Self::RightNodeId) -> Option<Self::LeftNodeId> {
        self.iter().find_map(|(left_node_id, assigned_right_node_id)| {
            if *assigned_right_node_id == right_node_node_id {
                Some(*left_node_id)
            } else {
                None
            }
        })
    }

    fn successor(&self, left_node_node_id: Self::LeftNodeId) -> Option<Self::RightNodeId> {
        self.iter().find_map(|(left_node_id, assigned_right_node_id)| {
            if *left_node_id == left_node_node_id {
                Some(*assigned_right_node_id)
            } else {
                None
            }
        })
    }

    fn number_of_assigned_nodes(&self) -> usize {
        self.len()
    }
}
