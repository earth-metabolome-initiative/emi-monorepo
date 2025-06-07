//! A submodule defining similarities.

/// Trait for calculating the similarity between two items.
pub trait ScalarSimilarity<Left, Right> {
    /// The type of the similarity.
    type Similarity;

    /// Calculate the similarity between two items.
    fn similarity(&self, left: &Left, right: &Right) -> Self::Similarity;
}
