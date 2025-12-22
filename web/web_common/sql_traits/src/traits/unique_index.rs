//! Submodule definining the `UniqueIndexLike` trait for SQL unique
//! indexes.

use std::fmt::Debug;

use sqlparser::ast::{Expr, Ident};

use crate::traits::{DatabaseLike, Metadata, TableLike};

pub(crate) fn collect_column_idents(expr: &Expr) -> Vec<&Ident> {
    match expr {
        Expr::Identifier(ident) => vec![ident],
        Expr::CompoundIdentifier(idents) => idents.iter().collect(),
        Expr::Tuple(exprs) => {
            let mut idents = Vec::new();
            for expr in exprs {
                idents.extend(collect_column_idents(expr));
            }
            idents
        }
        Expr::BinaryOp { left, right, .. } => {
            let mut cols = collect_column_idents(left);
            cols.extend(collect_column_idents(right));
            cols
        }
        Expr::UnaryOp { expr, .. } => collect_column_idents(expr),
        Expr::Nested(inner) => collect_column_idents(inner),
        _ => unimplemented!("Unhandled expression type in unique index: {:?}", expr),
    }
}

/// A unique index is a rule that specifies that the values in a column
/// (or a group of columns) must be unique across all rows in a table.
/// This trait represents such a unique index in a database-agnostic way.
pub trait UniqueIndexLike: Metadata + Ord + Eq + Debug + Clone {
    /// The database type the unique index belongs to.
    type DB: DatabaseLike<UniqueIndex = Self>;

    /// Returns the expression of the unique index as an SQL AST node.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db =
    ///     ParserDB::try_from(r#"CREATE TABLE my_table (id INT UNIQUE, name TEXT, UNIQUE (name));"#)?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let unique_indices: Vec<_> = table.unique_indices(&db).collect();
    /// let expressions: Vec<String> =
    ///     unique_indices.iter().map(|ui| ui.expression(&db).to_string()).collect();
    /// assert_eq!(expressions.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    fn expression<'db>(&'db self, database: &'db Self::DB) -> &'db Expr
    where
        Self: 'db;

    /// Returns a reference to the table this unique index belongs to.
    fn table<'db>(&'db self, database: &'db Self::DB) -> &'db <Self::DB as DatabaseLike>::Table
    where
        Self: 'db;

    /// Returns whether this unique index is also the primary key of the table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT, UNIQUE (name));"#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let unique_indices: Vec<_> = table.unique_indices(&db).collect();
    /// let primary_key_flags: Vec<bool> =
    ///     unique_indices.iter().map(|ui| ui.is_primary_key(&db)).collect();
    /// assert_eq!(primary_key_flags, vec![true, false]);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn is_primary_key(&self, database: &Self::DB) -> bool {
        self.table(database).primary_key_columns(database).eq(self.columns(database))
    }

    /// Returns whether the unique index is defined using simply columns
    /// and no other expressions.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db =
    ///     ParserDB::try_from(r#"CREATE TABLE my_table (id INT UNIQUE, name TEXT, UNIQUE (name));"#)?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let unique_indices: Vec<_> = table.unique_indices(&db).collect();
    /// let simple_flags: Vec<bool> = unique_indices.iter().map(|ui| ui.is_simple(&db)).collect();
    /// assert!(
    ///     simple_flags.iter().all(|&flag| flag),
    ///     "All unique indices should be simple column-based"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn is_simple(&self, database: &Self::DB) -> bool {
        let expr = self.expression(database);
        let inner_expr = match expr {
            Expr::Nested(inner) => inner,
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
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE my_table (id INT, name TEXT, UNIQUE (id), UNIQUE (name, id));"#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let unique_indices: Vec<_> = table.unique_indices(&db).collect();
    /// let single_column_ui = &unique_indices[0];
    /// let multi_column_ui = &unique_indices[1];
    /// let single_columns: Vec<&str> =
    ///     single_column_ui.columns(&db).map(|col| col.column_name()).collect();
    /// let multi_columns: Vec<&str> =
    ///     multi_column_ui.columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(single_columns, vec!["id"]);
    /// assert_eq!(multi_columns, vec!["name", "id"]);
    /// # Ok(())
    /// # }
    /// ```
    fn columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        let table = <Self as UniqueIndexLike>::table(self, database);
        let expr = self.expression(database);
        let column_idents = match expr {
            Expr::Nested(inner) => collect_column_idents(inner),
            _ => collect_column_idents(expr),
        };
        column_idents.into_iter().filter_map(|ident| table.column(&ident.value, database))
    }
}
