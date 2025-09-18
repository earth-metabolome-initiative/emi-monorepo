//! Submodule providing the trait `SimplePath`, which checks if the directed
//! graph associated with the provided structure is a simple path, i.e., a
//! linear sequence of nodes where each node (except the first and last) has
//! exactly one predecessor and one successor, with the last node having no
//! successors and the first node having no predecessors.

use num_traits::{ConstOne, ConstZero, SaturatingAdd};

use crate::traits::RootNodes;

/// A simple path is a special case of a directed acyclic graph (DAG) where
/// there is a single linear sequence of nodes. In a simple path, each node
/// (except the first and last) has exactly one predecessor and one successor,
/// with the last node having no successors and the first node having no
/// predecessors.
pub trait SimplePath: RootNodes {
    /// Returns whether the directed graph associated with the structure is a
    /// simple path.
    fn is_simple_path(&self) -> bool {
        if self.number_of_nodes() == Self::NodeId::ZERO {
            // An empty graph is not a simple path.
            return false;
        }

        let root_nodes = self.root_nodes();
        if root_nodes.len() != 1 {
            return false; // A simple path must have exactly one root node.
        }

        let mut current_node = root_nodes[0];
        let mut visited_nodes = Self::NodeId::ZERO;

        loop {
            visited_nodes = visited_nodes.saturating_add(&Self::NodeId::ONE);
            if self.out_degree(current_node) > Self::NodeId::ONE {
                return false; // More than one outgoing edge means it's not a simple path.
            }

            if let Some(successor) = self.successors(current_node).next() {
                if visited_nodes == self.number_of_nodes() {
                    return false; // More visited nodes than total nodes means a cycle.
                }

                current_node = successor;
            } else {
                break;
            }
        }

        visited_nodes == self.number_of_nodes()
    }
}

impl<T> SimplePath for T where T: RootNodes {}
