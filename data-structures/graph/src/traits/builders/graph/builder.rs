//! Submodule defining the trait for Options for building a graph.

use core::fmt::Display;

use common_traits::prelude::Builder;

use crate::traits::Graph;

#[derive(Debug, Clone)]
/// Options for building a graph.
pub enum GraphBuilderOptions {
    /// The edges-list describing the edges in the graph.
    Edges,
    /// The vocabulary describing the sources in the graph.
    Sources,
    /// The vocabulary describing the destinations in the graph.
    Destinations,
}

impl Display for GraphBuilderOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            GraphBuilderOptions::Edges => write!(f, "edges"),
            GraphBuilderOptions::Sources => write!(f, "sources"),
            GraphBuilderOptions::Destinations => write!(f, "destinations"),
        }
    }
}

/// Trait for Options for building a graph.
pub trait DirectedGraphBuilder:
    Builder<
    Object = <Self as DirectedGraphBuilder>::Graph,
    Error = crate::errors::builder::graph::GraphBuilderError<<Self as DirectedGraphBuilder>::Graph>,
    Attribute = GraphBuilderOptions,
>
{
    /// The type of the graph to build.
    type Graph: Graph;

    /// Sets the sources of the graph.
    ///
    /// # Arguments
    ///
    /// * `sources` - The sources of the graph.
    ///
    fn sources(&mut self, sources: <Self::Graph as Graph>::Sources) -> &mut Self;

    /// Sets the destinations of the graph.
    ///
    /// # Arguments
    /// 
    /// * `destinations` - The destinations of the graph.
    /// 
    fn destinations(&mut self, destinations: <Self::Graph as Graph>::Destinations) -> &mut Self;

    /// Sets the edges of the graph.
    ///
    /// # Arguments
    ///
    /// * `edges` - The edges of the graph.
    ///
    fn edges(&mut self, edges: <Self::Graph as Graph>::Edges) -> &mut Self;
}
