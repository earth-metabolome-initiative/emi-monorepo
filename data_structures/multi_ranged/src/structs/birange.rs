//! Submodule providing the `BiRange` struct.

use std::ops::{Mul, MulAssign};

use super::SimpleRange;
use crate::{MultiRanged, Step, errors::Error};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A struct representing a range which may be split into two parts.
pub enum BiRange<N> {
    /// A single range.
    Single(SimpleRange<N>),
    /// A double range.
    Double(SimpleRange<N>, SimpleRange<N>),
}

impl<N: Step> MultiRanged for BiRange<N> {
    type Step = N;

    fn insert(&mut self, element: Self::Step) -> Result<(), Error<N>> {
        match self {
            Self::Single(range) => {
                if let Err(err) = range.insert(element) {
                    match err {
                        Error::OutOfRange(out_of_range_element) => {
                            // If the element is out of the range, we can split the range.
                            if out_of_range_element
                                < range.absolute_start().expect("Range must have a start")
                            {
                                *self = Self::Double(
                                    SimpleRange::try_from((
                                        out_of_range_element,
                                        out_of_range_element + Self::Step::ONE,
                                    ))?,
                                    range.clone(),
                                );
                            } else if out_of_range_element
                                >= range.absolute_end().expect("Range must have an end")
                            {
                                *self = Self::Double(
                                    range.clone(),
                                    SimpleRange::try_from((
                                        out_of_range_element,
                                        out_of_range_element + Self::Step::ONE,
                                    ))?,
                                );
                            }
                            Ok(())
                        }
                        err => {
                            // If the element is a duplicate or not sorted, we return the error.
                            Err(err)
                        }
                    }
                } else {
                    Ok(())
                }
            }
            Self::Double(left, right) => {
                left.insert(element).or_else(|_| right.insert(element))?;
                if left.absolute_end() == right.absolute_start() {
                    *self = Self::try_from((
                        left.absolute_start().expect("Range must have a start"),
                        right.absolute_end().expect("Range must have an end"),
                    ))?;
                }
                Ok(())
            }
        }
    }

    fn merge<Rhs: MultiRanged<Step = Self::Step>>(
        &mut self,
        other: &Rhs,
    ) -> Result<(), Error<Self::Step>> {
        match self {
            Self::Single(range) => {
                range.merge(other)?;
                Ok(())
            }
            Self::Double(left, right) => {
                let outcome = left.merge(other);
                if outcome.is_ok() || right.merge(other).is_ok() {
                    if left.absolute_end() == right.absolute_start() {
                        *self = Self::try_from((
                            left.absolute_start().expect("Range must have a start"),
                            right.absolute_end().expect("Range must have an end"),
                        ))?;
                    }
                    Ok(())
                } else {
                    outcome
                }
            }
        }
    }

    #[inline]
    fn absolute_start(&self) -> Option<Self::Step> {
        match self {
            Self::Single(range) => range.absolute_start(),
            Self::Double(left, _) => left.absolute_start(),
        }
    }

    #[inline]
    fn absolute_end(&self) -> Option<Self::Step> {
        match self {
            Self::Single(range) => range.absolute_end(),
            Self::Double(_, right) => right.absolute_end(),
        }
    }

    fn contains(&self, element: Self::Step) -> bool {
        match self {
            Self::Single(range) => range.contains(element),
            Self::Double(left, right) => left.contains(element) || right.contains(element),
        }
    }

    fn is_dense(&self) -> bool {
        match self {
            Self::Single(_) => true,
            Self::Double(_, _) => false,
        }
    }
}

impl<N: Step> Default for BiRange<N> {
    #[inline]
    fn default() -> Self {
        Self::Single(SimpleRange::default())
    }
}

impl<N: Step> Iterator for BiRange<N> {
    type Item = N;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(range) => range.next(),
            Self::Double(range1, range2) => range1.next().or_else(|| range2.next()),
        }
    }
}

impl<N: Step> DoubleEndedIterator for BiRange<N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(range) => range.next_back(),
            Self::Double(range1, range2) => range2.next_back().or_else(|| range1.next_back()),
        }
    }
}

impl<N: Step> ExactSizeIterator for BiRange<N> {
    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::Single(range) => range.len(),
            Self::Double(range1, range2) => range1.len() + range2.len(),
        }
    }
}

impl<N: Step> From<SimpleRange<N>> for BiRange<N> {
    #[inline]
    fn from(range: SimpleRange<N>) -> Self {
        Self::Single(range)
    }
}

impl<N: Step> TryFrom<(N, N)> for BiRange<N> {
    type Error = Error<N>;

    #[inline]
    fn try_from((start, end): (N, N)) -> Result<Self, Self::Error> {
        Ok(SimpleRange::try_from((start, end))?.into())
    }
}

impl<N: Step> TryFrom<&[N]> for BiRange<N> {
    type Error = Error<N>;

    fn try_from(slice: &[N]) -> Result<Self, Self::Error> {
        let mut birange = Self::default();
        for element in slice {
            birange.insert(*element)?;
        }
        Ok(birange)
    }
}

impl<N: Step> TryFrom<Vec<N>> for BiRange<N> {
    type Error = Error<N>;

    fn try_from(vec: Vec<N>) -> Result<Self, Self::Error> {
        Self::try_from(vec.as_slice())
    }
}

impl<N: Step> From<N> for BiRange<N> {
    #[inline]
    fn from(step: N) -> Self {
        Self::Single(SimpleRange::from(step))
    }
}

impl<N: Step> Mul<N> for BiRange<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        match self {
            Self::Single(range) => Self::Single(range * rhs),
            Self::Double(left, right) => Self::Double(left * rhs, right * rhs),
        }
    }
}

impl<N: Step> MulAssign<N> for BiRange<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: N) {
        match self {
            Self::Single(range) => range.mul_assign(rhs),
            Self::Double(left, right) => {
                left.mul_assign(rhs);
                right.mul_assign(rhs);
            }
        }
    }
}

impl<N: Step> From<BiRange<N>> for Vec<N> {
    #[inline]
    fn from(multi_range: BiRange<N>) -> Self {
        let mut elements = Vec::new();
        for element in multi_range {
            elements.push(element);
        }
        elements
    }
}
