//! Submodule defining the properties of a edge in a graph.

use super::{numeric_identifier::NumericIdentifier, GraphRef};

/// A trait defining the properties of an edge in a graph.
pub trait EdgeRef<'graph> {
    /// The identifier of the edge.
    type Id: NumericIdentifier;
    /// The graph that the edge belongs to.
    type Graph: GraphRef<'graph, EdgeRef = Self>;

    /// Returns the identifier of the edge.
    fn id(&self) -> Self::Id;
}
