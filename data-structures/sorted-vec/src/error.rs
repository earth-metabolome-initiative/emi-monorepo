//! Enumeration of errors that may occour when handling sorted vectors.

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enumeration of errors that may occour when handling sorted vectors.
pub enum Error {
	/// Error indicating that the vector is not sorted.
	NotSorted,
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Error::NotSorted => write!(f, "The vector is not sorted"),
		}
	}
}