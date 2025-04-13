//! Submodule defining a builder trait for bipartite graphs.

use crate::traits::BipartiteGraph;

/// Trait for building a bipartite graph.
pub trait BipartiteGraphBuilder {
    /// The graph type that the builder builds.
    type BipartiteGraph: BipartiteGraph;

    #[must_use]
    /// Adds the left nodes partition to the graph.
    fn left_nodes(self, left_nodes: <Self::BipartiteGraph as BipartiteGraph>::LeftNodes) -> Self;

    #[must_use]
    /// Adds the right nodes partition to the graph.
    fn right_nodes(self, right_nodes: <Self::BipartiteGraph as BipartiteGraph>::RightNodes)
    -> Self;
}
