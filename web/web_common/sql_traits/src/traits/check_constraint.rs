//! Submodule definining the `CheckConstraintLike` trait for SQL check
//! constraints.

use sqlparser::ast::Expr;

/// A check constraint is a rule that specifies a condition that must be met
/// for data to be inserted or updated in a table. This trait represents such
/// a check constraint in a database-agnostic way.
pub trait CheckConstraintLike {
    /// Returns the expression of the check constraint as an SQL AST node.
    fn expression(&self) -> Expr;
}
