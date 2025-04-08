//! Submodule defining a builder trait for monoplex monopartite graph builder.

use super::{MonopartiteGraphBuilder, MonoplexGraphBuilder};
use crate::traits::MonoplexMonopartiteGraph;

/// Trait for building a monoplex monopartite graph.
pub trait MonoplexMonopartiteGraphBuilder:
    MonoplexGraphBuilder<
        MonoplexGraph = <Self as MonoplexMonopartiteGraphBuilder>::MonoplexMonopartiteGraph,
    > + MonopartiteGraphBuilder<
        MonopartiteGraph = <Self as MonoplexMonopartiteGraphBuilder>::MonoplexMonopartiteGraph,
    >
{
    /// The graph type that the builder builds.
    type MonoplexMonopartiteGraph: MonoplexMonopartiteGraph;
}

impl<B> MonoplexMonopartiteGraphBuilder for B
where
    B: MonoplexGraphBuilder
        + MonopartiteGraphBuilder<MonopartiteGraph = <B as MonoplexGraphBuilder>::MonoplexGraph>,
    B::MonoplexGraph: MonoplexMonopartiteGraph,
    B::MonopartiteGraph: MonoplexMonopartiteGraph,
{
    type MonoplexMonopartiteGraph = B::MonoplexGraph;
}
