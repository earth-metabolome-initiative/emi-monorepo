//! Submodule defining the `Constraint` trait, which defines a constraint which can be applied to
//! a `Constrainable` object.

use crate::traits::{Constrainable, ConstraintFailureInformation};

/// Trait for types that define a constraint object.
pub trait Constraint {
    /// Returns information about the failure of this constraint.
    fn error_information(
        &self,
        context: &dyn Constrainable,
    ) -> Box<dyn ConstraintFailureInformation>;
}
