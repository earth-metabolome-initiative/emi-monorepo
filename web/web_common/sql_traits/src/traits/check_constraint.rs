//! Submodule definining the `CheckConstraintLike` trait for SQL check
//! constraints.

use std::{borrow::Borrow, fmt::Debug};

use sqlparser::ast::{BinaryOperator, Expr, Ident, Value};

use crate::traits::{DatabaseLike, Metadata, column::ColumnLike, function_like::FunctionLike};

/// Helper function to determine if an expression evaluates to a constant
/// boolean value. Returns `Some(true)` if always true, `Some(false)` if always
/// false, and `None` otherwise.
fn evaluate_constant_expr<DB: DatabaseLike>(
    database: &DB,
    columns: &[&<DB as DatabaseLike>::Column],
    expr: &Expr,
) -> Option<bool> {
    match expr {
        // Literal true/false
        Expr::Value(value_with_span) => {
            match value_with_span.value {
                Value::Boolean(true) => Some(true),
                Value::Boolean(false) => Some(false),
                _ => None,
            }
        }

        Expr::IsNotNull(col_expr) => {
            // Check if the column is declared NOT NULL in the table schema
            if let Expr::Identifier(ident) = col_expr.as_ref() {
                for column in columns.iter() {
                    if column.column_name() == ident.value {
                        // If column is NOT NULL, IS NOT NULL is TRUE.
                        // If column is NULLABLE, IS NOT NULL is variable (None).
                        return if !column.is_nullable(database) { Some(true) } else { None };
                    }
                }
                None
            } else {
                None
            }
        }

        Expr::IsNull(col_expr) => {
            // Check if the column is declared NOT NULL in the table schema
            if let Expr::Identifier(ident) = col_expr.as_ref() {
                for column in columns.iter() {
                    if column.column_name() == ident.value {
                        // If column is NOT NULL, IS NULL is FALSE.
                        // If column is NULLABLE, IS NULL is variable (None).
                        return if !column.is_nullable(database) { Some(false) } else { None };
                    }
                }
                None
            } else {
                None
            }
        }

        // Nested expressions
        Expr::Nested(inner) => evaluate_constant_expr(database, columns, inner),

        // Binary operations
        Expr::BinaryOp { left, op, right } => {
            // Check for patterns like 1 = 1, 0 = 0, etc.
            if matches!(op, BinaryOperator::Eq) {
                if let (Expr::Value(left_val), Expr::Value(right_val)) =
                    (left.as_ref(), right.as_ref())
                {
                    return Some(left_val.value == right_val.value);
                }
            }

            // Check for IS NULL OR IS NOT NULL pattern (always true)
            if matches!(op, BinaryOperator::Or) {
                if let (Expr::IsNull(null_col), Expr::IsNotNull(not_null_col)) =
                    (left.as_ref(), right.as_ref())
                {
                    if null_col == not_null_col {
                        return Some(true);
                    }
                }
                if let (Expr::IsNotNull(not_null_col), Expr::IsNull(null_col)) =
                    (left.as_ref(), right.as_ref())
                {
                    if null_col == not_null_col {
                        return Some(true);
                    }
                }
            }

            // Recursively check if both sides are tautological for AND
            if matches!(op, BinaryOperator::And) {
                return match (
                    evaluate_constant_expr(database, columns, left),
                    evaluate_constant_expr(database, columns, right),
                ) {
                    (Some(true), Some(true)) => Some(true),
                    (Some(false), _) | (_, Some(false)) => Some(false),
                    _ => None,
                };
            }

            // Recursively check if either side is tautological for OR
            if matches!(op, BinaryOperator::Or) {
                return match (
                    evaluate_constant_expr(database, columns, left),
                    evaluate_constant_expr(database, columns, right),
                ) {
                    (Some(true), _) | (_, Some(true)) => Some(true),
                    (Some(false), Some(false)) => Some(false),
                    _ => None,
                };
            }

            None
        }

        // NOT false is true, NOT true is false
        Expr::UnaryOp { op: sqlparser::ast::UnaryOperator::Not, expr } => {
            match evaluate_constant_expr(database, columns, expr) {
                Some(true) => Some(false),
                Some(false) => Some(true),
                None => None,
            }
        }

        // Everything else is not obviously tautological
        _ => None,
    }
}

