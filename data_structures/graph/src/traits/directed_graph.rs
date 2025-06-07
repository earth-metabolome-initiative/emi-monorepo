//! Submodule defining the properties of a directed graph.

use super::Edges;

/// Trait defining the properties of the directed edges of a graph.
pub trait DirectedEdges: Edges {}

impl<E: Edges> DirectedEdges for E {}

/// Trait defining the properties of a directed graph.
pub trait DirectedGraph: super::Graph {}

impl<G> DirectedGraph for G where G: super::Graph {}
