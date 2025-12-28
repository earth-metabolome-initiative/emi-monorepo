//! Error enumeration for the vocabulary builder.

use crate::traits::Vocabulary;

#[derive(Debug, Clone, thiserror::Error)]
/// Enum representing the possible errors that can occur when building a graph.
pub enum VocabularyBuilderError<V: Vocabulary> {
    /// Error that occurs when building a vocabulary.
    #[error("Missing builder attribute: {0}")]
    MissingAttribute(&'static str),
    #[error("Unexpected number of symbols: expected {expected}, got {actual}")]
    /// Whether the expected number of symbols was not reached or it was
    /// overreached.
    NumberOfSymbols {
        /// The expected number of symbols.
        expected: usize,
        /// The actual number of symbols.
        actual: usize,
    },
    #[error("Repeated source symbol: {0:?}")]
    /// A source symbol appeared more than once in the vocabulary and it is not
    /// allowed.
    RepeatedSourceSymbol(V::SourceSymbol),
    #[error("Repeated destination symbol: {0:?}")]
    /// A destination symbol appeared more than once in the vocabulary and it is
    /// not allowed.
    RepeatedDestinationSymbol(V::DestinationSymbol),
    #[error("Sparse source node: {0:?}")]
    /// A source node did not respect the density requirements of the
    /// vocabulary.
    SparseSourceNode(V::SourceSymbol),
    #[error("Unordered destination node: {0:?}")]
    /// A destination node did not respect the sorting requirements of the
    /// vocabulary.
    UnorderedDestinationNode(V::DestinationSymbol),
}
