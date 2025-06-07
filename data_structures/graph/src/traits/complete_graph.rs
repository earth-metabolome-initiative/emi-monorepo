//! Submodule defining a complete graph, i.e. a graph that for each possible
//! combination of source and destination nodes has an edge.

use super::Graph;

/// Trait defining a complete graph.
pub trait CompleteGraph: Graph {}

impl<G> CompleteGraph for G where G: Graph {}
