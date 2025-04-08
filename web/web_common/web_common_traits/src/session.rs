//! Submodule defining a trait that characterizes a session.

/// Trait for sessions.
pub trait Session {
    /// The user type associated with the session.
    type User;

    /// Returns whether the session is authenticated.
    fn is_authenticated(&self) -> bool {
        self.user().is_some()
    }

    /// Returns the user associated with the session, if any.
    fn user(&self) -> Option<&Self::User>;
}
