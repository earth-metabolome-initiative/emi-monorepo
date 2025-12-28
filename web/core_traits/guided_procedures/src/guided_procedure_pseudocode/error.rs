//! Submodule defining the errors which might occur when using the
//! `GuidedProcedurePseudocode`.

use std::fmt::Display;

#[derive(Debug)]
/// Enum representing the possible errors which might occur when using the
/// `GuidedProcedurePseudocode`.
pub enum GuidedProcedurePseudocodeError {
    /// The provided graph is not a simple path.
    NotASimplePath,
}

impl Display for GuidedProcedurePseudocodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuidedProcedurePseudocodeError::NotASimplePath => {
                write!(f, "The provided graph is not a simple path.")
            }
        }
    }
}

impl std::error::Error for GuidedProcedurePseudocodeError {}
