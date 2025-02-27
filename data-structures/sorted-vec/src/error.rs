//! Enumeration of errors that may occour when handling sorted vectors.

use core::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enumeration of errors that may occour when handling sorted vectors.
pub enum Error<V> {
    /// Error indicating that the vector is not sorted, providing
    /// proof of the offending entry.
    UnsortedEntry(V),
}

impl<V: Debug> core::fmt::Display for Error<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::UnsortedEntry(v) => write!(f, "Found unsorted entru: {v:?}"),
        }
    }
}
