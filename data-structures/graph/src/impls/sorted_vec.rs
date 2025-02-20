//! Module implementing traits for the Vec type.

use algebra::prelude::Symbol;
use core::iter::Cloned;
use core::ops::Range;
use sorted_vec::prelude::SortedVec;

impl<V: Symbol> crate::traits::Vocabulary for SortedVec<V> {
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

impl<V: Symbol> crate::traits::VocabularyRef for SortedVec<V> {
    type DestinationRefs<'a> = core::slice::Iter<'a, Self::DestinationSymbol>
    where
        Self: 'a;

    fn convert_ref(&self, source: &Self::SourceSymbol) -> Option<&Self::DestinationSymbol> {
        self.get(*source)
    }

    fn destination_refs(&self) -> Self::DestinationRefs<'_> {
        self.iter()
    }
}

impl<V: Symbol + Ord> crate::traits::BidirectionalVocabulary for SortedVec<V> {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        self.binary_search(destination).ok()
    }
}
