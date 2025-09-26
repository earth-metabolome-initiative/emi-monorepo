//! Submodule providing `Information Content` structs
//! and methods for use with algorithms such as `Resnik`
mod error;
pub use error::InformationContentError;
use numeric_common_traits::prelude::IntoUsize;

use crate::traits::{MonoplexMonopartiteGraph, RootNodes};

/// Result of information content computation.
#[derive(Debug, PartialEq)]
pub struct InformationContentResult {
    /// Information content per node
    pub information_contents: Vec<f64>,
    /// Maximum information content across all nodes.
    pub max_information_content: f64,
}

/// Trait for computing information content from propagated occurrences.
///
/// Notes:
/// - This module assumes byte/float-safe sums. It does not perform DAG
///   validation
pub trait InformationContent: MonoplexMonopartiteGraph {
    /// Computes per-node information content
    /// # Errors
    /// - `UnequalOccurrenceSize` when occurrence lengths do not match the
    ///   expected size
    /// - `NonFiniteOccurrence` if any occurrence is not finite
    /// - `NegativeOccurrence` if any occurrence is negative
    /// - `NoOccurrencesAboveZero` if resulting ICs are all zero
    fn information_content(
        &self,
        occurrences: &[f64],
    ) -> Result<InformationContentResult, InformationContentError> {
        // Validate occurrences length.
        let expected = self.number_of_nodes().into_usize();
        if occurrences.len() != expected {
            return Err(InformationContentError::UnequalOccurrenceSize {
                expected,
                found: occurrences.len(),
            });
        }

        // Validate values for being finite and more than 0.
        for &occurrence in occurrences {
            if !occurrence.is_finite() {
                return Err(InformationContentError::NonFiniteOccurrence);
            }
            if occurrence < 0.0 {
                return Err(InformationContentError::NegativeOccurrence);
            }
        }

        // Propagate occurrences from all root nodes.
        let mut propagated_occurrences = vec![None::<f64>; expected];
        for root_node in self.root_nodes() {
            propagate_occurrences(self, root_node, occurrences, &mut propagated_occurrences);
        }

        // Maximum information content, initialized to 0
        let total_occurrences: f64 = occurrences.iter().sum();
        let mut max_information_content = 0.0;
        let information_contents = propagated_occurrences
            .into_iter()
            .map(|propagated_occurrence| {
                let propagated_occurrence = propagated_occurrence.unwrap_or_default();
                if propagated_occurrence == 0.0 {
                    0.0
                } else {
                    let information_content = -(propagated_occurrence / total_occurrences).ln();
                    if information_content > max_information_content {
                        max_information_content = information_content;
                    }
                    information_content
                }
            })
            .collect::<Vec<f64>>();

        if max_information_content == 0.0 {
            return Err(InformationContentError::NoOccurrencesAboveZero);
        }

        Ok(InformationContentResult { information_contents, max_information_content })
    }
}

/// Helper: recursively propagates occurrences over successors
fn propagate_occurrences<G>(
    graph: &G,
    node: G::NodeId,
    occurrences: &[f64],
    propagated_occurrences: &mut [Option<f64>],
) -> f64
where
    G: MonoplexMonopartiteGraph + ?Sized,
{
    // Check whether node has been visited by propagation
    if let Some(propagated_occurrence) = propagated_occurrences[node.into_usize()] {
        return propagated_occurrence;
    }
    let mut propagated_occurrence = occurrences[node.into_usize()];
    // Recursively call propagate_occurrences()
    for successor in graph.successors(node) {
        propagated_occurrence +=
            propagate_occurrences(graph, successor, occurrences, propagated_occurrences);
    }
    propagated_occurrences[node.into_usize()] = Some(propagated_occurrence);
    propagated_occurrence
}

impl<T> InformationContent for T where T: MonoplexMonopartiteGraph + ?Sized {}
