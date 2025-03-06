//! Submodule defining the trait for Options for building a vocabulary.

use common_traits::prelude::Builder;

use crate::traits::{GrowableVocabulary, Vocabulary};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Options for building a vocabulary.
pub enum VocabularyBuilderOptions {
    /// The symbol from which to load the vocabulary.
    Symbols,
}

impl core::fmt::Display for VocabularyBuilderOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            VocabularyBuilderOptions::Symbols => write!(f, "symbols"),
        }
    }
}

/// Trait for Options for building a vocabulary.
pub trait VocabularyBuilder:
    Builder<
    Object = <Self as VocabularyBuilder>::Vocabulary,
    Error = crate::errors::builder::vocabulary::VocabularyBuilderError<
        <Self as VocabularyBuilder>::Vocabulary,
    >,
    Attribute = VocabularyBuilderOptions,
>
{
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
