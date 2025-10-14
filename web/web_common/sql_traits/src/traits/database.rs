//! Submodule providing a trait for describing SQL Database-like entities.

use crate::traits::{ColumnLike, ForeignKeyLike, TableLike};

/// A trait for types that can be treated as SQL databases.
pub trait DatabaseLike {
    /// Type of the tables in the schema.
    type Table: TableLike<Column = Self::Column, Database = Self, ForeignKey = Self::ForeignKey>;
    /// Type of the columns in the schema.
    type Column: ColumnLike;
    /// Type of the foreign keys in the schema.
    type ForeignKey: ForeignKeyLike<Table = Self::Table, Column = Self::Column, Database = Self>;

    /// Iterates over the tables defined in the schema.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE table1 (id INT);
    /// CREATE TABLE table2 (name TEXT);
    /// CREATE TABLE table3 (score DECIMAL);
    /// "#,
    /// )?;
    /// let table_names: Vec<&str> = db.tables().map(|t| t.table_name()).collect();
    /// assert_eq!(table_names, vec!["table1", "table2", "table3"]);
    /// # Ok(())
    /// # }
    /// ```
    fn tables(&self) -> impl Iterator<Item = &Self::Table>;

    /// Returns the table with the given (optional) schema and name.
    ///
    /// # Arguments
    ///
    /// * `schema` - Optional schema name of the table.
    /// * `table_name` - Name of the table.
    ///
    /// # Panics
    ///
    /// Panics if the table is not found in the database.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_schema.my_table_with_schema (id INT);
    /// CREATE TABLE my_table (id INT);
    /// "#,
    /// )?;
    /// let table_with_schema = db.table(Some("my_schema"), "my_table_with_schema");
    /// assert_eq!(table_with_schema.table_name(), "my_table_with_schema");
    /// assert_eq!(table_with_schema.table_schema(), Some("my_schema"));
    ///
    /// let table_without_schema = db.table(None, "my_table");
    /// assert_eq!(table_without_schema.table_name(), "my_table");
    /// assert_eq!(table_without_schema.table_schema(), None);
    /// # Ok(())
    /// # }
    /// ```
    fn table(&self, database: Option<&str>, table_name: &str) -> &Self::Table;
}
