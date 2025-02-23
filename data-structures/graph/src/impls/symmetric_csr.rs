//! Submodule implementing Edges-related traits for [`SquaredSymmetricCSR2D`].

use crate::prelude::*;
use algebra::prelude::*;

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > Edges for SymmetricCSR2D<SparseIndex, Idx>
{
    type Edge = <Self as Matrix>::Coordinates;
    type SourceNodeId = Idx;
    type DestinationNodeId = Idx;
    type EdgeId = SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > DirectedEdges for SymmetricCSR2D<SparseIndex, Idx>
{
    type DirectedMatrix = Self;
    type NodeId = Idx;

    fn number_of_self_loops(&self) -> Idx {
        self.number_of_defined_diagonal_values()
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > TransposedDirectedEdges for SymmetricCSR2D<SparseIndex, Idx>
{
    type BiMatrix = Self;
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > UndirectedEdges for SymmetricCSR2D<SparseIndex, Idx>
{
    type SymmetricMatrix = Self;
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
        DE: DirectedEdges<NodeId = Idx>,
    > FromDirectedEdges<DE> for SymmetricCSR2D<SparseIndex, Idx>
where
    DE::Matrix: Symmetrize<Self>,
{
    fn from_directed_edges(edges: DE) -> Self {
        edges.matrix().symmetrize()
    }
}
