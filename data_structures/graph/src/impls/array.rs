//! Module implementing traits for the Vec type.

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;
use core::{iter::Cloned, ops::Range};

use algebra::prelude::Symbol;
use numeric_common_traits::prelude::PositiveInteger;

impl<V: Symbol, const N: usize> crate::traits::Vocabulary for [V; N] {
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
        N
    }

    fn sources(&self) -> Self::Sources<'_> {
        0..self.len()
    }

    fn destinations(&self) -> Self::Destinations<'_> {
        self.iter().cloned()
    }
}

impl<V: Symbol, const N: usize> crate::traits::VocabularyRef for [V; N] {
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

impl<V: Symbol, const N: usize> crate::traits::BidirectionalVocabulary for [V; N] {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        self.iter().position(|v| v == destination)
    }
}

impl<NodeId: PositiveInteger> crate::traits::Edge for [NodeId; 2] {
    type SourceNodeId = NodeId;
    type DestinationNodeId = NodeId;

    fn source(&self) -> Self::SourceNodeId {
        self[0]
    }

    fn destination(&self) -> Self::DestinationNodeId {
        self[1]
    }
}
