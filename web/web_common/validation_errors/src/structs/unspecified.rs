//! Submodule for unspecified or unknown fields marker struct.

use std::fmt::Display;

/// Marker struct for unspecified or unknown fields.
#[derive(Debug, PartialEq, Eq)]
pub struct Unspecified;

impl Display for Unspecified {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unspecified")
    }
}
