//! A submodule defining similarities.

use algebra::prelude::Number;

/// Trait for calculating the similarity between two items.
pub trait ScalarSimilarity<Other> {
    /// The type of the similarity.
    type Similarity: Number;

    /// Calculate the similarity between two items.
    fn similarity(&self, other: &Other) -> Self::Similarity;
}
