//! Submodule implementing Edges for `GenericBiMatrix2D`.

use algebra::prelude::*;
use num_traits::ConstZero;
use numeric_common_traits::prelude::TryFromUsize;

use crate::prelude::*;

impl<M, T> Edges for GenericBiMatrix2D<M, T>
where
    M: Matrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>
        + Edges<
            Edge = <M as Matrix>::Coordinates,
            SourceNodeId = <M as Matrix2D>::RowIndex,
            DestinationNodeId = <M as Matrix2D>::RowIndex,
            EdgeId = <M as SparseMatrix>::SparseIndex,
            Matrix = M,
        > + SizedRowsSparseMatrix2D,
    T: Matrix2D<RowIndex = <M as Matrix2D>::RowIndex, ColumnIndex = <M as Matrix2D>::ColumnIndex>
        + Edges<
            Edge = <M as Edges>::Edge,
            SourceNodeId = <M as Edges>::SourceNodeId,
            DestinationNodeId = <M as Edges>::DestinationNodeId,
            EdgeId = <M as Edges>::EdgeId,
            Matrix = T,
        > + SizedRowsSparseMatrix2D,
    M: TransposableMatrix2D<T>,
    <M as Matrix2D>::RowIndex: TryFromUsize,
    <M as SparseMatrix>::SparseIndex: TryFromUsize,
{
    type Edge = <M as Edges>::Edge;
    type SourceNodeId = <M as Edges>::SourceNodeId;
    type DestinationNodeId = <M as Edges>::DestinationNodeId;
    type EdgeId = <M as Edges>::EdgeId;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<M, T> Graph for GenericBiMatrix2D<M, T>
where
    M: Matrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>
        + Edges<
            Edge = <M as Matrix>::Coordinates,
            SourceNodeId = <M as Matrix2D>::RowIndex,
            DestinationNodeId = <M as Matrix2D>::RowIndex,
            EdgeId = <M as SparseMatrix>::SparseIndex,
            Matrix = M,
        > + SizedRowsSparseMatrix2D,
    T: Matrix2D<RowIndex = <M as Matrix2D>::RowIndex, ColumnIndex = <M as Matrix2D>::ColumnIndex>
        + Edges<
            Edge = <M as Edges>::Edge,
            SourceNodeId = <M as Edges>::SourceNodeId,
            DestinationNodeId = <M as Edges>::DestinationNodeId,
            EdgeId = <M as Edges>::EdgeId,
            Matrix = T,
        > + SizedRowsSparseMatrix2D,
    M: TransposableMatrix2D<T>,
    <M as Matrix2D>::RowIndex: TryFromUsize,
    <M as SparseMatrix>::SparseIndex: TryFromUsize,
{
    fn has_nodes(&self) -> bool {
        self.number_of_rows() > <M as Matrix2D>::RowIndex::ZERO
            && self.number_of_columns() > <M as Matrix2D>::RowIndex::ZERO
    }

    fn has_edges(&self) -> bool {
        self.number_of_defined_values() > <M as SparseMatrix>::SparseIndex::ZERO
    }
}

impl<M, T> MonoplexGraph for GenericBiMatrix2D<M, T>
where
    M: Matrix2D<ColumnIndex = <M as Matrix2D>::RowIndex>
        + Edges<
            Edge = <M as Matrix>::Coordinates,
            SourceNodeId = <M as Matrix2D>::RowIndex,
            DestinationNodeId = <M as Matrix2D>::RowIndex,
            EdgeId = <M as SparseMatrix>::SparseIndex,
            Matrix = M,
        > + SizedRowsSparseMatrix2D,
    T: Matrix2D<RowIndex = <M as Matrix2D>::RowIndex, ColumnIndex = <M as Matrix2D>::ColumnIndex>
        + Edges<
            Edge = <M as Edges>::Edge,
            SourceNodeId = <M as Edges>::SourceNodeId,
            DestinationNodeId = <M as Edges>::DestinationNodeId,
            EdgeId = <M as Edges>::EdgeId,
            Matrix = T,
        > + SizedRowsSparseMatrix2D,
    M: TransposableMatrix2D<T>,
    <M as Matrix2D>::RowIndex: TryFromUsize,
    <M as SparseMatrix>::SparseIndex: TryFromUsize,
{
    type Edge = <Self as Matrix>::Coordinates;
    type Edges = Self;

    fn edges(&self) -> &Self::Edges {
        self
    }
}
