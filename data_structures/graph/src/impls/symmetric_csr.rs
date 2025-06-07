//! Submodule implementing Edges-related traits for [`SquaredSymmetricCSR2D`].

use algebra::prelude::*;
use numeric_common_traits::prelude::TryFromUsize;

use crate::prelude::*;

impl<M> Edges for SymmetricCSR2D<M>
where
    M: SizedSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
    M::RowIndex: TryFromUsize,
    M::SparseIndex: TryFromUsize,
{
    type Edge = <Self as Matrix>::Coordinates;
    type SourceNodeId = <Self as Matrix2D>::RowIndex;
    type DestinationNodeId = <Self as Matrix2D>::RowIndex;
    type EdgeId = <Self as SparseMatrix>::SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<M, DE: MonopartiteEdges> FromDirectedMonopartiteEdges<DE> for SymmetricCSR2D<M>
where
    M: SizedSparseMatrix2D<ColumnIndex = <Self as Matrix2D>::RowIndex>,
    DE::MonopartiteMatrix: Symmetrize<Self>,
    M::RowIndex: TryFromUsize,
    M::SparseIndex: TryFromUsize,
{
    fn from_directed_edges(edges: DE) -> Self {
        edges.matrix().symmetrize()
    }
}
