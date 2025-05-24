//! Submodule defining the properties of an edge.

use core::fmt::Debug;

use numeric_common_traits::prelude::PositiveInteger;

/// Trait defining the properties of an edge.
pub trait Edge: Debug + Clone {
    /// Type of the source node identifier.
    type SourceNodeId: PositiveInteger;
    /// Type of the destination node identifier.
    type DestinationNodeId: PositiveInteger;

    /// Returns the identifier of the source node.
    fn source(&self) -> Self::SourceNodeId;

    /// Returns the identifier of the destination node.
    fn destination(&self) -> Self::DestinationNodeId;

    /// Returns whether the edge is a self-loop.
    fn is_self_loop(&self) -> bool
    where
        Self::SourceNodeId: PartialEq<Self::DestinationNodeId>,
    {
        self.source() == self.destination()
    }
}

/// Trait defining an attributed edge.
pub trait AttributedEdge: Edge {
    /// Type of the attribute.
    type Attribute;

    /// Returns the attribute of the edge.
    fn attribute(&self) -> Self::Attribute;
}
