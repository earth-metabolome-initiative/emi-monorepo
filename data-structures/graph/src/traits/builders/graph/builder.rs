//! Submodule defining the trait for Options for building a graph.

use core::fmt::Display;

use common_traits::prelude::{basic, Builder};

use crate::traits::{DirectedGraph, Graph, UndirectedGraph};

#[basic]
/// Options for building a graph.
pub enum GraphBuilderOptions {
    /// The edges-list describing the edges in the graph.
    Edges,
    /// The vocabulary describing the sources in the graph.
    Sources,
    /// The vocabulary describing the destinations in the graph.
    Destinations,
    /// The vocabulary describing the nodes in the graph.
    Nodes,
}

impl Display for GraphBuilderOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            GraphBuilderOptions::Edges => write!(f, "edges"),
            GraphBuilderOptions::Sources => write!(f, "sources"),
            GraphBuilderOptions::Destinations => write!(f, "destinations"),
            GraphBuilderOptions::Nodes => write!(f, "nodes"),
        }
    }
}

/// Trait for Options for building a graph.
pub trait GraphBuilder:
    Builder<
    Object = <Self as GraphBuilder>::Graph,
    Error = crate::errors::builder::graph::GraphBuilderError<<Self as GraphBuilder>::Graph>,
    Attribute = GraphBuilderOptions,
>
{
    /// The type of the graph to build.
    type Graph: Graph;

    #[must_use]
    /// Sets the sources of the graph.
    ///
    /// # Arguments
    ///
    /// * `sources` - The sources of the graph.
    fn sources(self, sources: <Self::Graph as Graph>::Sources) -> Self;

    #[must_use]
    /// Sets the destinations of the graph.
    ///
    /// # Arguments
    ///
    /// * `destinations` - The destinations of the graph.
    fn destinations(self, destinations: <Self::Graph as Graph>::Destinations) -> Self;

    #[must_use]
    /// Sets the edges of the graph.
    ///
    /// # Arguments
    ///
    /// * `edges` - The edges of the graph.
    fn edges(self, edges: <Self::Graph as Graph>::Edges) -> Self;
}

/// Trait for creating a directed graph.
pub trait DirectedGraphBuilder:
    Builder<
    Object = <Self as DirectedGraphBuilder>::Graph,
    Error = crate::errors::builder::graph::GraphBuilderError<<Self as DirectedGraphBuilder>::Graph>,
    Attribute = GraphBuilderOptions,
>
{
    /// The type of the graph to build.
    type Graph: DirectedGraph;

    #[must_use]
    /// Sets the nodes of the graph.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The nodes of the graph.
    fn nodes(self, nodes: <Self::Graph as DirectedGraph>::Nodes) -> Self;

    #[must_use]
    /// Sets the edges of the graph.
    ///
    /// # Arguments
    ///
    /// * `edges` - The edges of the graph.
    fn edges(self, edges: <Self::Graph as DirectedGraph>::DirectedEdges) -> Self;
}

/// Trait for creating an undirected graph.
pub trait UndirectedGraphBuilder:
    DirectedGraphBuilder<Graph = <Self as UndirectedGraphBuilder>::UndirectedGraph>
{
    /// The type of the graph to build.
    type UndirectedGraph: UndirectedGraph;
}
