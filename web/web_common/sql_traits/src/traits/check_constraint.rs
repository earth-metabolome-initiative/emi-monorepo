//! Submodule definining the `CheckConstraintLike` trait for SQL check
//! constraints.

use std::{borrow::Borrow, fmt::Debug};

use sqlparser::ast::{Expr, Ident};

use crate::traits::{DatabaseLike, Metadata, column::ColumnLike, function_like::FunctionLike};

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

    /// Returns a reference to the table that the check constraint is defined
    /// on.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the table
    ///   from.
    fn table<'db>(&'db self, database: &'db Self::DB) -> &'db <Self::DB as DatabaseLike>::Table;

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

    /// Returns a reference to the requested column by name, if any.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the columns
    ///  from.
    /// * `name` - The name of the column to retrieve.
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
    ///
    /// let table = db.table(None, "my_table").unwrap();
    /// let columns = table.columns(&db).collect::<Vec<_>>();
    /// let [id, name] = &columns.as_slice() else {
    ///     panic!("Expected two columns");
    /// };
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2] = &check_constraints.as_slice() else {
    ///     panic!("Expected two check constraints");
    /// };
    /// let col = cc1.column(&db, "id").unwrap();
    /// assert_eq!(col, *id);
    /// assert!(cc2.column(&db, "id").is_none());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn column<'db>(
        &'db self,
        database: &'db Self::DB,
        name: &str,
    ) -> Option<&'db <Self::DB as DatabaseLike>::Column> {
        self.columns(database).find(|c| c.column_name() == name)
    }

    /// Iterates over the functions used in the check constraint.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the
    ///   functions from.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE FUNCTION is_positive(INT) RETURNS BOOLEAN;
    ///        CREATE TABLE my_table (id INT CHECK (is_positive(id)));"#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc] = &check_constraints.as_slice() else {
    ///     panic!("Expected one check constraint");
    /// };
    /// let functions: Vec<_> = cc.functions(&db).collect();
    /// assert_eq!(functions.len(), 1);
    /// assert_eq!(functions[0].name(), "is_positive");
    /// # Ok(())
    /// # }
    /// ```
    fn functions<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Function> + 'db;

    /// Returns a reference to the requested function by name, if any.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the
    ///   functions
    ///  from.
    /// * `name` - The name of the function to retrieve.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE FUNCTION is_positive(INT) RETURNS BOOLEAN;
    ///        CREATE TABLE my_table (id INT CHECK (is_positive(id)), age INT CHECK (age > 0));"#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2] = &check_constraints.as_slice() else {
    ///     panic!("Expected two check constraints");
    /// };
    /// let func = cc1.function(&db, "is_positive").unwrap();
    /// assert_eq!(func.name(), "is_positive");
    /// assert!(cc2.function(&db, "is_positive").is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn function<'db>(
        &'db self,
        database: &'db Self::DB,
        name: &str,
    ) -> Option<&'db <Self::DB as DatabaseLike>::Function> {
        self.functions(database).find(|f| f.name() == name)
    }

    /// Returns whether the check constraint involves any functions.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the
    ///   functions
    ///  from.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE FUNCTION is_positive(INT) RETURNS BOOLEAN;
    ///        CREATE TABLE my_table (id INT CHECK (is_positive(id)), age INT CHECK (age > 0));"#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2] = &check_constraints.as_slice() else {
    ///     panic!("Expected two check constraints");
    /// };
    /// assert!(cc1.has_functions(&db));
    /// assert!(!cc2.has_functions(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_functions(&self, database: &Self::DB) -> bool {
        self.functions(database).next().is_some()
    }

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

    /// Returns whether the check constraint is a mutual nullability constraint.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the table
    ///  from.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE my_table (
    ///         col1 INT,
    ///         col2 INT,
    ///         col3 INT,
    ///         CHECK ((col1 IS NULL AND col2 IS NULL) OR (col2 IS NOT NULL AND col1 IS NOT NULL)),
    ///         CHECK ((col1 IS NULL AND col2 IS NULL AND col3 IS NULL) OR (col1 IS NOT NULL AND col3 IS NOT NULL AND col2 IS NOT NULL)),
    ///         CHECK ((col3 IS NULL OR col3 IS NULL) OR (col3 IS NOT NULL AND col1 IS NOT NULL)),
    ///         CHECK (col1 IS NOT NULL),
    ///         CHECK (col1 > 0),
    ///         CHECK (col2 < 100)
    ///     );"#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2, cc3, cc4, cc5, cc6] = &check_constraints.as_slice() else {
    ///     panic!("Expected six check constraints");
    /// };
    /// assert!(cc1.is_mutual_nullability_constraint(&db));
    /// assert!(cc2.is_mutual_nullability_constraint(&db));
    /// assert!(!cc3.is_mutual_nullability_constraint(&db));
    /// assert!(!cc4.is_mutual_nullability_constraint(&db));
    /// assert!(!cc5.is_mutual_nullability_constraint(&db));
    /// assert!(!cc6.is_mutual_nullability_constraint(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn is_mutual_nullability_constraint(&self, database: &Self::DB) -> bool {
        use sqlparser::ast::{BinaryOperator, Expr};

        let expr = self.expression(database);

        // Must be an OR expression
        let Expr::BinaryOp { left, op: BinaryOperator::Or, right } = expr else {
            return false;
        };

        // Helper to extract column names from nullability checks in an AND chain
        fn extract_null_columns(expr: &Expr, is_null: bool) -> Option<Vec<&Ident>> {
            match expr {
                Expr::IsNull(col_expr) if is_null => {
                    if let Expr::Identifier(ident) = col_expr.as_ref() {
                        Some(vec![ident])
                    } else {
                        None
                    }
                }
                Expr::IsNotNull(col_expr) if !is_null => {
                    if let Expr::Identifier(ident) = col_expr.as_ref() {
                        Some(vec![ident])
                    } else {
                        None
                    }
                }
                Expr::BinaryOp { left, op: BinaryOperator::And, right } => {
                    let mut left_cols = extract_null_columns(left, is_null)?;
                    left_cols.extend(extract_null_columns(right, is_null)?);
                    left_cols.sort_unstable();
                    left_cols.dedup();
                    Some(left_cols)
                }
                Expr::Nested(inner) => extract_null_columns(inner, is_null),
                _ => None,
            }
        }

        // Left side should be all NULL checks, right side all NOT NULL checks (or vice
        // versa)
        if let (Some(null_cols), Some(not_null_cols)) =
            (extract_null_columns(left, true), extract_null_columns(right, false))
        {
            null_cols == not_null_cols && null_cols.len() >= 2
        } else if let (Some(not_null_cols), Some(null_cols)) =
            (extract_null_columns(left, false), extract_null_columns(right, true))
        {
            null_cols == not_null_cols && null_cols.len() >= 2
        } else {
            false
        }
    }
}
