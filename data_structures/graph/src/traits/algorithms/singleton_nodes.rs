//! Submodule providing the `SingletonNodes` trait and its blanket
//! implementation, which provides a method to retrieve the singleton nodes from
//! a graph, which is a node with no predecessor and no successors edges.

use numeric_common_traits::prelude::IntoUsize;

use crate::traits::MonoplexMonopartiteGraph;
/// Trait providing the `singleton_nodes` method, which returns the singleton
/// nodes of the graph. A singleton node is a node with no predecessors and no
/// successors.
pub trait SingletonNodes: MonoplexMonopartiteGraph {
    /// Returns the singleton nodes of the graph.
    fn singleton_nodes(&self) -> Vec<Self::NodeId> {
        let mut visited = vec![false; self.number_of_nodes().into_usize()];

        // Iterate over all nodes and mark the successors of each node as
        // visited. A node is considered visited if it has a predecessor.
        for node in self.node_ids() {
            let mut has_successors = false;
            // Mark the successors of the node as visited.
            for successor_node_id in self.successors(node) {
                visited[successor_node_id.into_usize()] = true;
                has_successors = true;
            }
            visited[node.into_usize()] = has_successors;
        }
        // Finally, we iterate over all nodes and keep the nodes that have not
        // been visited. A node is considered visited if it has a predecessor.
        self.node_ids()
            .zip(visited)
            .filter_map(|(node, visited)| if visited { None } else { Some(node) })
            .collect()
    }
}

impl<G: MonoplexMonopartiteGraph + ?Sized> SingletonNodes for G {}
