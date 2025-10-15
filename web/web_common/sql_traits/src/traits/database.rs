//! Submodule providing a trait for describing SQL Database-like entities.

use crate::traits::{ColumnLike, ForeignKeyLike, FunctionLike, TableLike};

/// A trait for types that can be treated as SQL databases.
pub trait DatabaseLike {
    /// Type of the tables in the schema.
    type Table: TableLike<Column = Self::Column, Database = Self, ForeignKey = Self::ForeignKey>;
    /// Type of the columns in the schema.
    type Column: ColumnLike;
    /// Type of the foreign keys in the schema.
    type ForeignKey: ForeignKeyLike<Table = Self::Table, Column = Self::Column, Database = Self>;
    /// Type of the functions in the schema.
    type Function: FunctionLike;

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

    /// Iterates over the functions created in the database.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// 
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add_one(x INT) RETURNS INT AS 'SELECT x + 1;';
    /// CREATE FUNCTION greet(name TEXT) RETURNS TEXT AS 'SELECT "Hello, " || name;';
    /// "#,
    /// )?;
    /// let function_names: Vec<&str> = db.functions().map(|f| f.name()).collect();
    /// assert_eq!(function_names, vec!["add_one", "greet"]);
    /// # Ok(())
    /// # }
    /// ```
    fn functions(&self) -> impl Iterator<Item = &Self::Function>;

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
    fn table(&self, schema: Option<&str>, table_name: &str) -> &Self::Table;

    /// Returns the function with the given name.
    ///
    /// # Arguments
    /// 
    /// * `name` - Name of the function.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add_one(x INT) RETURNS INT AS 'SELECT x + 1;';
    /// "#,
    /// )?;
    /// let add_one = db.function("add_one").expect("Function 'add_one' should exist");
    /// assert_eq!(add_one.name(), "add_one");
    /// let non_existent = db.function("non_existent");
    /// assert!(non_existent.is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn function(&self, name: &str) -> Option<&Self::Function>;
}
