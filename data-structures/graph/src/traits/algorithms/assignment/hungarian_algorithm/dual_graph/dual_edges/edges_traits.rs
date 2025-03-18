//! Submodule implementing the traits relative to Edges for the `DualMatrix`
//! struct used in the context of the Hungarian algorithm.

use super::{DualEdges, DualMatrix};
use crate::traits::{Edges, WeightedEdges};

impl<'matrix, E: WeightedEdges + ?Sized> Edges for DualEdges<'matrix, E> {
    type Edge = E::Edge;
    type EdgeId = E::EdgeId;
    type SourceNodeId = E::SourceNodeId;
    type DestinationNodeId = E::DestinationNodeId;
    type Matrix = DualMatrix<'matrix, E::Matrix>;

    fn matrix(&self) -> &Self::Matrix {
        &self.matrix
    }
}

impl<'edges, E: WeightedEdges + ?Sized> From<&'edges E> for DualEdges<'edges, E> {
    fn from(edges: &'edges E) -> Self {
        DualEdges { matrix: DualMatrix::from(edges.matrix()) }
    }
}
