//! Submodule providing the errors which may occur when using the `Ranged`
//! trait.

use std::fmt::Display;

/// Error enumeration associated with the `Ranged` trait.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error<N> {
    /// The provided element cannot be added to the range.
    OutOfRange(N),
    /// The provided range is not dense.
    NotDense,
    /// The provided element already exists in the range.
    DuplicateElement(N),
    /// The provided element is not sorted correctly.
    NotSorted(N),
}

impl<N: Display> Display for Error<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OutOfRange(n) => write!(f, "Element `{n}` is out of range"),
            Error::NotDense => write!(f, "Range is not dense"),
            Error::DuplicateElement(n) => write!(f, "Element `{n}` already exists in the range"),
            Error::NotSorted(n) => write!(f, "Element `{n}` is not sorted correctly"),
        }
    }
}
