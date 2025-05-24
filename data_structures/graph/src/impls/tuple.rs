//! Submodule implementing graph-related traits for tuples.
use numeric_common_traits::prelude::{Number, PositiveInteger};

use crate::traits::{AttributedEdge, Edge};

impl<SourceNodeId: PositiveInteger, DestinationNodeId: PositiveInteger> Edge
    for (SourceNodeId, DestinationNodeId)
{
    type SourceNodeId = SourceNodeId;
    type DestinationNodeId = DestinationNodeId;

    fn source(&self) -> Self::SourceNodeId {
        self.0
    }

    fn destination(&self) -> Self::DestinationNodeId {
        self.1
    }
}

impl<SourceNodeId: PositiveInteger, DestinationNodeId: PositiveInteger, Weight: Number> Edge
    for (SourceNodeId, DestinationNodeId, Weight)
{
    type SourceNodeId = SourceNodeId;
    type DestinationNodeId = DestinationNodeId;

    fn source(&self) -> Self::SourceNodeId {
        self.0
    }

    fn destination(&self) -> Self::DestinationNodeId {
        self.1
    }
}

impl<SourceNodeId: PositiveInteger, DestinationNodeId: PositiveInteger, Weight: Number>
    AttributedEdge for (SourceNodeId, DestinationNodeId, Weight)
{
    type Attribute = Weight;

    fn attribute(&self) -> Self::Attribute {
        self.2
    }
}
