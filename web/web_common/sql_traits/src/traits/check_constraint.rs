//! Submodule definining the `CheckConstraintLike` trait for SQL check
//! constraints.

/// A check constraint is a rule that specifies a condition that must be met
/// for data to be inserted or updated in a table. This trait represents such
/// a check constraint in a database-agnostic way.
pub trait CheckConstraintLike {
    /// Returns the clause of the check constraint as a string.
    fn clause(&self) -> String;
}
