//! Submodule defining the trait for Options for building a vocabulary.

use crate::traits::{GrowableVocabulary, Vocabulary};

/// Trait for Options for building a vocabulary.
pub trait VocabularyBuilder {
    /// The type of the vocabulary being built.
    type Vocabulary: GrowableVocabulary;
    /// The iterator of symbols.
    type Symbols: IntoIterator<
        Item = (
            <Self::Vocabulary as Vocabulary>::SourceSymbol,
            <Self::Vocabulary as Vocabulary>::DestinationSymbol,
        ),
    >;

    #[must_use]
    /// Set whether to ignore duplicated symbols.
    fn ignore_duplicates(self) -> Self;

    /// Returns whether to ignore duplicated symbols.
    fn should_ignore_duplicates(&self) -> bool;

    #[must_use]
    /// Set the expected number of symbols.
    ///
    /// # Arguments
    ///
    /// * `number_of_symbols` - The expected number of symbols.
    fn expected_number_of_symbols(self, number_of_symbols: usize) -> Self;

    /// Returns the expected number of symbols.
    fn get_expected_number_of_symbols(&self) -> Option<usize>;

    #[must_use]
    /// Set the iterator of symbols.
    ///
    /// # Arguments
    ///
    /// * `symbols` - The iterator of symbols.
    fn symbols(self, symbols: Self::Symbols) -> Self;
}
