//! Submodule providing the Hopcroft-Karp algorithm for the assignment problem.

mod partial_assignment;

use algebra::prelude::Number;
use partial_assignment::PartialAssignment;

use super::Assignment;
use crate::traits::MonoplexBipartiteGraph;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enumeration of the errors that might occur during the Hopcroft-Karp
/// algorithm.
pub enum HopcroftKarpError {
    /// The provided distance type is not large enough to be used in the
    /// algorithm for the provided graph.
    InsufficientDistanceType,
}

/// Trait providing the Hopcroft-Karp algorithm for the assignment problem.
pub trait HopcroftKarp<A, Distance = u16>: MonoplexBipartiteGraph
where
    A: for<'a> From<PartialAssignment<'a, Self, Distance>>
        + Assignment<LeftNodeId = Self::LeftNodeId, RightNodeId = Self::RightNodeId>,
    Distance: Number,
{
    /// Return the assignment as assigned by the Hopcroft-Karp algorithm.
    fn hopcroft_karp(&self) -> Result<A, HopcroftKarpError> {
        let mut partial_assignment = PartialAssignment::from(self);
        while partial_assignment.bfs()? {
            for left_node_id in self.left_node_ids() {
                if !partial_assignment.has_successor(left_node_id) {
                    partial_assignment.dfs(Some(left_node_id));
                }
            }
        }

        Ok(A::from(partial_assignment))
    }
}

impl<G, A, Distance> HopcroftKarp<A, Distance> for G
where
    G: MonoplexBipartiteGraph,
    Distance: Number,
    A: for<'a> From<PartialAssignment<'a, G, Distance>>
        + Assignment<LeftNodeId = G::LeftNodeId, RightNodeId = G::RightNodeId>,
{
}
