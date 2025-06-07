//! A generic vocabulary builder that can be used to build a vocabulary for any
//! type of graph.

use common_traits::prelude::Builder;

use crate::traits::{GrowableVocabulary, Vocabulary, VocabularyBuilder, VocabularyBuilderOptions};

#[derive(Clone)]
/// A generic vocabulary builder that can be used to build a vocabulary for any
/// type of graph.
pub struct GenericVocabularyBuilder<Symbols, Vocabulary> {
    /// The symbols to build the vocabulary from.
    symbols: Option<Symbols>,
    /// The expected number of symbols.
    expected_number_of_symbols: Option<usize>,
    /// Whether to ignore duplicated symbols.
    ignore_duplicates: bool,
    /// The vocabulary type.
    _vocabulary: core::marker::PhantomData<Vocabulary>,
}

impl<Symbols, V> Default for GenericVocabularyBuilder<Symbols, V> {
    fn default() -> Self {
        Self {
            symbols: None,
            expected_number_of_symbols: None,
            ignore_duplicates: false,
            _vocabulary: core::marker::PhantomData,
        }
    }
}

impl<Symbols, V: GrowableVocabulary> VocabularyBuilder for GenericVocabularyBuilder<Symbols, V>
where
    Symbols: IntoIterator<
        Item = (<V as Vocabulary>::SourceSymbol, <V as Vocabulary>::DestinationSymbol),
    >,
{
    type Symbols = Symbols;
    type Vocabulary = V;

    fn expected_number_of_symbols(mut self, number_of_symbols: usize) -> Self {
        self.expected_number_of_symbols = Some(number_of_symbols);
        self
    }

    fn get_expected_number_of_symbols(&self) -> Option<usize> {
        self.expected_number_of_symbols
    }

    fn ignore_duplicates(mut self) -> Self {
        self.ignore_duplicates = true;
        self
    }

    fn should_ignore_duplicates(&self) -> bool {
        self.ignore_duplicates
    }

    fn symbols(mut self, symbols: Self::Symbols) -> Self {
        self.symbols = Some(symbols);
        self
    }
}

impl<Symbols, V> Builder for GenericVocabularyBuilder<Symbols, V>
where
    Self: VocabularyBuilder<Symbols = Symbols>,
    Symbols: IntoIterator<
        Item = (
            <<Self as VocabularyBuilder>::Vocabulary as Vocabulary>::SourceSymbol,
            <<Self as VocabularyBuilder>::Vocabulary as Vocabulary>::DestinationSymbol,
        ),
    >,
{
    type Object = <Self as VocabularyBuilder>::Vocabulary;
    type Error = crate::errors::builder::vocabulary::VocabularyBuilderError<
        <Self as VocabularyBuilder>::Vocabulary,
    >;
    type Attribute = VocabularyBuilderOptions;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let expected_number_of_symbols = self.get_expected_number_of_symbols();
        let mut vocabulary = if let Some(number_of_symbols) = expected_number_of_symbols {
            <Self as VocabularyBuilder>::Vocabulary::with_capacity(number_of_symbols)
        } else {
            <Self as VocabularyBuilder>::Vocabulary::new()
        };
        let should_ignore_duplicates = self.should_ignore_duplicates();
        self.symbols
            .ok_or({
                common_traits::prelude::BuilderError::IncompleteBuild(
                    VocabularyBuilderOptions::Symbols,
                )
            })?
            .into_iter()
            .try_for_each(|(source, destination)| {
                if let Err(err) = vocabulary.add(source, destination) {
                    match err {
						crate::errors::builder::vocabulary::VocabularyBuilderError::RepeatedSourceSymbol(
							_,
						) | crate::errors::builder::vocabulary::VocabularyBuilderError::RepeatedDestinationSymbol(
							_,
						) => {
							if should_ignore_duplicates {
								Ok(())
							} else {
								Err(err)
							}
						}
						other => Err(other)
					}
                } else {
                    Ok(())
                }
            })?;

        if let Some(expected_number_of_symbols) = expected_number_of_symbols {
            if vocabulary.len() != expected_number_of_symbols {
                return Err(
                    crate::errors::builder::vocabulary::VocabularyBuilderError::NumberOfSymbols {
                        expected: expected_number_of_symbols,
                        actual: vocabulary.len(),
                    },
                );
            }
        }

        Ok(vocabulary)
    }
}
