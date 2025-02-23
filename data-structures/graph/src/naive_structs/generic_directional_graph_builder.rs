//! Submodule providing a general builder to build a generic graph.

use common_traits::prelude::{Builder, BuilderError};
use core::marker::PhantomData;

use crate::{
    errors::builder::graph::GraphBuilderError,
    traits::{
        DirectedGraph, DirectedGraphBuilder, GraphBuilderOptions,
        UndirectedGraph, UndirectedGraphBuilder,
    },
};

/// Basic builder for a generic graph.
pub struct GenericDirectionalGraphBuilder<G: DirectedGraph> {
    /// The sources of the graph.
    nodes: Option<G::Sources>,
    /// The edges of the graph.
    edges: Option<G::Edges>,
    _graph: PhantomData<G>,
}

impl<G: DirectedGraph> Default for GenericDirectionalGraphBuilder<G> {
    fn default() -> Self {
        Self { nodes: None, edges: None, _graph: PhantomData }
    }
}

impl<G: DirectedGraph> DirectedGraphBuilder for GenericDirectionalGraphBuilder<G>
where
    G: TryFrom<(G::Sources, G::Edges), Error = GraphBuilderError<G>>,
{
    type Graph = G;

    fn nodes(mut self, nodes: <Self::Graph as DirectedGraph>::Nodes) -> Self {
        self.nodes = Some(nodes);
        self
    }

    fn edges(mut self, edges: <Self::Graph as DirectedGraph>::DirectedEdges) -> Self {
        self.edges = Some(edges);
        self
    }
}

impl<G: DirectedGraph> Builder for GenericDirectionalGraphBuilder<G>
where
    G: TryFrom<(G::Sources, G::Edges), Error = GraphBuilderError<G>>,
{
    type Object = G;
    type Error = GraphBuilderError<G>;
    type Attribute = GraphBuilderOptions;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let nodes = self.nodes.ok_or(BuilderError::IncompleteBuild {
            missing_attribute: GraphBuilderOptions::Sources,
        })?;
        let edges = self.edges.ok_or(BuilderError::IncompleteBuild {
            missing_attribute: GraphBuilderOptions::Edges,
        })?;

        G::try_from((nodes, edges))
    }
}

impl<G: UndirectedGraph> UndirectedGraphBuilder for GenericDirectionalGraphBuilder<G>
where
    G: TryFrom<(G::Nodes, G::UndirectedEdges), Error = GraphBuilderError<G>>,
{
    type UndirectedGraph = G;
}
