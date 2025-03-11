//! Submodule providing the traits for a generic graph that has weighted edges.

use algebra::prelude::{ImplicitValuedMatrix, Number, ValuedMatrix};

use super::{AttributedEdge, Edges, Graph};

/// Trait defining a weighted edge.
pub trait WeightedEdge: AttributedEdge<Attribute = Self::Weight> {
    /// Type of the weight.
    type Weight: Number;

    /// Returns the weight of the edge.
    fn weight(&self) -> Self::Weight;
}

impl<E> WeightedEdge for E
where
    E: AttributedEdge,
    E::Attribute: Number,
{
    type Weight = E::Attribute;

    fn weight(&self) -> Self::Weight {
        self.attribute()
    }
}

/// Trait defining an edge data structure that has weighted edges.
pub trait WeightedEdges:
    Edges<
    Edge = <Self as WeightedEdges>::WeightedEdge,
    Matrix = <Self as WeightedEdges>::WeightedMatrix,
>
{
    /// The type of the weight.
    type WeightedEdge: WeightedEdge;
    /// The type of the underlying matrix.
    type WeightedMatrix: ImplicitValuedMatrix<Value = <Self::WeightedEdge as WeightedEdge>::Weight>;
}

impl<E> WeightedEdges for E
where
    E: Edges,
    E::Edge: WeightedEdge,
    E::Matrix: ImplicitValuedMatrix<Value = <E::Edge as WeightedEdge>::Weight>,
{
    type WeightedEdge = E::Edge;
    type WeightedMatrix = E::Matrix;
}

/// Trait defining a graph that has weighted edges.
pub trait WeightedGraph: Graph<Edges = <Self as WeightedGraph>::WeightedEdges> {
    /// The type of the weighted edges.
    type WeightedEdges: WeightedEdges;
}

impl<G> WeightedGraph for G
where
    G: Graph,
    G::Edges: WeightedEdges,
{
    type WeightedEdges = G::Edges;
}
