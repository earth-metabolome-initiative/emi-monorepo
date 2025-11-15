//! Submodule defining a struct implementing `ConstraintFailureInformation` for
//! error reporting.

mod builder;

use std::fmt::Display;

use crate::traits::ConstraintFailureInformation;

#[derive(Debug)]
/// Struct implementing `ConstraintFailureInformation` for detailed error
/// reporting.
pub struct ConstraintErrorInfo {
    /// Type of constraint which failed.
    constraint: &'static str,
    /// DB object which failed the constraint.
    object: String,
    /// Error message describing the failure.
    message: String,
    /// What should be done to fix the failure.
    resolution: Option<String>,
}

impl ConstraintErrorInfo {
    /// Creates a new constraint error info builder.
    #[must_use]
    pub fn new() -> builder::ConstraintErrorInfoBuilder {
        builder::ConstraintErrorInfoBuilder::default()
    }
}

impl From<ConstraintErrorInfo> for Box<dyn ConstraintFailureInformation> {
    fn from(info: ConstraintErrorInfo) -> Self {
        Box::new(info)
    }
}

impl Display for ConstraintErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Constraint: {}\nObject: {}\nMessage: {}",
            self.constraint, self.object, self.message
        )?;
        if let Some(resolution) = &self.resolution {
            write!(f, "\nResolution: {resolution}")?;
        }
        Ok(())
    }
}

impl ConstraintFailureInformation for ConstraintErrorInfo {
    fn constraint(&self) -> &'static str {
        self.constraint
    }

    fn object(&self) -> &str {
        &self.object
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn resolution(&self) -> Option<&str> {
        self.resolution.as_deref()
    }
}
