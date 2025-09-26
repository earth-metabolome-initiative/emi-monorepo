//! Submodule providing `Resnik` trait
//! implementation.
use algebra::prelude::Kahn;
mod error;
pub use error::ResnikError;
use functional_properties::prelude::ScalarSimilarity;
use numeric_common_traits::prelude::IntoUsize;

use crate::{
    prelude::information_content::InformationContentError,
    traits::{
        InformationContent, MonoplexMonopartiteGraph, RootNodes, edges::Edges,
        information_content::InformationContentResult,
    },
};

/// Struct to provide methods to compute Resnik Similarity Score
#[derive(Debug, PartialEq)]
pub struct ResnikResult<'graph, G: ?Sized + MonoplexMonopartiteGraph> {
    /// The graph to be analyzed
    graph: &'graph G,
    /// The information content and max information content
    information_contents: InformationContentResult,
    /// Root nodes of the graph
    root_nodes: Vec<G::NodeId>,
}

impl<'graph, G> ScalarSimilarity<G::NodeId, G::NodeId> for ResnikResult<'_, G>
where
    G: MonoplexMonopartiteGraph,
{
    type Similarity = f64;
    fn similarity(&self, left: &G::NodeId, right: &G::NodeId) -> Self::Similarity {
        if left == right {
            return 1.0;
        }
        let mut max_score = 0.0;
        for root_node in &self.root_nodes {
            if let InformationContentSearch::BothNodesFound(score) =
                information_content_search(self, left, right, root_node)
                && max_score < score
            {
                max_score = score;
            }
        }
        max_score / self.information_contents.max_information_content
    }
}

fn information_content_search<'graph, G>(
    resnik_result: &ResnikResult<'_, G>,
    left: &G::NodeId,
    right: &G::NodeId,
    current_node: &G::NodeId,
) -> InformationContentSearch
where
    G: MonoplexMonopartiteGraph + ?Sized,
{
    // Base cases
    let mut state = if left == current_node {
        InformationContentSearch::LeftNodeFound
    } else if right == current_node {
        InformationContentSearch::RightNodeFound
    } else {
        InformationContentSearch::NotFound
    };
    for successor in resnik_result.graph.successors(*current_node) {
        state = match (state, information_content_search(resnik_result, left, right, &successor)) {
            (InformationContentSearch::LeftNodeFound, InformationContentSearch::RightNodeFound)
            | (InformationContentSearch::RightNodeFound, InformationContentSearch::LeftNodeFound) => {
                InformationContentSearch::BothNodesFound(
                    resnik_result.information_contents.information_contents
                        [current_node.into_usize()],
                )
            }
            (InformationContentSearch::NotFound, other) => other,
            (
                InformationContentSearch::BothNodesFound(current),
                InformationContentSearch::BothNodesFound(other),
            ) => {
                InformationContentSearch::BothNodesFound(if current > other {
                    current
                } else {
                    other
                })
            }
            (_, InformationContentSearch::BothNodesFound(other)) => {
                InformationContentSearch::BothNodesFound(other)
            }
            (_, InformationContentSearch::NotFound)
            | (InformationContentSearch::LeftNodeFound, InformationContentSearch::LeftNodeFound)
            | (
                InformationContentSearch::RightNodeFound,
                InformationContentSearch::RightNodeFound,
            )
            | (InformationContentSearch::BothNodesFound(_), _) => continue,
        };
    }
    state
}

/// Enum for tracking IC Search possibilities
#[derive(Clone, Copy)]
enum InformationContentSearch {
    NotFound,
    LeftNodeFound,
    RightNodeFound,
    BothNodesFound(f64),
}

/// Trait providing the `Resnik` method
/// TODO: Finish
pub trait Resnik: MonoplexMonopartiteGraph {
    /// The method for applying the Resnik
    ///
    /// # Errors
    /// - If the graph is not a dag
    /// - If the occurrences are not equal
    fn resnik(&self, occurrences: &[f64]) -> Result<ResnikResult<'_, Self>, ResnikError> {
        // Check whether the graph is a DAG (characterize by having no cycles)
        let _topological_ordering = self.edges().matrix().kahn()?;
        // Compute IC using information content method
        let information_content = self.information_content(occurrences).map_err(|e| {
            match e {
                InformationContentError::UnequalOccurrenceSize { expected, found } => {
                    ResnikError::UnequalOccurrenceSize { expected, found }
                }
                InformationContentError::NonFiniteOccurrence => ResnikError::NonFiniteOccurrence,
                InformationContentError::NegativeOccurrence => ResnikError::NegativeOccurrence,
                InformationContentError::NoOccurrencesAboveZero => {
                    ResnikError::NoOccurrencesAboveZero
                }
            }
        })?;

        // 3) Resnik still needs roots for the pairwise search
        let root_nodes = self.root_nodes();
        Ok(ResnikResult { graph: self, information_contents: information_content, root_nodes })
    }
}

impl<G> Resnik for G where G: MonoplexMonopartiteGraph {}
