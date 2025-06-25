//! Submodule defining a trait characterizing monoplex monopartite graphs.
//!
//! These graphs are characterized by the fact that:
//!
//! * They are monopartite, i.e., they have only one type of nodes.
//! * They are monoplex, i.e., they have only one type of edges.

use numeric_common_traits::into_usize::IntoUsize;

use super::{MonopartiteEdges, MonopartiteGraph, MonoplexGraph};

/// Trait defining the properties of monoplex monopartite graphs.
pub trait MonoplexMonopartiteGraph: MonoplexGraph<Edges = <Self as MonoplexMonopartiteGraph>::MonoplexMonopartiteEdges>
    + MonopartiteGraph<NodeId = <<Self as MonoplexMonopartiteGraph>::MonoplexMonopartiteEdges as MonopartiteEdges>::NodeId>
{
    /// The type of edges in the graph.
    type MonoplexMonopartiteEdges: MonopartiteEdges;

    /// Returns whether the graph has self-loops.
    fn has_self_loops(&self) -> bool {
        self.edges().has_self_loops()
    }

    /// Returns the number of self-loops in the graph.
    fn number_of_self_loops(&self) -> Self::NodeId {
        self.edges().number_of_self_loops()
    }

    /// Returns whether the current graph labelling follows a
    /// topological order, which means that for every directed edge (u, v),
    /// u comes before v in the ordering.
    fn is_topologically_sorted(&self) -> bool {
        self.sparse_coordinates().all(|(src, dst)| src < dst)
    }

    /// Returns the set of nodes reachable from the given source node.
    ///
    /// # Arguments
    ///
    /// * `source` - The identifier of the source node.
    fn successors_set(&self, source: Self::NodeId) -> Vec<Self::NodeId> {
        let mut visited_nodes = vec![false; self.number_of_nodes().into_usize()];

        let mut frontier = vec![source];
        let mut temporary_frontier = Vec::new();
        visited_nodes[source.into_usize()] = true;
        let mut reachable_nodes = Vec::new();

        while !frontier.is_empty() {
            for node in frontier.drain(..) {
                for successor in self.successors(node) {
                    if !visited_nodes[successor.into_usize()] {
                        visited_nodes[successor.into_usize()] = true;
                        temporary_frontier.push(successor);
                    }
                }
            }
            reachable_nodes.extend(temporary_frontier.iter().cloned());
            std::mem::swap(&mut frontier, &mut temporary_frontier);
            temporary_frontier.clear();
        }

        reachable_nodes.sort_unstable();

        reachable_nodes
    }

    /// Returns whether the provided source node can reach the destination node.
    fn has_path(
        &self,
        source: Self::NodeId,
        destination: Self::NodeId,
    ) -> bool {
        let mut visited_nodes = vec![false; self.number_of_nodes().into_usize()];

        let mut frontier = vec![source];
        let mut temporary_frontier = Vec::new();
        visited_nodes[source.into_usize()] = true;

        while !frontier.is_empty() {
            for node in frontier.drain(..) {
                for successor in self.successors(node) {
                    if successor == destination {
                        return true;
                    }
                    if !visited_nodes[successor.into_usize()] {
                        visited_nodes[successor.into_usize()] = true;
                        temporary_frontier.push(successor);
                    }
                }
            }
            std::mem::swap(&mut frontier, &mut temporary_frontier);
            temporary_frontier.clear();
        }

        false
    }

    /// Returns whether there exist a path from a provided source node to a target node,
    /// possing through the provided node.
    /// 
    /// # Arguments
    /// 
    /// * `source` - The identifier of the source node.
    /// * `destination` - The identifier of the destination node.
    /// * `passing_through` - The identifier of the node that must be passed through.
    /// 
    fn is_reachable_through(
        &self,
        source: Self::NodeId,
        destination: Self::NodeId,
        passing_through: Self::NodeId,
    ) -> bool {
        self.has_path(source, passing_through)
            && self.has_path(passing_through, destination)
    }
}

impl<G> MonoplexMonopartiteGraph for G
where
    G: MonopartiteGraph + MonoplexGraph,
    G::Edges: MonopartiteEdges<NodeId = G::NodeId>,
{
    type MonoplexMonopartiteEdges = G::Edges;
}
