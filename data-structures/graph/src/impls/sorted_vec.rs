//! Module implementing traits for the Vec type.

use core::{iter::Cloned, ops::Range};

use algebra::prelude::Symbol;
use sorted_vec::prelude::SortedVec;

use crate::prelude::*;

impl<V: Symbol> Vocabulary for SortedVec<V> {
    type SourceSymbol = usize;
    type DestinationSymbol = V;
    type Sources<'a>
        = Range<usize>
    where
        Self: 'a;
    type Destinations<'a>
        = Cloned<core::slice::Iter<'a, Self::DestinationSymbol>>
    where
        Self: 'a;

    fn convert(&self, source: &Self::SourceSymbol) -> Option<Self::DestinationSymbol> {
        self.get(*source).cloned()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn sources(&self) -> Self::Sources<'_> {
        0..self.len()
    }

    fn destinations(&self) -> Self::Destinations<'_> {
        self.iter().cloned()
    }
}

impl<V: Symbol> VocabularyRef for SortedVec<V> {
    type DestinationRefs<'a>
        = core::slice::Iter<'a, Self::DestinationSymbol>
    where
        Self: 'a;

    fn convert_ref(&self, source: &Self::SourceSymbol) -> Option<&Self::DestinationSymbol> {
        self.get(*source)
    }

    fn destination_refs(&self) -> Self::DestinationRefs<'_> {
        self.iter()
    }
}

impl<V: Symbol + Ord> BidirectionalVocabulary for SortedVec<V> {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        self.as_ref().binary_search(destination).ok()
    }
}

impl<V: Symbol + Ord> GrowableVocabulary for SortedVec<V> {
    fn new() -> Self {
        SortedVec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        SortedVec::with_capacity(capacity)
    }

    fn add(
        &mut self,
        source: Self::SourceSymbol,
        destination: Self::DestinationSymbol,
    ) -> Result<(), crate::errors::builder::vocabulary::VocabularyBuilderError<Self>> {
        if source != self.len() {
            return Err(
                crate::errors::builder::vocabulary::VocabularyBuilderError::SparseSourceNode(
                    source,
                ),
            );
        }

        self.push(destination).map_err(|err| {
            match err {
                sorted_vec::error::Error::UnsortedEntry(destination) => {
                    crate::errors::builder::vocabulary::VocabularyBuilderError::UnorderedDestinationNode(destination)
                }
            }
        })?;

        Ok(())
    }
}
