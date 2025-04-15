//! Submodule defining a schema for the translation between `PostgreSQL` and
//! `SQLite`.

/// Trait to define a schema for the translation between `PostgreSQL` and
/// `SQLite`.
pub trait Schema {
    /// Returns whether a non-strictly `PostgreSQL` function is available in the
    /// Schema.
    fn has_function(&self, name: &str) -> bool;

    /// Returns whether to drop CHECK constraints containing unsupported
    /// functions.
    fn should_remove_unsupported_check_constraints(&self) -> bool;
}
