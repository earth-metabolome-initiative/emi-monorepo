//! Enumeration of errors that may occour when handling sorted vectors.

#[derive(Clone, PartialEq, Eq)]
/// Enumeration of errors that may occour when handling sorted vectors.
pub enum Error<V> {
    /// Error indicating that the vector is not sorted, providing
    /// proof of the offending entry.
    UnsortedEntry(V),
}

impl<V: core::fmt::Display> core::fmt::Display for Error<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::UnsortedEntry(v) => write!(f, "Found unsorted entry: {v}"),
        }
    }
}

impl<V: core::fmt::Display> core::fmt::Debug for Error<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Error<V> as core::fmt::Display>::fmt(self, f)
    }
}

impl<V: core::fmt::Display> core::error::Error for Error<V> {}
