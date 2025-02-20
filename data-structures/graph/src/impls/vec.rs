//! Module implementing traits for the Vec type.

use algebra::prelude::Symbol;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

use core::iter::Cloned;
use core::ops::Range;

impl<V: Symbol> crate::traits::Vocabulary for Vec<V> {
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

impl<V: Symbol> crate::traits::VocabularyRef for Vec<V> {
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

impl<V: Symbol> crate::traits::BidirectionalVocabulary for Vec<V> {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        self.iter().position(|v| v == destination)
    }
}
