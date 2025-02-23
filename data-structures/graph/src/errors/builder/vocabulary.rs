//! Error enumeration for the vocabulary builder.

use crate::traits::{Vocabulary, VocabularyBuilderOptions};

#[derive(Debug, Clone)]
/// Enum representing the possible errors that can occur when building a graph.
pub enum VocabularyBuilderError<V: Vocabulary> {
	/// Error that occurs when building a vocabulary.
	BuilderError(common_traits::builder::BuilderError<VocabularyBuilderOptions>),
	/// Whether the expected number of symbols was not reached or it was overreached.
	NumberOfSymbols {
		/// The expected number of symbols.
		expected: usize,
		/// The actual number of symbols.
		actual: usize,
	},
	/// A source symbol appeared more than once in the vocabulary and it is not allowed.
	RepeatedSourceSymbol(V::SourceSymbol),
	/// A destination symbol appeared more than once in the vocabulary and it is not allowed.
	RepeatedDestinationSymbol(V::DestinationSymbol),
	/// A source node did not respect the density requirements of the vocabulary.
	SparseSourceNode(V::SourceSymbol),
	/// A destination node did not respect the sorting requirements of the vocabulary.
	UnorderedDestinationNode(V::DestinationSymbol),
}

impl<V: Vocabulary> From<common_traits::builder::BuilderError<VocabularyBuilderOptions>> for VocabularyBuilderError<V> {
	fn from(e: common_traits::builder::BuilderError<VocabularyBuilderOptions>) -> Self {
		VocabularyBuilderError::BuilderError(e)
	}
}

impl<V: Vocabulary> core::error::Error for VocabularyBuilderError<V> {}

impl<V: Vocabulary> core::fmt::Display for VocabularyBuilderError<V> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			VocabularyBuilderError::BuilderError(e) => write!(f, "{e}"),
			VocabularyBuilderError::RepeatedSourceSymbol(e) => {
				write!(f, "Repeated source symbol: {e:?}")
			}
			VocabularyBuilderError::RepeatedDestinationSymbol(e) => {
				write!(f, "Repeated destination symbol: {e:?}")
			}
			VocabularyBuilderError::NumberOfSymbols { expected, actual } => {
				write!(
					f,
					"Expected number of symbols: {expected}, actual number of symbols: {actual}"
				)
			}
			VocabularyBuilderError::SparseSourceNode(e) => {
				write!(f, "Sparse source node: {e:?}")
			}
			VocabularyBuilderError::UnorderedDestinationNode(e) => {
				write!(f, "Unordered destination node: {e:?}")
			}
		}
	}
}