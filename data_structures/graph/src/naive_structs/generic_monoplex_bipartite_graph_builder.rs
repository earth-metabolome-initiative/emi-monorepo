//! Submodule providing a general builder to build a generic graph.

use core::marker::PhantomData;

use common_traits::{
    builder::IsCompleteBuilder,
    prelude::{Builder, BuilderError},
};

use crate::traits::{
    BipartiteGraph, BipartiteGraphBuilder, MonoplexBipartiteGraph, MonoplexGraphBuilder,
};

#[derive(Clone)]
/// Basic builder for a generic graph.
pub struct GenericMonoplexBipartiteGraphBuilder<G: MonoplexBipartiteGraph> {
    /// The left nodes of the graph.
    left_nodes: Option<G::LeftNodes>,
    /// The right nodes of the graph.
    right_nodes: Option<G::RightNodes>,
    /// The edges of the graph.
    edges: Option<G::Edges>,
    /// Phantom data to store the graph type.
    _graph: PhantomData<G>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum to define the different attributes that can be set for the graph.
pub enum MonoplexBipartiteGraphBuilder {
    /// The left nodes of the graph.
    LeftNodes,
    /// The right nodes of the graph.
    RightNodes,
    /// The edges of the graph.
    Edges,
}

impl core::fmt::Display for MonoplexBipartiteGraphBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LeftNodes => write!(f, "left nodes"),
            Self::RightNodes => write!(f, "right nodes"),
            Self::Edges => write!(f, "edges"),
        }
    }
}

impl<G: MonoplexBipartiteGraph> Default for GenericMonoplexBipartiteGraphBuilder<G> {
    fn default() -> Self {
        Self { left_nodes: None, right_nodes: None, edges: None, _graph: PhantomData }
    }
}

impl<G: MonoplexBipartiteGraph> BipartiteGraphBuilder for GenericMonoplexBipartiteGraphBuilder<G> {
    type BipartiteGraph = G;

    fn left_nodes(self, left_nodes: <Self::BipartiteGraph as BipartiteGraph>::LeftNodes) -> Self {
        Self { left_nodes: Some(left_nodes), ..self }
    }

    fn right_nodes(
        self,
        right_nodes: <Self::BipartiteGraph as BipartiteGraph>::RightNodes,
    ) -> Self {
        Self { right_nodes: Some(right_nodes), ..self }
    }
}

impl<G: MonoplexBipartiteGraph> MonoplexGraphBuilder for GenericMonoplexBipartiteGraphBuilder<G> {
    type MonoplexGraph = G;

    fn edges(self, edges: <Self::MonoplexGraph as crate::prelude::MonoplexGraph>::Edges) -> Self {
        Self { edges: Some(edges), ..self }
    }
}

#[derive(Clone, Debug)]
/// Error type for the builder.
pub enum MonoplexBipartiteGraphBuilderError {
    /// A build error occurred.
    BuildError(common_traits::prelude::BuilderError<MonoplexBipartiteGraphBuilder>),
}

impl core::error::Error for MonoplexBipartiteGraphBuilderError {}

impl core::fmt::Display for MonoplexBipartiteGraphBuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BuildError(error) => <common_traits::prelude::BuilderError<
                MonoplexBipartiteGraphBuilder,
            > as core::fmt::Display>::fmt(error, f),
        }
    }
}

impl From<common_traits::prelude::BuilderError<MonoplexBipartiteGraphBuilder>>
    for MonoplexBipartiteGraphBuilderError
{
    fn from(error: common_traits::prelude::BuilderError<MonoplexBipartiteGraphBuilder>) -> Self {
        Self::BuildError(error)
    }
}

impl<G: MonoplexBipartiteGraph> IsCompleteBuilder for GenericMonoplexBipartiteGraphBuilder<G>
where
    G: TryFrom<(G::LeftNodes, G::RightNodes, G::Edges), Error = MonoplexBipartiteGraphBuilderError>,
{
    fn is_complete(&self) -> bool {
        self.left_nodes.is_some() && self.right_nodes.is_some() && self.edges.is_some()
    }
}

impl<G: MonoplexBipartiteGraph> Builder for GenericMonoplexBipartiteGraphBuilder<G>
where
    G: TryFrom<(G::LeftNodes, G::RightNodes, G::Edges), Error = MonoplexBipartiteGraphBuilderError>,
{
    type Object = G;
    type Error = MonoplexBipartiteGraphBuilderError;
    type Attribute = MonoplexBipartiteGraphBuilder;

    fn build(self) -> Result<Self::Object, Self::Error> {
        G::try_from((
            self.left_nodes
                .ok_or(BuilderError::IncompleteBuild(MonoplexBipartiteGraphBuilder::LeftNodes))?,
            self.right_nodes
                .ok_or(BuilderError::IncompleteBuild(MonoplexBipartiteGraphBuilder::RightNodes))?,
            self.edges
                .ok_or(BuilderError::IncompleteBuild(MonoplexBipartiteGraphBuilder::Edges))?,
        ))
    }
}
