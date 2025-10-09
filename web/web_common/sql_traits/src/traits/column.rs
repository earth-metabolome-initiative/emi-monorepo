//! Submodule providing a trait for describing SQL Column-like entities.

use std::hash::Hash;

/// A trait for types that can be treated as SQL columns.
pub trait ColumnLike: Hash + Eq + Ord {
    /// Returns the name of the column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table");
    /// let columns: Vec<&str> = table.columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(columns, vec!["id", "name"]);
    /// # Ok(())
    /// # }
    /// ```
    fn column_name(&self) -> &str;

    /// Returns the data type of the column as a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     "CREATE TABLE my_table (id INT, name TEXT, score DECIMAL(10,2));",
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column_by_name("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column_by_name("name", &db).expect("Column 'name' should exist");
    /// let score_column = table.column_by_name("score", &db).expect("Column 'score' should exist");
    /// assert_eq!(id_column.data_type(), "INT");
    /// assert_eq!(name_column.data_type(), "TEXT");
    /// assert_eq!(score_column.data_type(), "DECIMAL(10,2)");
    /// # Ok(())
    /// # }
    /// ```
    fn data_type(&self) -> String;

    /// Returns whether the column is nullable.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     "CREATE TABLE my_table (id INT NOT NULL, name TEXT, optional_field INT);",
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column_by_name("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column_by_name("name", &db).expect("Column 'name' should exist");
    /// let optional_column =
    ///     table.column_by_name("optional_field", &db).expect("Column 'optional_field' should exist");
    /// assert!(!id_column.is_nullable(), "id column should not be nullable");
    /// assert!(name_column.is_nullable(), "name column should be nullable by default");
    /// assert!(optional_column.is_nullable(), "optional_field column should be nullable by default");
    /// # Ok(())
    /// # }
    /// ```
    fn is_nullable(&self) -> bool;
}
