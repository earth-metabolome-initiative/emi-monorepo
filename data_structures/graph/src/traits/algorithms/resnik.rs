//! Submodule providing `Resnik` trait based on the algorithm implementation.
use functional_properties::prelude::ScalarSimilarity;

use crate::{
    prelude::information_content::InformationContentError,
    traits::{
        InformationContent, MonoplexMonopartiteGraph, information_content::InformationContentResult,
    },
};

/// Struct to provide methods to compute Resnik Similarity Score
#[derive(Debug, PartialEq)]
pub struct ResnikResult<'graph, G: ?Sized + MonoplexMonopartiteGraph>(
    InformationContentResult<'graph, G>,
);

impl<'graph, G: ?Sized + MonoplexMonopartiteGraph> AsRef<InformationContentResult<'graph, G>>
    for ResnikResult<'graph, G>
{
    fn as_ref(&self) -> &InformationContentResult<'graph, G> {
        &self.0
    }
}

impl<'graph, G: ?Sized + MonoplexMonopartiteGraph> From<InformationContentResult<'graph, G>>
    for ResnikResult<'graph, G>
{
    fn from(value: InformationContentResult<'graph, G>) -> Self {
        Self(value)
    }
}

impl<G> ScalarSimilarity<G::NodeId, G::NodeId> for ResnikResult<'_, G>
where
    G: MonoplexMonopartiteGraph,
{
    type Similarity = f64;
    fn similarity(&self, left: &G::NodeId, right: &G::NodeId) -> Self::Similarity {
        let mut max_score = 0.0;
        for root_node in self.as_ref().root_nodes() {
            if let InformationContentSearch::Both(score) =
                information_content_search(self.as_ref(), left, right, root_node)
                && max_score < score
            {
                max_score = score;
            }
        }
        max_score
    }
}

fn information_content_search<G>(
    information_content_result: &InformationContentResult<'_, G>,
    left: &G::NodeId,
    right: &G::NodeId,
    current_node: &G::NodeId,
) -> InformationContentSearch
where
    G: MonoplexMonopartiteGraph + ?Sized,
{
    // Base cases
    let mut state = if left == current_node {
        InformationContentSearch::Left
    } else if right == current_node {
        InformationContentSearch::Right
    } else {
        InformationContentSearch::NotFound
    };
    for successor in information_content_result.graph().successors(*current_node) {
        state = match (
            state,
            information_content_search(information_content_result, left, right, &successor),
        ) {
            (InformationContentSearch::Left, InformationContentSearch::Right)
            | (InformationContentSearch::Right, InformationContentSearch::Left) => {
                InformationContentSearch::Both(information_content_result[*current_node])
            }
            (InformationContentSearch::NotFound, other) => other,
            (InformationContentSearch::Both(current), InformationContentSearch::Both(other)) => {
                InformationContentSearch::Both(if current > other { current } else { other })
            }
            (_, InformationContentSearch::Both(other)) => InformationContentSearch::Both(other),
            (_, InformationContentSearch::NotFound)
            | (InformationContentSearch::Left, InformationContentSearch::Left)
            | (InformationContentSearch::Right, InformationContentSearch::Right)
            | (InformationContentSearch::Both(_), _) => continue,
        };
    }
    state
}

/// Enum for tracking IC Search possibilities
#[derive(Clone, Copy)]
enum InformationContentSearch {
    NotFound,
    Left,
    Right,
    Both(f64),
}

/// Trait providing the `Resnik` method
///
/// # Reference
/// The implementation of the Resnik score as described in [Using Information Content to Evaluate Semantic Similarity in a Taxonomy](https://arxiv.org/pdf/cmp-lg/9511007)
pub trait Resnik: MonoplexMonopartiteGraph {
    /// The method for applying the Resnik
    ///
    /// # Arguments
    /// - `occurrences`: the number of times each node has been observed
    ///
    /// # Errors
    /// - If the graph is not a dag
    /// - If the occurrences are not equal
    fn resnik(
        &self,
        occurrences: &[usize],
    ) -> Result<ResnikResult<'_, Self>, InformationContentError> {
        // Compute IC using information content method
        Ok(self.information_content(occurrences)?.into())
    }
}

impl<G> Resnik for G where G: MonoplexMonopartiteGraph {}
