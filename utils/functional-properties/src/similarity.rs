//! A submodule defining similarities.

use algebra::prelude::Number;

/// Trait for calculating the similarity between two items.
pub trait ScalarSimilarity<Left, Right> {
    /// The type of the similarity.
    type Similarity: Number;

    /// Calculate the similarity between two items.
    fn similarity(&self, left: &Left, right: &Right) -> Self::Similarity;
}
