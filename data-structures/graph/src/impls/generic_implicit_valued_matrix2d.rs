//! Implementation of the [`Edges`] trait for
//! [`GenericImplicitValuedMatrix2D`](algebra::prelude::GenericImplicitValuedMatrix2D).

use algebra::prelude::{
    GenericImplicitValuedMatrix2D, Matrix2D, Matrix2DRef, Number, SparseMatrix, SparseMatrix2D,
    TryFromUsize, Zero,
};

use crate::traits::{BidirectionalVocabulary, BipartiteGraph, Edges, Graph, MonoplexGraph};

impl<M, Map, Value> Edges for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    Value: Number,
    M: SparseMatrix2D,
    M::RowIndex: TryFromUsize,
    M::ColumnIndex: TryFromUsize,
    M::SparseIndex: TryFromUsize,
    Map: Fn(M::Coordinates) -> Value,
{
    type Edge = (M::RowIndex, M::ColumnIndex, Value);
    type SourceNodeId = M::RowIndex;
    type DestinationNodeId = M::ColumnIndex;
    type EdgeId = M::SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<M, Map, Value> Graph for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    M::RowIndex: TryFromUsize,
    M::ColumnIndex: TryFromUsize,
    M::SparseIndex: TryFromUsize,
    Value: Number,
    Map: Fn(M::Coordinates) -> Value,
{
    fn has_nodes(&self) -> bool {
        self.number_of_rows() > M::RowIndex::ZERO && self.number_of_columns() > M::ColumnIndex::ZERO
    }

    fn has_edges(&self) -> bool {
        self.number_of_defined_values() > M::SparseIndex::ZERO
    }
}

impl<M, Map, Value> MonoplexGraph for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    M::RowIndex: TryFromUsize,
    M::ColumnIndex: TryFromUsize,
    M::SparseIndex: TryFromUsize,
    Value: Number,
    Map: Fn(M::Coordinates) -> Value,
{
    type Edge = (M::RowIndex, M::ColumnIndex, Value);
    type Edges = Self;

    fn edges(&self) -> &Self::Edges {
        self
    }
}

impl<M, Map, Value> BipartiteGraph for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D + Matrix2DRef,
    M::SparseIndex: TryFromUsize,
    M::RowIndex: TryFromUsize
        + BidirectionalVocabulary<SourceSymbol = M::RowIndex, DestinationSymbol = M::RowIndex>,
    M::ColumnIndex: TryFromUsize
        + BidirectionalVocabulary<SourceSymbol = M::ColumnIndex, DestinationSymbol = M::ColumnIndex>,
    Value: Number,
    Map: Fn(M::Coordinates) -> Value,
{
    type LeftNodeId = M::RowIndex;
    type RightNodeId = M::ColumnIndex;
    type LeftNodeSymbol = M::RowIndex;
    type RightNodeSymbol = M::ColumnIndex;
    type LeftNodes = M::RowIndex;
    type RightNodes = M::ColumnIndex;

    fn left_nodes_vocabulary(&self) -> &Self::LeftNodes {
        self.number_of_rows_ref()
    }

    fn right_nodes_vocabulary(&self) -> &Self::RightNodes {
        self.number_of_columns_ref()
    }
}
