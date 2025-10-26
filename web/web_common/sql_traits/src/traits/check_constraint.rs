//! Submodule definining the `CheckConstraintLike` trait for SQL check
//! constraints.

use std::fmt::Debug;

use sqlparser::ast::Expr;

/// A check constraint is a rule that specifies a condition that must be met
/// for data to be inserted or updated in a table. This trait represents such
/// a check constraint in a database-agnostic way.
pub trait CheckConstraintLike: Clone + Eq + Ord + Debug {
    /// Returns the expression of the check constraint as an SQL AST node.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE my_table (id INT CHECK (id > 0), name TEXT CHECK (length(name) > 0));"#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let check_constraints: Vec<_> =
    ///     table.check_constraints(&db).map(|cc| cc.expression().to_string()).collect();
    /// assert_eq!(check_constraints, vec!["id > 0", "length(name) > 0"]);
    /// # Ok(())
    /// # }
    /// ```
    fn expression(&self) -> Expr;
}
