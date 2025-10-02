//! Implementation for the 'Lin' trait based on the algorithm implementation

use core::f64;

use functional_properties::prelude::ScalarSimilarity;

use crate::{
    prelude::{information_content::InformationContentError, resnik::ResnikResult},
    traits::{MonoplexMonopartiteGraph, Resnik, information_content::InformationContentResult},
};

/// Struct for the Lin similarity trait
#[derive(Debug, PartialEq)]
pub struct LinResult<'graph, G: ?Sized + MonoplexMonopartiteGraph>(ResnikResult<'graph, G>);

impl<'graph, G: ?Sized + MonoplexMonopartiteGraph> From<ResnikResult<'graph, G>>
    for LinResult<'graph, G>
{
    fn from(value: ResnikResult<'graph, G>) -> Self {
        Self(value)
    }
}

/// Trait providing the `Lin` method
///
/// # Reference
/// The implementation of the Lin similarity score as described in [An Information-Theoretic Definition of Similarity](https://www.cs.swarthmore.edu/~richardw/classes/cs65/f08/litreview/phyo.pdf)
pub trait Lin: Resnik {
    /// The method for applying the Lin
    ///
    /// # Arguments
    /// - `occurrences`: the number of times each node has been observed
    ///
    /// # Errors
    /// - If the graph is not a dag
    /// - If the occurrences are not equal
    fn lin(&self, occurrences: &[usize]) -> Result<LinResult<'_, Self>, InformationContentError> {
        Ok(self.resnik(occurrences)?.into())
    }
}

impl<'graph, G: ?Sized + MonoplexMonopartiteGraph> AsRef<ResnikResult<'graph, G>>
    for LinResult<'graph, G>
{
    fn as_ref(&self) -> &ResnikResult<'graph, G> {
        &self.0
    }
}

impl<'graph, G: ?Sized + MonoplexMonopartiteGraph> AsRef<InformationContentResult<'graph, G>>
    for LinResult<'graph, G>
{
    fn as_ref(&self) -> &InformationContentResult<'graph, G> {
        self.0.as_ref()
    }
}

impl<G> ScalarSimilarity<G::NodeId, G::NodeId> for LinResult<'_, G>
where
    G: MonoplexMonopartiteGraph,
{
    type Similarity = f64;
    fn similarity(&self, left: &G::NodeId, right: &G::NodeId) -> Self::Similarity {
        if left == right {
            return 1.0;
        }
        let ic: &InformationContentResult<G> = self.as_ref();
        let resnik: &ResnikResult<G> = self.as_ref();
        let left_ic = ic[*left];
        let right_ic = ic[*right];
        let resnik_score = resnik.similarity(left, right);
        // reminder: resnik score needs to always be less than max of left_ic/right_ic
        let mut denominator = left_ic + right_ic;
        if denominator < f64::EPSILON {
            denominator = f64::EPSILON;
        }
        (2.0 * resnik_score) / denominator
    }
}

impl<G> Lin for G where G: MonoplexMonopartiteGraph {}
