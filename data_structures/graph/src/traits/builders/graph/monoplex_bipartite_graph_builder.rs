//! Submodule defining a builder trait for monoplex bipartite graph builder.

use super::{BipartiteGraphBuilder, MonoplexGraphBuilder};
use crate::traits::MonoplexBipartiteGraph;

/// Trait for building a monoplex bipartite graph.
pub trait MonoplexBipartiteGraphBuilder:
    MonoplexGraphBuilder<
        MonoplexGraph = <Self as MonoplexBipartiteGraphBuilder>::MonoplexBipartiteGraph,
    > + BipartiteGraphBuilder<
        BipartiteGraph = <Self as MonoplexBipartiteGraphBuilder>::MonoplexBipartiteGraph,
    >
{
    /// The graph type that the builder builds.
    type MonoplexBipartiteGraph: MonoplexBipartiteGraph;
}

impl<B> MonoplexBipartiteGraphBuilder for B
where
    B: MonoplexGraphBuilder
        + BipartiteGraphBuilder<BipartiteGraph = <B as MonoplexGraphBuilder>::MonoplexGraph>,
    B::MonoplexGraph: MonoplexBipartiteGraph,
    B::BipartiteGraph: MonoplexBipartiteGraph,
{
    type MonoplexBipartiteGraph = B::MonoplexGraph;
}
