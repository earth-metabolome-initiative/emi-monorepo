//! Submodule definining the `UniqueIndexLike` trait for SQL unique
//! indexes.

use sqlparser::ast::{Expr, Ident};

use crate::traits::{ColumnLike, DatabaseLike, TableLike};

pub(crate) fn collect_column_idents(expr: Expr) -> Vec<Ident> {
    match expr {
        Expr::Identifier(ident) => vec![ident],
        Expr::CompoundIdentifier(idents) => idents,
        Expr::Tuple(exprs) => {
            let mut idents = Vec::new();
            for expr in exprs {
                idents.extend(collect_column_idents(expr));
            }
            idents
        }
        Expr::BinaryOp { left, right, .. } => {
            let mut cols = collect_column_idents(*left);
            cols.extend(collect_column_idents(*right));
            cols
        }
        Expr::UnaryOp { expr, .. } => collect_column_idents(*expr),
        Expr::Nested(inner) => collect_column_idents(*inner),
        _ => unimplemented!("Unhandled expression type in unique index: {:?}", expr),
    }
}

/// A unique index is a rule that specifies that the values in a column
/// (or a group of columns) must be unique across all rows in a table.
/// This trait represents such a unique index in a database-agnostic way.
pub trait UniqueIndexLike {
    /// The table type the unique index belongs to.
    type Table: TableLike<Database = Self::Database, UniqueIndex = Self, Column = Self::Column>;
    /// The column type the unique index belongs to.
    type Column: ColumnLike;
    /// The database type the unique index belongs to.
    type Database: DatabaseLike<Table = Self::Table, Column = Self::Column>;

    /// Returns the expression of the unique index as an SQL AST node.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"CREATE TABLE my_table (id INT UNIQUE, name TEXT, UNIQUE (name));"#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let unique_indices: Vec<_> = table.unique_indices(&db).collect();
    /// let expressions: Vec<String> =
    ///     unique_indices.iter().map(|ui| ui.expression(&db).to_string()).collect();
    /// assert_eq!(expressions.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    fn expression(&self, database: &Self::Database) -> Expr;

    /// Returns whether the unique index is defined using simply columns
    /// and no other expressions.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"CREATE TABLE my_table (id INT UNIQUE, name TEXT, UNIQUE (name));"#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let unique_indices: Vec<_> = table.unique_indices(&db).collect();
    /// let simple_flags: Vec<bool> = unique_indices.iter().map(|ui| ui.is_simple(&db)).collect();
    /// assert!(
    ///     simple_flags.iter().all(|&flag| flag),
    ///     "All unique indices should be simple column-based"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_simple(&self, database: &Self::Database) -> bool {
        let expr = self.expression(database);
        let inner_expr = match expr {
            Expr::Nested(inner) => *inner,
            _ => expr,
        };
        matches!(inner_expr, Expr::Identifier(_) | Expr::CompoundIdentifier(_) | Expr::Tuple(_))
    }

    /// Returns the columns which appear in the unique index.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"CREATE TABLE my_table (id INT, name TEXT, UNIQUE (id), UNIQUE (name, id));"#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let unique_indices: Vec<_> = table.unique_indices(&db).collect();
    /// let single_column_ui = &unique_indices[0];
    /// let multi_column_ui = &unique_indices[1];
    /// let single_columns: Vec<&str> =
    ///     single_column_ui.columns(&db, &table).map(|col| col.column_name()).collect();
    /// let multi_columns: Vec<&str> =
    ///     multi_column_ui.columns(&db, &table).map(|col| col.column_name()).collect();
    /// assert_eq!(single_columns, vec!["id"]);
    /// assert_eq!(multi_columns, vec!["name", "id"]);
    /// # Ok(())
    /// # }
    /// ```
    fn columns<'db>(
        &'db self,
        database: &'db Self::Database,
        table: &'db Self::Table,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        let expr = self.expression(database);
        let column_idents = match expr {
            Expr::Nested(inner) => collect_column_idents(*inner),
            _ => collect_column_idents(expr),
        };
        column_idents.into_iter().filter_map(|ident| table.column_by_name(&ident.value, database))
    }
}
