//! Submodule providing a `Range`-like struct with softer and stable
//! constrainsts.

use numeric_common_traits::prelude::{IntoUsize, PositiveInteger};

/// Error enumeration associated with the `Ranged` trait.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RangedError {
    /// The provided element cannot be added to the range.
    OutOfRange,
    /// The provided element already exists in the range.
    DuplicateElement,
}

/// A trait for types that represent a range.
pub trait Ranged:
    core::fmt::Debug
    + Clone
    + Default
    + ExactSizeIterator<Item = <Self as Ranged>::Step>
    + DoubleEndedIterator<Item = <Self as Ranged>::Step>
{
    /// The type of the elements in the range.
    type Step: PositiveInteger + IntoUsize;

    /// Adds a new element to the range.
    ///
    /// # Arguments
    ///
    /// * `element`: The element to add to the range.
    ///
    /// # Returns
    ///
    /// The range with the new element.
    ///
    /// # Errors
    ///
    /// * If the element cannot be added to the range.
    /// * If the element already exists in the range.
    fn add(&mut self, element: Self::Step) -> Result<(), RangedError>;

    /// Returns the number of elements.
    fn number_of_elements(&self) -> Self::Step;

    /// Returns the start of the range.
    fn start(&self) -> Self::Step;

    /// Returns the end of the range.
    fn end(&self) -> Self::Step;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A `Range`-like struct with softer and stable constrainsts.
pub struct SimpleRanged<N> {
    /// The start of the range.
    pub start: N,
    /// The end of the range.
    pub end: N,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A `Range`-like struct with softer and stable constrainsts.
pub enum DoubleRanged<N> {
    /// A single range.
    Single(SimpleRanged<N>),
    /// A double range.
    Double(SimpleRanged<N>, SimpleRanged<N>),
}

impl<N> Ranged for SimpleRanged<N>
where
    N: PositiveInteger + IntoUsize,
{
    type Step = N;

    fn add(&mut self, element: Self::Step) -> Result<(), RangedError> {
        if element >= self.start && element < self.end {
            return Err(RangedError::DuplicateElement);
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
            Err(RangedError::OutOfRange)
        }
    }

    #[inline]
    fn number_of_elements(&self) -> Self::Step {
        self.end - self.start
    }

    #[inline]
    fn start(&self) -> Self::Step {
        self.start
    }

    #[inline]
    fn end(&self) -> Self::Step {
        self.end
    }
}

impl<N> Ranged for DoubleRanged<N>
where
    N: PositiveInteger + IntoUsize,
{
    type Step = N;

    fn add(&mut self, element: Self::Step) -> Result<(), RangedError> {
        match self {
            Self::Single(range) => {
                if let Err(err) = range.add(element) {
                    match err {
                        RangedError::DuplicateElement => Err(RangedError::DuplicateElement),
                        RangedError::OutOfRange => {
                            // If the element is out of the range, we can split the range.
                            if element < range.start {
                                *self = Self::Double(
                                    SimpleRanged::new(element, element + Self::Step::ONE),
                                    range.clone(),
                                );
                            } else if element >= range.end {
                                *self = Self::Double(
                                    range.clone(),
                                    SimpleRanged::new(element, element + Self::Step::ONE),
                                );
                            }
                            Ok(())
                        }
                    }
                } else {
                    Ok(())
                }
            }
            Self::Double(left, right) => {
                left.add(element).or_else(|_| right.add(element))?;
                if left.end == right.start {
                    *self = Self::Single(SimpleRanged::new(left.start, right.end));
                }
                Ok(())
            }
        }
    }

    #[inline]
    fn number_of_elements(&self) -> Self::Step {
        match self {
            Self::Single(range) => range.number_of_elements(),
            Self::Double(left, right) => left.number_of_elements() + right.number_of_elements(),
        }
    }

    #[inline]
    fn start(&self) -> Self::Step {
        match self {
            Self::Single(range) => range.start,
            Self::Double(left, _) => left.start,
        }
    }

    #[inline]
    fn end(&self) -> Self::Step {
        match self {
            Self::Single(range) => range.end,
            Self::Double(_, right) => right.end,
        }
    }
}

impl<N: PositiveInteger> Default for SimpleRanged<N> {
    #[inline]
    fn default() -> Self {
        Self { start: N::ZERO, end: N::ZERO }
    }
}

impl<N: PositiveInteger> Default for DoubleRanged<N> {
    #[inline]
    fn default() -> Self {
        Self::Single(SimpleRanged::default())
    }
}

impl<N: PositiveInteger> SimpleRanged<N> {
    #[inline]
    /// Creates a new `SimpleRanged` struct.
    pub fn new(start: N, end: N) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }
}

impl<N: PositiveInteger> Iterator for SimpleRanged<N> {
    type Item = N;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let current = self.start;
            self.start += N::ONE;
            Some(current)
        } else {
            None
        }
    }
}

impl<N: PositiveInteger> Iterator for DoubleRanged<N> {
    type Item = N;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(range) => range.next(),
            Self::Double(range1, range2) => range1.next().or_else(|| range2.next()),
        }
    }
}

impl<N: PositiveInteger> DoubleEndedIterator for SimpleRanged<N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= N::ONE;
            Some(self.end)
        } else {
            None
        }
    }
}

impl<N: PositiveInteger> DoubleEndedIterator for DoubleRanged<N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(range) => range.next_back(),
            Self::Double(range1, range2) => range2.next_back().or_else(|| range1.next_back()),
        }
    }
}

impl<N: PositiveInteger + IntoUsize> ExactSizeIterator for SimpleRanged<N> {
    #[inline]
    fn len(&self) -> usize {
        (self.end - self.start).into_usize()
    }
}

impl<N: PositiveInteger + IntoUsize> ExactSizeIterator for DoubleRanged<N> {
    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::Single(range) => range.len(),
            Self::Double(range1, range2) => range1.len() + range2.len(),
        }
    }
}

impl<N: PositiveInteger> From<(N, N)> for SimpleRanged<N> {
    #[inline]
    fn from((start, end): (N, N)) -> Self {
        Self::new(start, end)
    }
}

impl<N: PositiveInteger> From<(N, N)> for DoubleRanged<N> {
    #[inline]
    fn from((start, end): (N, N)) -> Self {
        Self::Single(SimpleRanged::new(start, end))
    }
}

impl<N: PositiveInteger> From<((N, N), (N, N))> for DoubleRanged<N> {
    #[inline]
    fn from(((start1, end1), (start2, end2)): ((N, N), (N, N))) -> Self {
        Self::Double(SimpleRanged::new(start1, end1), SimpleRanged::new(start2, end2))
    }
}
