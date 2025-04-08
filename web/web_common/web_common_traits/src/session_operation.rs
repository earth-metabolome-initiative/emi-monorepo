//! Submodule providing an operation trait that requires the user is logged in.

use crate::prelude::Operation;

/// Trait for an operation which requires the user to be logged in.
pub trait SessionOperation {
    /// The session type associated with the operation.
    type User;
    /// The authenticated operation type associated with the operation.
    type Operation: Operation;

    /// Uses the session to authenticate the operation.
    fn authenticate(self, session_user: &Self::User) -> Self::Operation;
}
