//! Submodule providing the `SinkNodes` trait and its blanket
//! implementation, which provides a method to retrieve the sink nodes of the
//! graph, which are the set of nodes with no successors.

use numeric_common_traits::prelude::IntoUsize;

use crate::traits::MonoplexMonopartiteGraph;
/// Trait providing the `sink_nodes` method, which returns the sink nodes of the
/// graph. A sink node is a node with no successors.
pub trait SinkNodes: MonoplexMonopartiteGraph {
    /// Returns the sink nodes of the graph.
    fn sink_nodes(&self) -> Vec<Self::NodeId> {
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
            .filter_map(
                |(node, visited)| {
                    if visited && !self.has_successors(node) { Some(node) } else { None }
                },
            )
            .collect()
    }
}

impl<G: MonoplexMonopartiteGraph + ?Sized> SinkNodes for G {}
