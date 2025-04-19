//! Submodule providing the Intersection trait and the IntersectionIter trait,
//! which determines the intersection of two sorted iterators.

#[derive(Debug, Clone)]
/// The `Intersection` struct represents the intersection of two iterators.
/// It is a generic struct that takes two types, `Left` and `Right`, which
/// implement the `Iterator` trait, with the same item type `Item`. The entries
/// are expected to be sorted and distinct.
pub struct Intersection<Left, Right>
where
    Left: Iterator,
{
    /// The left iterator.
    left: Left,
    /// The right iterator.
    right: Right,
    /// The next item in the left iterator.
    left_next: Option<Left::Item>,
    /// The next item in the right iterator.
    right_next: Option<Left::Item>,
    /// The next back item in the left iterator.
    left_next_back: Option<Left::Item>,
    /// The next back item in the right iterator.
    right_next_back: Option<Left::Item>,
}

impl<Left, Right> Intersection<Left, Right>
where
    Left: Iterator + DoubleEndedIterator,
    Right: Iterator<Item = Left::Item> + DoubleEndedIterator,
{
    #[inline]
    /// Creates a new `Intersection` instance from two sorted iterators of
    /// distinct items.
    ///
    /// # Arguments
    ///
    /// * `left` - The left sorted iterator.
    /// * `right` - The right sorted iterator.
    pub fn new(mut left: Left, mut right: Right) -> Self {
        Self {
            left_next: left.next(),
            right_next: right.next(),
            left_next_back: left.next_back(),
            right_next_back: right.next_back(),
            left,
            right,
        }
    }
}

impl<Left, Right> Iterator for Intersection<Left, Right>
where
    Left: Iterator,
    Left::Item: PartialEq + PartialOrd + Copy,
    Right: Iterator<Item = Left::Item>,
{
    type Item = Left::Item;

    #[inline]
    /// Returns the next item in the intersection of the two sorted iterators of
    /// distinct items.
    fn next(&mut self) -> Option<Self::Item> {
        match (self.left_next, self.right_next) {
            (Some(left), Some(right)) => {
                if left == right {
                    let current_left = self.left_next.take();
                    self.left_next = self.left.next().or_else(|| self.left_next_back.take());
                    self.right_next = self.right.next().or_else(|| self.right_next_back.take());
                    debug_assert!(self.left_next.as_ref().map_or(true, |x| x > &left));
                    debug_assert!(self.right_next.as_ref().map_or(true, |x| x > &right));
                    current_left
                } else if left < right {
                    self.left_next = self.left.next().or_else(|| self.left_next_back.take());
                    debug_assert!(self.left_next.as_ref().map_or(true, |x| x > &left));
                    self.next()
                } else {
                    self.right_next = self.right.next().or_else(|| self.right_next_back.take());
                    debug_assert!(self.right_next.as_ref().map_or(true, |x| x > &right));
                    self.next()
                }
            }
            (_, None) | (None, _) => None,
        }
    }
}

impl<Left, Right> DoubleEndedIterator for Intersection<Left, Right>
where
    Left: Iterator + DoubleEndedIterator,
    Left::Item: PartialEq + PartialOrd + Copy,
    Right: Iterator<Item = Left::Item> + DoubleEndedIterator,
{
    #[inline]
    /// Returns the next item in the intersection of the two sorted iterators of
    /// distinct items from the back.
    fn next_back(&mut self) -> Option<Self::Item> {
        match (self.left_next_back, self.right_next_back) {
            (Some(left), Some(right)) => {
                if left == right {
                    let current_left = self.left_next_back.take();
                    self.left_next_back = self.left.next_back().or_else(|| self.left_next.take());
                    self.right_next_back =
                        self.right.next_back().or_else(|| self.right_next.take());
                    debug_assert!(self.left_next_back.as_ref().map_or(true, |x| x < &left));
                    debug_assert!(self.right_next_back.as_ref().map_or(true, |x| x < &right));
                    current_left
                } else if left > right {
                    self.left_next_back = self.left.next_back().or_else(|| self.left_next.take());
                    debug_assert!(self.left_next_back.as_ref().map_or(true, |x| x < &left));
                    self.next_back()
                } else {
                    self.right_next_back =
                        self.right.next_back().or_else(|| self.right_next.take());
                    debug_assert!(self.right_next_back.as_ref().map_or(true, |x| x < &right));
                    self.next_back()
                }
            }
            (_, None) | (None, _) => None,
        }
    }
}

/// The `IntersectionIter` trait provides a method to get the intersection of
/// two sorted iterators of distinct items.
pub trait IntersectionIter: Iterator + DoubleEndedIterator {
    #[inline]
    /// Returns an iterator that yields the intersection of two sorted
    /// iterators of distinct items.
    ///
    /// # Arguments
    ///
    /// * `self` - The left iterator.
    /// * `other` - The right iterator.
    fn sorted_intersection<Rhs>(self, other: Rhs) -> Intersection<Self, Rhs>
    where
        Self: Sized,
        Self::Item: PartialEq,
        Rhs: Iterator<Item = Self::Item> + DoubleEndedIterator,
    {
        Intersection::new(self, other)
    }
}

impl<I> IntersectionIter for I where I: Iterator + DoubleEndedIterator {}
