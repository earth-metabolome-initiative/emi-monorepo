//! Submodule providing the `TopologicalSorting` trait and its blanket
//! implementation.

use algebra::prelude::{IntoUsize, One, Zero};

use crate::traits::MonoplexMonopartiteGraph;

/// Error enumeration for the topological sorting algorithm.
#[derive(Debug, Clone, PartialEq)]
pub enum TopologicalSortingError {
    /// Error when some nodes are not reachable from the starting nodes.
    UnreachableNodes,
}

impl std::fmt::Display for TopologicalSortingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopologicalSortingError::UnreachableNodes => {
                write!(f, "Some nodes are not reachable from the starting nodes")
            }
        }
    }
}

impl std::error::Error for TopologicalSortingError {}

/// Trait defining the topological sorting algorithm.
pub trait TopologicalSorting: MonoplexMonopartiteGraph {
    /// Returns whether the current graph labelling follows a
    /// topological order, which means that for every directed edge (u, v),
    /// u comes before v in the ordering.
    fn is_topologically_sorted(&self) -> bool {
        self.sparse_coordinates().all(|(src, dst)| src <= dst)
    }

    /// Returns the topological order of the graph from the provided node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to start the topological sort from.
    ///
    /// # Returns
    ///
    /// The topological order of the graph, starting from the provided node.
    ///
    /// # Errors
    ///
    /// * `TopologicalSortingError::UnreachableNodes` - If some nodes are not
    ///   reachable from the root nodes.
    fn topological_sort_from_node(
        &self,
        node: Self::NodeId,
    ) -> Result<Vec<Self::NodeId>, TopologicalSortingError> {
        self.topological_sort_from_nodes(&[node])
    }

    /// Returns the topological order of the graph from the provided nodes.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The nodes to start the topological sort from.
    ///
    /// # Returns
    ///
    /// Vector with the new positions of the nodes in the topological order.
    ///
    /// # Errors
    ///
    /// * `TopologicalSortingError::UnreachableNodes` - If some nodes are not
    ///   reachable from the root nodes.
    fn topological_sort_from_nodes(
        &self,
        nodes: &[Self::NodeId],
    ) -> Result<Vec<Self::NodeId>, TopologicalSortingError> {
        if !self.has_nodes() {
            return Ok(Vec::new());
        }

        if nodes.is_empty() {
            return Err(TopologicalSortingError::UnreachableNodes);
        }

        let mut number_of_visited_nodes = Self::NodeId::ZERO;

        let mut topological_order = vec![None; self.number_of_nodes().into_usize()];

        let mut frontier = nodes.to_vec();
        let mut temporary_frontier = Vec::new();

        while !frontier.is_empty() {
            temporary_frontier.clear();
            for node in frontier.drain(..) {
                if topological_order[node.into_usize()].is_none() {
                    topological_order[node.into_usize()] = Some(number_of_visited_nodes);
                    number_of_visited_nodes += Self::NodeId::ONE;
                }

                temporary_frontier.extend(self.successors(node).filter(|successor_node_id| {
                    topological_order[successor_node_id.into_usize()].is_none()
                }));
            }

            core::mem::swap(&mut frontier, &mut temporary_frontier);
        }

        if number_of_visited_nodes < self.number_of_nodes() {
            return Err(TopologicalSortingError::UnreachableNodes);
        }

        Ok(topological_order.into_iter().flatten().collect())
    }
}

impl<G: MonoplexMonopartiteGraph> TopologicalSorting for G {}
