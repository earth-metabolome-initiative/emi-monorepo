//! Submodule defining a complete graph, i.e. a graph that for each possible
//! combination of source and destination nodes has an edge.

use super::{Edges, Graph};

/// Trait defining a complete edges set.
pub trait CompleteEdges: Edges {}

/// Trait defining a complete graph.
pub trait CompleteGraph: Graph<Edges = <Self as CompleteGraph>::CompleteEdges> {
    /// The type of the complete edges.
    type CompleteEdges: CompleteEdges;
}

impl<G> CompleteGraph for G
where
    G: Graph,
    G::Edges: CompleteEdges,
{
    type CompleteEdges = G::Edges;
}
