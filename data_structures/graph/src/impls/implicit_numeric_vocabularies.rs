//! Submodule providing the implementation of implicit numeric vocabularies.

use crate::traits::{BidirectionalVocabulary, Vocabulary};

impl Vocabulary for u8 {
    type SourceSymbol = u8;
    type DestinationSymbol = u8;
    type Sources<'a> = core::ops::Range<u8>;
    type Destinations<'a> = core::ops::Range<u8>;

    fn convert(&self, source: &Self::SourceSymbol) -> Option<Self::DestinationSymbol> {
        Some(*source)
    }

    fn len(&self) -> usize {
        usize::from(*self)
    }

    fn sources(&self) -> Self::Sources<'_> {
        0..*self
    }

    fn destinations(&self) -> Self::Destinations<'_> {
        0..*self
    }
}

impl BidirectionalVocabulary for u8 {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        Some(*destination)
    }
}

impl Vocabulary for u16 {
    type SourceSymbol = u16;
    type DestinationSymbol = u16;
    type Sources<'a> = core::ops::Range<u16>;
    type Destinations<'a> = core::ops::Range<u16>;

    fn convert(&self, source: &Self::SourceSymbol) -> Option<Self::DestinationSymbol> {
        Some(*source)
    }

    fn len(&self) -> usize {
        usize::from(*self)
    }

    fn sources(&self) -> Self::Sources<'_> {
        0..*self
    }

    fn destinations(&self) -> Self::Destinations<'_> {
        0..*self
    }
}

impl BidirectionalVocabulary for u16 {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        Some(*destination)
    }
}

impl Vocabulary for u32 {
    type SourceSymbol = u32;
    type DestinationSymbol = u32;
    type Sources<'a> = core::ops::Range<u32>;
    type Destinations<'a> = core::ops::Range<u32>;

    fn convert(&self, source: &Self::SourceSymbol) -> Option<Self::DestinationSymbol> {
        Some(*source)
    }

    fn len(&self) -> usize {
        *self as usize
    }

    fn sources(&self) -> Self::Sources<'_> {
        0..*self
    }

    fn destinations(&self) -> Self::Destinations<'_> {
        0..*self
    }
}

impl BidirectionalVocabulary for u32 {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        Some(*destination)
    }
}

impl Vocabulary for usize {
    type SourceSymbol = usize;
    type DestinationSymbol = usize;
    type Sources<'a> = core::ops::Range<usize>;
    type Destinations<'a> = core::ops::Range<usize>;

    fn convert(&self, source: &Self::SourceSymbol) -> Option<Self::DestinationSymbol> {
        Some(*source)
    }

    fn len(&self) -> usize {
        *self
    }

    fn sources(&self) -> Self::Sources<'_> {
        0..*self
    }

    fn destinations(&self) -> Self::Destinations<'_> {
        0..*self
    }
}

impl BidirectionalVocabulary for usize {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        Some(*destination)
    }
}

impl Vocabulary for u64 {
    type SourceSymbol = u64;
    type DestinationSymbol = u64;
    type Sources<'a> = core::ops::Range<u64>;
    type Destinations<'a> = core::ops::Range<u64>;

    fn convert(&self, source: &Self::SourceSymbol) -> Option<Self::DestinationSymbol> {
        Some(*source)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn len(&self) -> usize {
        *self as usize
    }

    fn sources(&self) -> Self::Sources<'_> {
        0..*self
    }

    fn destinations(&self) -> Self::Destinations<'_> {
        0..*self
    }
}

impl BidirectionalVocabulary for u64 {
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol> {
        Some(*destination)
    }
}
