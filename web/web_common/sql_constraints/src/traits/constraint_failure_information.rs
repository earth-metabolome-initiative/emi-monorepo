//! Submodule providing the `ConstraintFailureInformation` trait for error reporting.

use std::fmt::Display;

/// Trait for types that provide information about a constraint failure.
pub trait ConstraintFailureInformation: Display {
    /// Type of constraint which failed.
    fn constraint(&self) -> &'static str;

    /// DB object which failed the constraint.
    fn object(&self) -> &str;

    /// Error message describing the failure.
    fn message(&self) -> &str;
}
