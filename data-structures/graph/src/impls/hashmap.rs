//! Module implementing traits for the [`HashMap`] type.

use core::iter::Cloned;
use std::{collections::HashMap, hash::BuildHasher};

use algebra::prelude::Symbol;

use crate::prelude::*;

impl<K: Symbol, V: Symbol, S: BuildHasher + Clone> Vocabulary for HashMap<K, V, S> {
    type SourceSymbol = K;
    type DestinationSymbol = V;
    type Sources<'a>
        = Cloned<std::collections::hash_map::Keys<'a, K, V>>
    where
        Self: 'a;
    type Destinations<'a>
        = Cloned<std::collections::hash_map::Values<'a, K, V>>
    where
        Self: 'a;

    fn convert(&self, source: &Self::SourceSymbol) -> Option<Self::DestinationSymbol> {
        self.get(source).cloned()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn sources(&self) -> Self::Sources<'_> {
        self.keys().cloned()
    }

    fn destinations(&self) -> Self::Destinations<'_> {
        self.values().cloned()
    }
}

impl<K: Symbol, V: Symbol, S: BuildHasher + Clone> VocabularyRef for HashMap<K, V, S> {
    type DestinationRefs<'a>
        = std::collections::hash_map::Values<'a, K, V>
    where
        Self: 'a;

    fn convert_ref(&self, source: &Self::SourceSymbol) -> Option<&Self::DestinationSymbol> {
        self.get(source)
    }

    fn destination_refs(&self) -> Self::DestinationRefs<'_> {
        self.values()
    }
}

impl<K: Symbol, V: Symbol, S: BuildHasher + Clone> BidirectionalVocabulary for HashMap<K, V, S> {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        self.iter().find(|(_, v)| v == &destination).map(|(k, _)| k.clone())
    }
}

impl<K: Symbol, V: Symbol, S: BuildHasher + Clone> BidirectionalVocabularyRef for HashMap<K, V, S> {
    type SourceRefs<'a>
        = std::collections::hash_map::Keys<'a, K, V>
    where
        Self: 'a;

    fn invert_ref(&self, destination: &Self::DestinationSymbol) -> Option<&Self::SourceSymbol> {
        self.iter().find(|(_, v)| v == &destination).map(|(k, _)| k)
    }

    fn source_refs(&self) -> Self::SourceRefs<'_> {
        self.keys()
    }
}

impl<K: Symbol, V: Symbol, S: BuildHasher + Default + Clone> GrowableVocabulary
    for HashMap<K, V, S>
{
    fn new() -> Self {
        HashMap::with_hasher(Default::default())
    }

    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity_and_hasher(capacity, Default::default())
    }

    fn add(
        &mut self,
        source: K,
        destination: V,
    ) -> Result<(), crate::errors::builder::vocabulary::VocabularyBuilderError<Self>> {
        if self.contains_key(&source) {
            return Err(
                crate::errors::builder::vocabulary::VocabularyBuilderError::RepeatedSourceSymbol(
                    source,
                ),
            );
        }
        if self.invert_ref(&destination).is_some() {
            return Err(crate::errors::builder::vocabulary::VocabularyBuilderError::RepeatedDestinationSymbol(destination));
        }
        self.insert(source, destination);
        Ok(())
    }
}
