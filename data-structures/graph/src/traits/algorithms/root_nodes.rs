//! Submodule providing the `RootNodes` trait and its blanket
//! implementation, which provides a method to retrieve the root nodes of the
//! graph, which are the set of nodes with no predecessors.

use algebra::prelude::IntoUsize;

use super::{TopologicalSorting, topological_sorting::TopologicalSortingError};

/// Trait providing the `root_nodes` method, which returns the root nodes of the
/// graph. A root node is a node with no predecessors.
pub trait RootNodes: TopologicalSorting {
    /// Returns the root nodes of the graph.
    fn root_nodes(&self) -> Vec<Self::NodeId> {
        // Create a vector to store whether a node has been visited or not
        // and initialize it to false for all nodes.
        let mut visited = vec![false; self.number_of_nodes().into_usize()];

        // Iterate over all nodes and mark the successors of each node as
        // visited. A node is considered visited if it has a predecessor.
        for node in self.node_ids() {
            // Mark the successors of the node as visited.
            for successor_node_id in self.successors(node) {
                visited[successor_node_id.into_usize()] = true;
            }
        }

        // Finally, we iterate over all nodes and keep the nodes that have not
        // been visited. A node is considered visited if it has a predecessor.
        self.node_ids()
            .zip(visited)
            .filter_map(|(node, visited)| if visited { None } else { Some(node) })
            .collect()
    }

    /// Returns the topological order of the graph.
    ///
    /// # Errors
    ///
    /// * `TopologicalSortingError::UnreachableNodes` - If some nodes are not
    ///   reachable from the root nodes.
    fn topological_sort_from_roots(&self) -> Result<Vec<Self::NodeId>, TopologicalSortingError> {
        // Get the root nodes of the graph.
        let root_nodes = self.root_nodes();

        // Perform a topological sort from the root nodes.
        self.topological_sort_from_nodes(&root_nodes)
    }
}

impl<G: TopologicalSorting> RootNodes for G {}
