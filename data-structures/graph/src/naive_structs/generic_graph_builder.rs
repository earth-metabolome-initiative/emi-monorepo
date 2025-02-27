//! Submodule providing a general builder to build a generic graph.

use core::marker::PhantomData;

use common_traits::prelude::{Builder, BuilderError};

use crate::{
    errors::builder::graph::GraphBuilderError,
    traits::{Graph, GraphBuilder, GraphBuilderOptions},
};

/// Basic builder for a generic graph.
pub struct GenericGraphBuilder<G: Graph> {
    /// The sources of the graph.
    sources: Option<G::Sources>,
    /// The destinations of the graph.
    destinations: Option<G::Destinations>,
    /// The edges of the graph.
    edges: Option<G::Edges>,
    _graph: PhantomData<G>,
}

impl<G: Graph> Default for GenericGraphBuilder<G> {
    fn default() -> Self {
        Self { sources: None, destinations: None, edges: None, _graph: PhantomData }
    }
}

impl<G: Graph> GraphBuilder for GenericGraphBuilder<G>
where
    G: TryFrom<(G::Sources, G::Destinations, G::Edges), Error = GraphBuilderError<G>>,
{
    type Graph = G;

    fn sources(mut self, sources: <Self::Graph as crate::prelude::Graph>::Sources) -> Self {
        self.sources = Some(sources);
        self
    }

    fn destinations(
        mut self,
        destinations: <Self::Graph as crate::prelude::Graph>::Destinations,
    ) -> Self {
        self.destinations = Some(destinations);
        self
    }

    fn edges(mut self, edges: <Self::Graph as Graph>::Edges) -> Self {
        self.edges = Some(edges);
        self
    }
}

impl<G: Graph> Builder for GenericGraphBuilder<G>
where
    G: TryFrom<(G::Sources, G::Destinations, G::Edges), Error = GraphBuilderError<G>>,
{
    type Object = G;
    type Error = GraphBuilderError<G>;
    type Attribute = GraphBuilderOptions;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let sources = self.sources.ok_or(BuilderError::IncompleteBuild {
            missing_attribute: GraphBuilderOptions::Sources,
        })?;
        let destinations = self.destinations.ok_or(BuilderError::IncompleteBuild {
            missing_attribute: GraphBuilderOptions::Destinations,
        })?;
        let edges = self.edges.ok_or(BuilderError::IncompleteBuild {
            missing_attribute: GraphBuilderOptions::Edges,
        })?;

        G::try_from((sources, destinations, edges))
    }
}