/// Helper to extract column names from nullability checks in an AND chain
fn extract_null_columns(expr: &Expr, is_null: bool) -> Option<Vec<&Ident>> {
    use sqlparser::ast::BinaryOperator;
    match expr {
        Expr::IsNull(col_expr) if is_null => {
            if let Expr::Identifier(ident) = col_expr.as_ref() {
                Some(vec![ident])
            } else {
                None
            }
        }
        Expr::IsNotNull(col_expr) if !is_null => {
            if let Expr::Identifier(ident) = col_expr.as_ref() { Some(vec![ident]) } else { None }
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

    /// Returns the number of columns involved in the check constraint.
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
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2] = &check_constraints.as_slice() else {
    ///     panic!("Expected two check constraints");
    /// };
    /// assert_eq!(cc1.number_of_columns(&db), 1);
    /// assert_eq!(cc2.number_of_columns(&db), 1);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn number_of_columns(&self, database: &Self::DB) -> usize {
        self.columns(database).count()
    }

    /// Returns a reference to the requested column by name, if any.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the columns
    ///   from.
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
    ///   functions from.
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

    /// Returns whether the check constraint is a tautology (always true).
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the table
    ///   from.
    ///
    /// # Implementation Note
    ///
    /// This method recognizes several tautological patterns:
    /// - `CHECK (TRUE)` - literal true
    /// - `CHECK (1 = 1)`, `CHECK (0 = 0)` - equal constant comparisons
    /// - `CHECK (NOT FALSE)` - negated false
    /// - `CHECK (column IS NOT NULL)` for `NOT NULL` columns
    /// - `CHECK (column IS NULL OR column IS NOT NULL)` - always true for any
    ///   column
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE my_table (
    ///         col1 INT NOT NULL,
    ///         col2 INT,
    ///         CHECK (col1 IS NOT NULL),
    ///         CHECK (TRUE),
    ///         CHECK (1 = 1),
    ///         CHECK (NOT FALSE),
    ///         CHECK (col2 IS NOT NULL),
    ///         CHECK (col2 IS NULL OR col2 IS NOT NULL),
    ///         CHECK (col1 IS NULL OR col2 IS NOT NULL)
    ///     );"#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2, cc3, cc4, cc5, cc6, cc7] = &check_constraints.as_slice() else {
    ///     panic!("Expected seven check constraints");
    /// };
    /// assert!(cc1.is_tautology(&db)); // col1 IS NOT NULL on NOT NULL column
    /// assert!(cc2.is_tautology(&db)); // TRUE
    /// assert!(cc3.is_tautology(&db)); // 1 = 1
    /// assert!(cc4.is_tautology(&db)); // NOT FALSE
    /// assert!(!cc5.is_tautology(&db)); // col2 IS NOT NULL on nullable column
    /// assert!(cc6.is_tautology(&db)); // IS NULL OR IS NOT NULL is always true
    /// assert!(!cc7.is_tautology(&db)); // mixed columns
    /// //
    /// # Ok(())
    /// # }
    /// ```
    fn is_tautology(&self, database: &Self::DB) -> bool {
        let columns = self.columns(database).collect::<Vec<_>>();
        let expr = self.expression(database);

        // First check using expression analysis
        if let Some(true) = evaluate_constant_expr(database, &columns, expr) {
            return true;
        }

        false
    }

    /// Returns whether the check constraint is a negation (always false).
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the table
    ///   from.
    ///
    /// # Implementation Note
    ///
    /// This method recognizes several negation patterns:
    /// - `CHECK (FALSE)` - literal false
    /// - `CHECK (1 = 0)` - unequal constant comparisons
    /// - `CHECK (NOT TRUE)` - negated true
    /// - `CHECK (column IS NULL)` for `NOT NULL` columns
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE my_table (
    ///         col1 INT NOT NULL,
    ///         col2 INT,
    ///         CHECK (col1 IS NULL),
    ///         CHECK (FALSE),
    ///         CHECK (1 = 0),
    ///         CHECK (NOT TRUE),
    ///         CHECK (col2 IS NULL)
    ///     );"#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let check_constraints: Vec<_> = table.check_constraints(&db).collect();
    /// let [cc1, cc2, cc3, cc4, cc5] = &check_constraints.as_slice() else {
    ///     panic!("Expected five check constraints");
    /// };
    /// assert!(cc1.is_negation(&db)); // col1 IS NULL on NOT NULL column
    /// assert!(cc2.is_negation(&db)); // FALSE
    /// assert!(cc3.is_negation(&db)); // 1 = 0
    /// assert!(cc4.is_negation(&db)); // NOT TRUE
    /// assert!(!cc5.is_negation(&db)); // col2 IS NULL on nullable column
    /// //
    /// # Ok(())
    /// # }
    /// ```
    fn is_negation(&self, database: &Self::DB) -> bool {
        let columns = self.columns(database).collect::<Vec<_>>();
        let expr = self.expression(database);

        // First check using expression analysis
        if let Some(false) = evaluate_constant_expr(database, &columns, expr) {
            return true;
        }

        false
    }

    /// Returns whether the check constraint is a mutual nullability constraint.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the table
    ///   from.
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
