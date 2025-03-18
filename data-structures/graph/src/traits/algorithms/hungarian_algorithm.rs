//! Submodule providing a trait providing an implementation of the Hungarian
//! algorithm.

use algebra::prelude::Zero;

use crate::traits::BipartiteWeightedMonoplexGraph;

mod augmenting_alternating_path;
mod dual_graph;
mod partial_assignment;

use augmenting_alternating_path::AugmentingAlternatingPath;
use dual_graph::Dual;
use partial_assignment::PartialAssignment;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that may occur when executing the Hungarian algorithm.
pub enum HungarianAlgorithmError {
    /// Temporary entry in the error enum.
    N,
}

impl core::fmt::Display for HungarianAlgorithmError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            HungarianAlgorithmError::N => write!(f, "N"),
        }
    }
}

/// Struct representing the resulting assignment of the Hungarian algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HungarianAssignment<'graph, G: BipartiteWeightedMonoplexGraph + ?Sized> {
    /// The associated graph.
    graph: &'graph G,
    /// The cost of the assignment.
    cost: G::Weight,
    /// The assignment itself.
    assignments: Vec<(G::LeftNodeId, G::RightNodeId)>,
}

impl<G: BipartiteWeightedMonoplexGraph + ?Sized> HungarianAssignment<'_, G> {
    /// Return the cost of the assignment.
    pub fn cost(&self) -> G::Weight {
        self.cost
    }

    /// Return the assignments.
    pub fn assignments(&self) -> &[(G::LeftNodeId, G::RightNodeId)] {
        &self.assignments
    }
}

/// Trait providing an implementation of the Hungarian algorithm.
pub trait HungarianAlgorithm: BipartiteWeightedMonoplexGraph {
    /// Return the assignment as assigned by the Hungarian algorithm.
    fn hungarian(&self) -> Result<HungarianAssignment<'_, Self>, HungarianAlgorithmError> {
        // If the graph is empty, we return an empty assignment.
        if self.is_empty() {
            return Ok(HungarianAssignment {
                graph: self,
                cost: Self::Weight::ZERO,
                assignments: vec![],
            });
        }

        // We start by initializing the dual solution.
        let mut dual: Dual<'_, Self> = Dual::from(self);
        // Next, we initialize the partial assignment, which currsponds
        // to the primal solution.
        let mut partial_assignment: PartialAssignment<Self> = PartialAssignment::from(&dual);

        // While the assignment is not complete
        while !partial_assignment.is_complete() {
            // We initialize the augmenting alternating paths.
            let mut augmenting_path: AugmentingAlternatingPath<Self> =
                AugmentingAlternatingPath::new(&dual, &partial_assignment);

            // While we have not identified an augmenting path
            let path_end: Self::RightNodeId = 'external: loop {
                while augmenting_path.has_unpropagated_labels() {
                    // We propagate the labels.
                    if let Some(path_end) =
                        augmenting_path.propagate_labels(&partial_assignment, &dual)
                    {
                        break 'external path_end;
                    }
                }
                // We update the dual weights.
                dual.update(&mut augmenting_path);
            };
            // We update the assignment by executing a new
            // primal iteration.
            partial_assignment.update(path_end, &augmenting_path);
        }

        Ok(HungarianAssignment { graph: self, cost: Self::Weight::ZERO, assignments: vec![] })
    }
}

impl<G: BipartiteWeightedMonoplexGraph + ?Sized> HungarianAlgorithm for G {}
