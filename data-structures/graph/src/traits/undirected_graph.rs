//! Subtype of directed graph where the edges are undirected.

/// Subtype of directed graph where the edges are undirected.
pub trait UndirectedGraph:
    super::DirectedGraph<
    SuccessorsIter = <Self as UndirectedGraph>::NeighborsIter,
    PredecessorsIter = <Self as UndirectedGraph>::NeighborsIter,
>
{
    /// Neighbors of a node.
    type NeighborsIter: Iterator<Item = Self::NodeId>;

    /// Returns the neighbors of the node with the given identifier.
    fn neighbors(&self, id: Self::NodeId) -> Self::NeighborsIter;
}

impl<G> super::DirectedGraph for G
where
    G: UndirectedGraph,
{
    type SuccessorsIter = <G as UndirectedGraph>::NeighborsIter;
    type PredecessorsIter = <G as UndirectedGraph>::NeighborsIter;

    fn successors(&self, id: Self::NodeId) -> Self::SuccessorsIter {
        self.neighbors(id)
    }

    fn predecessors(&self, id: Self::NodeId) -> Self::PredecessorsIter {
        self.neighbors(id)
    }
}
