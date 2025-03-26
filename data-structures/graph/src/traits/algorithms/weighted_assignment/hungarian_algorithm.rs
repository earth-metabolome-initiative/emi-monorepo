//! Submodule providing a trait providing an implementation of the Hungarian
//! algorithm.

use crate::traits::{BipartiteGraph, BipartiteWeightedMonoplexGraph, HopcroftKarp, Assignment};

mod augmenting_alternating_path;
mod dual_graph;
mod partial_assignment;

use augmenting_alternating_path::AugmentingAlternatingPath;
use dual_graph::Dual;
use partial_assignment::PartialAssignment;

use super::WeightedAssignment;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that may occur when executing the Hungarian algorithm.
pub enum HungarianAlgorithmError {
    /// Error that occurs when the graph has no edges.
    NoEdges,
    /// When the procedure failed to find a valid assignment.
    NoAssignment,
}

impl core::fmt::Display for HungarianAlgorithmError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            HungarianAlgorithmError::NoEdges => write!(f, "The graph has no edges."),
            HungarianAlgorithmError::NoAssignment => write!(f, "No valid assignment found."),
        }
    }
}

/// Trait providing an implementation of the Hungarian algorithm.
pub trait HungarianAlgorithm<A>: BipartiteWeightedMonoplexGraph
where
    A: WeightedAssignment + From<PartialAssignment<Self>>,
    Self: HopcroftKarp<
        Vec<(<Self as BipartiteGraph>::LeftNodeId, <Self as BipartiteGraph>::RightNodeId)>,
    >,
{
    /// Return the assignment as assigned by the Hungarian algorithm.
    ///
    /// # Errors
    ///
    /// * If the graph has no edges, an error is returned.
    fn hungarian(&self) -> Result<A, HungarianAlgorithmError> {
        // If the graph is empty, we return an empty assignment.
        if !self.has_nodes() {
            return Ok(Default::default());
        }

        // If the graph has nodes, but no edges, we return an error.
        if !self.has_edges() {
            return Err(HungarianAlgorithmError::NoEdges);
        }

        // We start by initializing the dual solution.
        let mut dual: Dual<'_, Self> = Dual::from(self);
        // Next, we initialize the partial assignment, which currsponds
        // to the primal solution.
        let mut partial_assignment: PartialAssignment<Self> = PartialAssignment::from(&dual);
        // We initialize the augmenting alternating paths.
        let mut augmenting_path: AugmentingAlternatingPath<Self> =
            AugmentingAlternatingPath::new(&dual, &partial_assignment);

        // While the assignment is not complete
        'outer: while !partial_assignment.is_complete(&self) {
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
                if !dual.update(&mut augmenting_path) {
                    println!("No dual update");
                    break 'outer;
                }
            };
            // We update the assignment by executing a new
            // primal iteration.
            partial_assignment.update(path_end, &augmenting_path);
        }

        if !partial_assignment.is_complete(&self) {
            println!(
                "Found {} assignments out of {}, {}, with {} possible edges to use",
                partial_assignment.number_of_assigned_nodes,
                self.number_of_non_singletons_in_left_partition(),
                self.number_of_right_nodes(),
                self.number_of_edges()
            );
            println!("{}", self.to_mb_dot());
            let assignment: Vec<(
                <Self as BipartiteGraph>::LeftNodeId,
                <Self as BipartiteGraph>::RightNodeId,
            )> = self.hopcroft_karp().unwrap();
            println!("{}", assignment.number_of_assigned_nodes());
            return Err(HungarianAlgorithmError::NoAssignment);
        }

        Ok(partial_assignment.into())
    }
}

impl<A, G> HungarianAlgorithm<A> for G
where
    G: BipartiteWeightedMonoplexGraph,
    A: WeightedAssignment + From<PartialAssignment<G>>,
{
}
