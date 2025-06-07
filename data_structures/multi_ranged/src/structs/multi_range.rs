//! Submodule providing the `MultiRange` struct.

use std::ops::{Mul, MulAssign};

use super::SimpleRange;
use crate::{MultiRanged, Step, errors::Error};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A struct representing a range which may be split into two parts.
pub struct MultiRange<N> {
    /// A vector of `SimpleRange` instances.
    ranges: Vec<SimpleRange<N>>,
}

impl<N: Step> MultiRanged for MultiRange<N> {
    type Step = N;

    #[inline]
    fn insert(&mut self, element: Self::Step) -> Result<(), Error<N>> {
        if self.ranges.is_empty() {
            // If there are no ranges, create a new one with the element.
            self.ranges.push(SimpleRange::from(element));
            return Ok(());
        }

        match self.ranges.binary_search_by(|probe| {
            let absolute_start = probe.absolute_start().expect("Range must have a start").prev();
            let absolute_end = probe.absolute_end().expect("Range must have an end");
            if element < absolute_start {
                std::cmp::Ordering::Greater
            } else if element > absolute_end {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }) {
            Ok(mut index) => {
                let _ = self.ranges[index].insert(element);

                // We check whether inserting this element has now merged two ranges.
                if index > 0
                    && self.ranges[index - 1].absolute_end() == self.ranges[index].absolute_start()
                {
                    // Merge with the previous range.
                    let merged_range = self.ranges.remove(index);
                    self.ranges[index - 1].merge(&merged_range)?;
                    index -= 1; // Adjust index after removal.
                }
                if index < self.ranges.len() - 1
                    && self.ranges[index + 1].absolute_start() == self.ranges[index].absolute_end()
                {
                    // Merge with the next range.
                    let merged_range = self.ranges.remove(index + 1);
                    self.ranges[index].merge(&merged_range)?;
                }
            }
            Err(index) => {
                self.ranges.insert(index, SimpleRange::from(element));
            }
        }

        Ok(())
    }

    fn merge<Rhs: MultiRanged<Step = Self::Step>>(
        &mut self,
        _other: &Rhs,
    ) -> Result<(), Error<Self::Step>> {
        todo!("Merging MultiRange is not implemented yet");
    }

    #[inline]
    fn absolute_start(&self) -> Option<Self::Step> {
        self.ranges.first().and_then(MultiRanged::absolute_start)
    }

    #[inline]
    fn absolute_end(&self) -> Option<Self::Step> {
        self.ranges.last().and_then(MultiRanged::absolute_end)
    }

    #[inline]
    fn contains(&self, element: Self::Step) -> bool {
        self.ranges.iter().any(|range| range.contains(element))
    }

    #[inline]
    fn is_dense(&self) -> bool {
        self.ranges.len() == 1
    }
}

impl<N: Step> Default for MultiRange<N> {
    #[inline]
    fn default() -> Self {
        Self { ranges: Vec::new() }
    }
}

impl<N: Step> Iterator for MultiRange<N> {
    type Item = N;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let first_range = self.ranges.first_mut()?;
        first_range.next().or_else(|| {
            // If the first range is exhausted, remove it and try the next one.
            self.ranges.remove(0);
            self.next()
        })
    }
}

impl<N: Step> DoubleEndedIterator for MultiRange<N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let last_range = self.ranges.last_mut()?;
        last_range.next_back().or_else(|| {
            // If the last range is exhausted, remove it and try the next one.
            self.ranges.pop();
            self.next_back()
        })
    }
}

impl<N: Step> ExactSizeIterator for MultiRange<N> {
    #[inline]
    fn len(&self) -> usize {
        self.ranges.iter().map(ExactSizeIterator::len).sum()
    }
}

impl<N: Step> From<SimpleRange<N>> for MultiRange<N> {
    #[inline]
    fn from(range: SimpleRange<N>) -> Self {
        if range.len() == 0 {
            return Self::default();
        }

        Self { ranges: vec![range] }
    }
}

impl<N: Step> TryFrom<(N, N)> for MultiRange<N> {
    type Error = Error<N>;

    #[inline]
    fn try_from((start, end): (N, N)) -> Result<Self, Self::Error> {
        Ok(SimpleRange::try_from((start, end))?.into())
    }
}

impl<N: Step> TryFrom<&[N]> for MultiRange<N> {
    type Error = Error<N>;

    fn try_from(slice: &[N]) -> Result<Self, Self::Error> {
        let mut multi_range = Self::default();
        for element in slice {
            multi_range.insert(*element)?;
        }
        Ok(multi_range)
    }
}

impl<N: Step> TryFrom<Vec<N>> for MultiRange<N> {
    type Error = Error<N>;

    fn try_from(vec: Vec<N>) -> Result<Self, Self::Error> {
        Self::try_from(vec.as_slice())
    }
}

impl<N: Step, const L: usize> TryFrom<[N; L]> for MultiRange<N> {
    type Error = Error<N>;

    fn try_from(array: [N; L]) -> Result<Self, Self::Error> {
        Self::try_from(array.as_slice())
    }
}

impl<N: Step> From<N> for MultiRange<N> {
    #[inline]
    fn from(element: N) -> Self {
        Self::from(SimpleRange::from(element))
    }
}

impl<N: Step> Mul<N> for MultiRange<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        Self { ranges: self.ranges.into_iter().map(|range| range * rhs).collect() }
    }
}

impl<N: Step> MulAssign<N> for MultiRange<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: N) {
        for range in &mut self.ranges {
            *range *= rhs;
        }
    }
}

impl<N: Step> MultiRange<N> {
    #[inline]
    /// Computes the multiplication of all elements in the `MultiRange` by a
    /// given factor, checking for overflow.
    pub fn checked_mul(&self, factor: N) -> Option<Self> {
        let mut new_ranges = Vec::with_capacity(self.ranges.len());
        for range in &self.ranges {
            new_ranges.push(range.checked_mul(factor)?);
        }
        Some(Self { ranges: new_ranges })
    }
}

impl<N: Step> From<MultiRange<N>> for Vec<N> {
    #[inline]
    fn from(multi_range: MultiRange<N>) -> Self {
        let mut elements = Vec::new();
        for element in multi_range {
            elements.push(element);
        }
        elements
    }
}
