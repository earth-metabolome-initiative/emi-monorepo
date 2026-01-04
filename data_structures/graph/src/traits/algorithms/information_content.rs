//! Submodule providing `Information Content` structs
//! and methods for use with algorithms such as `Resnik`
mod error;
use std::ops::Index;

use algebra::prelude::Kahn;
pub use error::InformationContentError;
use numeric_common_traits::prelude::IntoUsize;

use crate::traits::{MonoplexMonopartiteGraph, RootNodes, SingletonNodes, SinkNodes, edges::Edges};

/// Result of information content computation.
#[derive(Debug, PartialEq)]
pub struct InformationContentResult<'graph, G: ?Sized + MonoplexMonopartiteGraph> {
    /// The graph to be analyzed
    graph: &'graph G,
    /// Information content per node
    information_contents: Vec<f64>,
    /// Maximum information content across all nodes.
    max_information_content: f64,
    /// Root nodes of the graph
    root_nodes: Vec<G::NodeId>,
}

impl<G: ?Sized + MonoplexMonopartiteGraph> InformationContentResult<'_, G> {
    /// Returns a reference to underlying graph
    pub(crate) fn graph(&self) -> &G {
        self.graph
    }
    /// Returns a reference to the root nodes of the graph
    pub(crate) fn root_nodes(&self) -> &[G::NodeId] {
        &self.root_nodes
    }
}

impl<G: ?Sized + MonoplexMonopartiteGraph> Index<G::NodeId> for InformationContentResult<'_, G> {
    type Output = f64;
    fn index(&self, index: G::NodeId) -> &Self::Output {
        &self.information_contents[index.into_usize()]
    }
}

/// Trait for computing information content from propagated occurrences.
///
/// Notes:
/// - This module assumes byte/float-safe sums. It does not perform DAG
///   validation
pub trait InformationContent: MonoplexMonopartiteGraph {
    #[allow(clippy::cast_precision_loss)]
    /// Computes per-node information content
    /// # Errors
    /// - `UnequalOccurrenceSize` when occurrence lengths do not match the
    ///   expected size
    /// - `NonFiniteOccurrence` if any occurrence is not finite
    /// - `NegativeOccurrence` if any occurrence is negative
    /// - `NoOccurrencesAboveZero` if resulting ICs are all zero
    fn information_content(
        &self,
        occurrences: &[usize],
    ) -> Result<InformationContentResult<'_, Self>, InformationContentError> {
        // Check whether the graph is a DAG (characterize by having no cycles)
        let _topological_ordering = self.edges().matrix().kahn()?;
        // Validate occurrences length.
        let expected = self.number_of_nodes().into_usize();
        if occurrences.len() != expected {
            return Err(InformationContentError::UnequalOccurrenceSize {
                expected,
                found: occurrences.len(),
            });
        }

        for sink_node in self.sink_nodes().into_iter().chain(self.singleton_nodes()) {
            if occurrences[sink_node.into_usize()] == 0 {
                // raise error that sink_node has a zero occurrence
                return Err(InformationContentError::SinkNodeZeroOccurrence);
            }
        }

        // Propagate occurrences from all root nodes.
        let mut propagated_occurrences = vec![None::<usize>; expected];
        let root_nodes = self.root_nodes();
        for root_node in &root_nodes {
            propagate_occurrences(self, *root_node, occurrences, &mut propagated_occurrences);
        }

        // total information content, initialized to 0
        let mut total_occurrences: usize = 0;
        for occurrence in occurrences {
            total_occurrences = total_occurrences.saturating_add(*occurrence);
        }
        let mut max_information_content = 0.0;

        let information_contents = propagated_occurrences
            .into_iter()
            .map(|propagated_occurrence| {
                let information_content =
                    -(propagated_occurrence.unwrap() as f64 / total_occurrences as f64).ln();
                if information_content > max_information_content {
                    max_information_content = information_content;
                }
                information_content
            })
            .collect::<Vec<f64>>();

        Ok(InformationContentResult {
            information_contents,
            max_information_content,
            root_nodes,
            graph: self,
        })
    }
}

/// Helper: recursively propagates occurrences over successors
fn propagate_occurrences<G>(
    graph: &G,
    node: G::NodeId,
    occurrences: &[usize],
    propagated_occurrences: &mut [Option<usize>],
) -> usize
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
        propagated_occurrence =
            propagate_occurrences(graph, successor, occurrences, propagated_occurrences)
                .saturating_add(propagated_occurrence);
    }
    propagated_occurrences[node.into_usize()] = Some(propagated_occurrence);
    propagated_occurrence
}

impl<T> InformationContent for T where T: MonoplexMonopartiteGraph + ?Sized {}
