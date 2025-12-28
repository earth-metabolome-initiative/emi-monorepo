//! Submodule providing a general builder to build a generic graph.

use core::marker::PhantomData;

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

#[derive(Clone, Debug, thiserror::Error)]
/// Error type for the builder.
pub enum MonoplexBipartiteGraphBuilderError {
    /// A build error occurred.
    #[error("Missing attribute: {0}")]
    MissingAttribute(&'static str),
}

impl<G: MonoplexBipartiteGraph> GenericMonoplexBipartiteGraphBuilder<G>
where
    G: TryFrom<(G::LeftNodes, G::RightNodes, G::Edges), Error = MonoplexBipartiteGraphBuilderError>,
{
    /// Builds the graph.
    pub fn build(self) -> Result<G, MonoplexBipartiteGraphBuilderError> {
        G::try_from((
            self.left_nodes
                .ok_or(MonoplexBipartiteGraphBuilderError::MissingAttribute("left_nodes"))?,
            self.right_nodes
                .ok_or(MonoplexBipartiteGraphBuilderError::MissingAttribute("right_nodes"))?,
            self.edges.ok_or(MonoplexBipartiteGraphBuilderError::MissingAttribute("edges"))?,
        ))
    }
}
