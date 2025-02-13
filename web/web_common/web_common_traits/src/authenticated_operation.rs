//! Submodule providing an operation trait that requires the user is authenticated.

use crate::prelude::Operation;

/// Trait for authenticated operations.
pub trait AuthenticatedOperation: Operation {
    /// The session type associated with the operation.
    type User;
    /// The authenticated operation type associated with the operation.
    type Operation: Operation;

    /// Uses the session to authenticate the operation.
    fn authenticate(self, session_user: &Self::User) -> Self::Operation;
}
