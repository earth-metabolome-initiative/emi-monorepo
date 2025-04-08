//! Submodule defining a builder trait for monoplex graphs.

use crate::traits::MonoplexGraph;

/// Trait for building a monoplex graph.
pub trait MonoplexGraphBuilder {
    /// The graph type that the builder builds.
    type MonoplexGraph: MonoplexGraph;

    #[must_use]
    /// Sets the edges of the graph.
    fn edges(self, edges: <Self::MonoplexGraph as MonoplexGraph>::Edges) -> Self;
}
