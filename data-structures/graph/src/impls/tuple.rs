//! Submodule implementing graph-related traits for tuples.
use algebra::prelude::PositiveInteger;

use crate::traits::Edge;

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
