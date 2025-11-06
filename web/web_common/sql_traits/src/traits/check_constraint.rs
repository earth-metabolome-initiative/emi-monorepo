//! Submodule definining the `CheckConstraintLike` trait for SQL check
//! constraints.

use std::{borrow::Borrow, fmt::Debug};

use sqlparser::ast::Expr;

use crate::traits::{DatabaseLike, Metadata};

/// A check constraint is a rule that specifies a condition that must be met
/// for data to be inserted or updated in a table. This trait represents such
/// a check constraint in a database-agnostic way.
pub trait CheckConstraintLike:
    Clone
    + Eq
    + Ord
    + Debug
    + Metadata
    + Borrow<<<Self as CheckConstraintLike>::DB as DatabaseLike>::CheckConstraint>
{
    /// The type of the database that this column belongs to.
    type DB: DatabaseLike<CheckConstraint: Borrow<Self>>;

    /// Returns the expression of the check constraint as an SQL AST node.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the check
    ///   constraint from.
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
    /// let table = db.table(None, "my_table").unwrap();
    /// let check_constraints: Vec<_> =
    ///     table.check_constraints(&db).map(|cc| cc.expression(&db).to_string()).collect();
    /// assert_eq!(check_constraints, vec!["id > 0", "length(name) > 0"]);
    /// # Ok(())
    /// # }
    /// ```
    fn expression<'db>(&'db self, database: &'db Self::DB) -> &'db Expr;

    /// Iterates over the columns involved in the check constraint.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the columns
    ///   from.
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
    /// let table = db.table(None, "my_table").unwrap();
    /// let columns = table.columns(&db).collect::<Vec<_>>();
    /// let [id, name] = &columns.as_slice() else {
    ///     panic!("Expected two columns");
    /// };
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2] = &check_constraints.as_slice() else {
    ///     panic!("Expected two check constraints");
    /// };
    /// assert_eq!(cc1.columns(&db).collect::<Vec<_>>(), vec![*id]);
    /// assert_eq!(cc2.columns(&db).collect::<Vec<_>>(), vec![*name]);
    /// # Ok(())
    /// # }
    /// ```
    fn columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>;

    /// Returns whether the check constraint involves a specific column.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the table
    ///   from.
    /// * `column` - A reference to the column to check for involvement.
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
    /// let table = db.table(None, "my_table").unwrap();
    /// let columns = table.columns(&db).collect::<Vec<_>>();
    /// let [id, name] = &columns.as_slice() else {
    ///     panic!("Expected two columns");
    /// };
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2] = &check_constraints.as_slice() else {
    ///     panic!("Expected two check constraints");
    /// };
    /// assert!(cc1.involves_column(&db, id));
    /// assert!(!cc1.involves_column(&db, name));
    /// assert!(!cc2.involves_column(&db, id));
    /// assert!(cc2.involves_column(&db, name));
    /// # Ok(())
    /// # }
    /// ```
    fn involves_column(
        &self,
        database: &Self::DB,
        column: &<Self::DB as DatabaseLike>::Column,
    ) -> bool {
        self.columns(database).any(|col| col == column)
    }
}
