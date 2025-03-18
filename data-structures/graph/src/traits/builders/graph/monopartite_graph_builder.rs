//! Submodule defining a builder trait for monopartite graphs.

use crate::traits::MonopartiteGraph;

/// Trait for building a monoparite graph.
pub trait MonopartiteGraphBuilder {
    /// The graph type that the builder builds.
    type MonopartiteGraph: MonopartiteGraph;

    #[must_use]
    /// Sets the nodes of the graph.
    fn nodes(self, nodes: <Self::MonopartiteGraph as MonopartiteGraph>::Nodes) -> Self;
}
