//! Submodule providing the `CycleDetection` trait and its blanket
//! implementation.

use numeric_common_traits::prelude::IntoUsize;

use crate::traits::MonoplexMonopartiteGraph;

/// Struct to support cycle detection in a graph.
struct CycleDetector<'graph, G: MonoplexMonopartiteGraph + ?Sized> {
    /// The graph to be analyzed.
    graph: &'graph G,
    /// A vector to keep track of visited nodes.
    visited: Vec<bool>,
    /// A vector to keep track of the recursion stack.
    stack: Vec<G::NodeId>,
}

impl<'graph, G: MonoplexMonopartiteGraph + ?Sized> From<&'graph G> for CycleDetector<'graph, G> {
    fn from(graph: &'graph G) -> Self {
        CycleDetector {
            graph,
            visited: vec![false; graph.number_of_nodes().into_usize()],
            stack: Vec::new(),
        }
    }
}

impl<G: MonoplexMonopartiteGraph + ?Sized> CycleDetector<'_, G> {
    /// Recursive function to detect cycles in the graph.
    fn dfs(&mut self, node: G::NodeId) -> bool {
        if !self.visited[node.into_usize()] {
            self.visited[node.into_usize()] = true;
            self.stack.push(node);

            for successor_node_id in self.graph.successors(node) {
                if !self.visited[successor_node_id.into_usize()] && self.dfs(successor_node_id)
                    || self.stack.contains(&successor_node_id)
                {
                    return true;
                }
            }
        }

        self.stack.pop();
        false
    }
}

/// Trait providing the `has_cycle` method, which returns true if the graph
/// contains a cycle, and false otherwise. The cycle detection algorithm is
/// based on a depth-first search (DFS) approach.
pub trait CycleDetection: MonoplexMonopartiteGraph {
    /// Returns true if the graph contains a cycle, false otherwise.
    fn has_cycle(&self) -> bool {
        let mut cycle_detector = CycleDetector::from(self);
        for node in self.node_ids() {
            if cycle_detector.dfs(node) {
                return true;
            }
        }
        false
    }
}

impl<G: MonoplexMonopartiteGraph> CycleDetection for G {}
