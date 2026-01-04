//! Contiguous range implementation.

use std::ops::{Mul, MulAssign};

use crate::{MultiRanged, Step, errors::Error};

/// A contiguous range from start to end (exclusive).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleRange<N> {
    /// The start of the range.
    start: N,
    /// The end of the range.
    end: N,
}

impl<N> MultiRanged for SimpleRange<N>
where
    N: Step,
{
    type Step = N;

    fn insert(&mut self, element: Self::Step) -> Result<(), Error<N>> {
        if element >= self.start && element < self.end {
            return Err(Error::DuplicateElement(element));
        }

        // If the range is currently completely empty,
        // we need to set the start and end relative to the element.
        if self.start == self.end {
            self.start = element;
            self.end = element + N::ONE;
            Ok(())
        } else if element + N::ONE == self.start {
            self.start = element;
            Ok(())
        } else if element == self.end {
            self.end = element + N::ONE;
            Ok(())
        } else {
            Err(Error::OutOfRange(element))
        }
    }

    fn merge<Rhs: MultiRanged<Step = Self::Step>>(
        &mut self,
        other: &Rhs,
    ) -> Result<(), Error<Self::Step>> {
        if other.len() == 0 {
            return Ok(());
        }
        if !other.is_dense() {
            return Err(Error::NotDense);
        }
        if self.len() == 0 {
            self.start = other.absolute_start().unwrap_or(self.start);
            self.end = other.absolute_end().unwrap_or(self.end);
            return Ok(());
        }

        if Some(self.start) <= other.absolute_start() && Some(self.end) >= other.absolute_start()
            || other.absolute_end() >= Some(self.start) && other.absolute_end() >= Some(self.end)
        {
            self.start = other.absolute_start().unwrap().min(self.start);
            self.end = other.absolute_end().unwrap().max(self.end);
            Ok(())
        } else {
            Err(Error::OutOfRange(other.absolute_start().unwrap()))
        }
    }

    #[inline]
    fn absolute_start(&self) -> Option<Self::Step> {
        if self.start < self.end { Some(self.start) } else { None }
    }

    #[inline]
    fn absolute_end(&self) -> Option<Self::Step> {
        if self.start < self.end { Some(self.end) } else { None }
    }

    fn contains(&self, element: Self::Step) -> bool {
        element >= self.start && element < self.end && self.start < self.end
    }

    fn is_dense(&self) -> bool {
        true
    }
}

impl<N: Step> Default for SimpleRange<N> {
    #[inline]
    fn default() -> Self {
        Self { start: N::ZERO, end: N::ZERO }
    }
}

impl<N: Step> Iterator for SimpleRange<N> {
    type Item = N;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let current = self.start;
            self.start = self.start.next();
            Some(current)
        } else {
            None
        }
    }
}

impl<N: Step> DoubleEndedIterator for SimpleRange<N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end = self.end.prev();
            Some(self.end)
        } else {
            None
        }
    }
}

impl<N: Step> ExactSizeIterator for SimpleRange<N> {
    #[inline]
    fn len(&self) -> usize {
        (self.end - self.start).to_usize().expect("Step type should implement ToPrimitive")
    }
}

impl<N: Step> TryFrom<(N, N)> for SimpleRange<N> {
    type Error = Error<N>;

    fn try_from((start, end): (N, N)) -> Result<Self, Self::Error> {
        if start > end { Err(Error::OutOfRange(start)) } else { Ok(Self { start, end }) }
    }
}

impl<N: Step> TryFrom<&[N]> for SimpleRange<N> {
    type Error = Error<N>;

    fn try_from(slice: &[N]) -> Result<Self, Self::Error> {
        slice.windows(2).try_for_each(|window| {
            if window[0] >= window[1] {
                return Err(Error::NotSorted(window[0]));
            }
            Ok(())
        })?;
        let start = slice[0];
        let end = slice[slice.len() - 1] + N::ONE;
        SimpleRange::try_from((start, end))
    }
}

impl<N: Step> TryFrom<Vec<N>> for SimpleRange<N> {
    type Error = Error<N>;

    fn try_from(vec: Vec<N>) -> Result<Self, Self::Error> {
        Self::try_from(vec.as_slice())
    }
}

impl<N: Step> From<N> for SimpleRange<N> {
    #[inline]
    fn from(element: N) -> Self {
        Self { start: element, end: element.next() }
    }
}

impl<N: Step> Mul<N> for SimpleRange<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        Self { start: self.start * rhs, end: ((self.end.prev()) * rhs).next() }
    }
}

impl<N: Step> MulAssign<N> for SimpleRange<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: N) {
        self.start *= rhs;
        self.end = (self.end.prev() * rhs).next();
    }
}

impl<N: Step> SimpleRange<N> {
    #[inline]
    /// Computes the multiplication of all elements in the `SimpleRange` by a
    /// given factor, checking for overflow.
    pub fn checked_mul(&self, factor: N) -> Option<Self> {
        let start = self.start.checked_mul(&factor)?;
        let end = self.end.prev().checked_mul(&factor)?.next();
        Some(Self { start, end })
    }
}

impl<N: Step> From<SimpleRange<N>> for (N, N) {
    #[inline]
    fn from(range: SimpleRange<N>) -> Self {
        (range.start, range.end)
    }
}

impl<N: Step> From<SimpleRange<N>> for Vec<N> {
    #[inline]
    fn from(range: SimpleRange<N>) -> Self {
        let mut vec = Vec::with_capacity(range.len());
        for element in range {
            vec.push(element);
        }
        vec
    }
}
