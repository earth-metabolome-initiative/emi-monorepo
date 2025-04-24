//! Submodule providing a general builder to build a generic graph.

use core::marker::PhantomData;

use common_traits::prelude::{Builder, BuilderError};

use crate::traits::{MonopartiteGraphBuilder, MonoplexGraphBuilder, MonoplexMonopartiteGraph};

#[derive(Clone)]
/// Basic builder for a generic graph.
pub struct GenericMonoplexMonopartiteGraphBuilder<G: MonoplexMonopartiteGraph> {
    /// The nodes of the graph.
    nodes: Option<G::Nodes>,
    /// The edges of the graph.
    edges: Option<G::Edges>,
    /// Phantom data to store the graph type.
    _graph: PhantomData<G>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum to define the different attributes that can be set for the graph.
pub enum MonoplexMonopartiteGraphBuilder {
    /// The nodes of the graph.
    Nodes,
    /// The edges of the graph.
    Edges,
}

impl core::fmt::Display for MonoplexMonopartiteGraphBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Nodes => write!(f, "nodes"),
            Self::Edges => write!(f, "edges"),
        }
    }
}

impl<G: MonoplexMonopartiteGraph> Default for GenericMonoplexMonopartiteGraphBuilder<G> {
    fn default() -> Self {
        Self { nodes: None, edges: None, _graph: PhantomData }
    }
}

impl<G: MonoplexMonopartiteGraph> MonopartiteGraphBuilder
    for GenericMonoplexMonopartiteGraphBuilder<G>
{
    type MonopartiteGraph = G;

    fn nodes(
        self,
        nodes: <Self::MonopartiteGraph as crate::prelude::MonopartiteGraph>::Nodes,
    ) -> Self {
        Self { nodes: Some(nodes), ..self }
    }
}

impl<G: MonoplexMonopartiteGraph> MonoplexGraphBuilder
    for GenericMonoplexMonopartiteGraphBuilder<G>
{
    type MonoplexGraph = G;

    fn edges(self, edges: <Self::MonoplexGraph as crate::prelude::MonoplexGraph>::Edges) -> Self {
        Self { edges: Some(edges), ..self }
    }
}

#[derive(Clone, Debug)]
/// Error type for the builder.
pub enum MonoplexMonopartiteGraphBuilderError {
    /// A build error occurred.
    BuildError(common_traits::prelude::BuilderError<MonoplexMonopartiteGraphBuilder>),
}

impl core::error::Error for MonoplexMonopartiteGraphBuilderError {}

impl core::fmt::Display for MonoplexMonopartiteGraphBuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BuildError(error) => <common_traits::prelude::BuilderError<
                MonoplexMonopartiteGraphBuilder,
            > as core::fmt::Display>::fmt(error, f),
        }
    }
}

impl From<common_traits::prelude::BuilderError<MonoplexMonopartiteGraphBuilder>>
    for MonoplexMonopartiteGraphBuilderError
{
    fn from(error: common_traits::prelude::BuilderError<MonoplexMonopartiteGraphBuilder>) -> Self {
        Self::BuildError(error)
    }
}

impl<G: MonoplexMonopartiteGraph> Builder for GenericMonoplexMonopartiteGraphBuilder<G>
where
    G: TryFrom<(G::Nodes, G::Edges), Error = MonoplexMonopartiteGraphBuilderError>,
{
    type Object = G;
    type Error = MonoplexMonopartiteGraphBuilderError;
    type Attribute = MonoplexMonopartiteGraphBuilder;

    fn build(self) -> Result<Self::Object, Self::Error> {
        G::try_from((
            self.nodes
                .ok_or(BuilderError::IncompleteBuild(MonoplexMonopartiteGraphBuilder::Nodes))?,
            self.edges
                .ok_or(BuilderError::IncompleteBuild(MonoplexMonopartiteGraphBuilder::Edges))?,
        ))
    }
}
