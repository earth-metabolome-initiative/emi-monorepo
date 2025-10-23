//! Submodule providing a trait for describing SQL Column-like entities.

use std::hash::Hash;

use crate::traits::{DatabaseLike, ForeignKeyLike, Metadata, TableLike};

/// A trait for types that can be treated as SQL columns.
pub trait ColumnLike: Hash + Eq + Ord + Metadata {
    /// The type of the foreign keys associated with this column.
    type ForeignKey: ForeignKeyLike<Column = Self, Database = Self::Database, Table = Self::Table>;
    /// The type of the table that this column belongs to.
    type Table: TableLike<Database = Self::Database, Column = Self, ForeignKey = Self::ForeignKey>;
    /// The type of the database that this column belongs to.
    type Database: DatabaseLike<Table = Self::Table, Column = Self, ForeignKey = Self::ForeignKey>;

    /// Returns the name of the column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table");
    /// let columns: Vec<&str> = table.columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(columns, vec!["id", "name"]);
    /// # Ok(())
    /// # }
    /// ```
    fn column_name(&self) -> &str;

    /// Returns the documentation of the column, if any.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the column
    ///   documentation from.
    fn column_doc<'db>(&'db self, database: &'db Self::Database) -> Option<&'db str>
    where
        Self: 'db;

    /// Returns the data type of the column as a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT, score DECIMAL(10,2));")?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let score_column = table.column("score", &db).expect("Column 'score' should exist");
    /// assert_eq!(id_column.data_type(), "INT");
    /// assert_eq!(name_column.data_type(), "TEXT");
    /// assert_eq!(score_column.data_type(), "DECIMAL(10,2)");
    /// # Ok(())
    /// # }
    /// ```
    fn data_type(&self) -> String;

    /// Returns whether the data type of the column is generative, i.e., it
    /// generates values automatically (e.g., SERIAL in PostgreSQL).
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id SERIAL, name TEXT, age INT, bigg_id BIGSERIAL);",
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let age_column = table.column("age", &db).expect("Column 'age' should exist");
    /// let bigg_id_column = table.column("bigg_id", &db).expect("Column 'bigg_id' should exist");
    /// assert!(id_column.is_generated(), "id column should be generative");
    /// assert!(!name_column.is_generated(), "name column should not be generative");
    /// assert!(!age_column.is_generated(), "age column should not be generative");
    /// assert!(bigg_id_column.is_generated(), "bigg_id column should be generative");
    /// # Ok(())
    /// # }
    /// ```
    fn is_generated(&self) -> bool;

    /// Returns the normalized data type of the column as a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id INT, serial_id SERIAL, bigg_id BIGSERIAL, small_id SMALLSERIAL, name TEXT);",
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let serial_id_column = table.column("serial_id", &db).expect("Column 'serial_id' should exist");
    /// let bigg_id_column = table.column("bigg_id", &db).expect("Column 'bigg_id' should exist");
    /// let small_id_column = table.column("small_id", &db).expect("Column 'small_id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// assert_eq!(id_column.normalized_data_type(), "INT");
    /// assert_eq!(serial_id_column.normalized_data_type(), "INT");
    /// assert_eq!(bigg_id_column.normalized_data_type(), "BIGINT");
    /// assert_eq!(small_id_column.normalized_data_type(), "SMALLINT");
    /// assert_eq!(name_column.normalized_data_type(), "TEXT");
    /// # Ok(())
    /// # }
    /// ```
    fn normalized_data_type(&self, database: &Self::Database) -> String;

    /// Returns whether the column is nullable.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id INT NOT NULL, name TEXT, optional_field INT);",
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let optional_column =
    ///     table.column("optional_field", &db).expect("Column 'optional_field' should exist");
    /// assert!(!id_column.is_nullable(), "id column should not be nullable");
    /// assert!(name_column.is_nullable(), "name column should be nullable by default");
    /// assert!(optional_column.is_nullable(), "optional_field column should be nullable by default");
    /// # Ok(())
    /// # }
    /// ```
    fn is_nullable(&self) -> bool;

    /// Returns whether the column has a default value.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id INT DEFAULT 0, name TEXT, created_at TIMESTAMP DEFAULT NOW());",
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let created_at_column =
    ///     table.column("created_at", &db).expect("Column 'created_at' should exist");
    /// assert!(id_column.has_default(), "id column should have a default value");
    /// assert!(!name_column.has_default(), "name column should not have a default value");
    /// assert!(created_at_column.has_default(), "created_at column should have a default value");
    /// # Ok(())
    /// # }
    /// ```
    fn has_default(&self) -> bool;

    /// Returns the table that this column belongs to.
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
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let column_table = ColumnLike::table(id_column, &db);
    /// assert_eq!(column_table.table_name(), "my_table");
    /// # Ok(())
    /// # }
    /// ```
    fn table<'db>(&'db self, database: &'db Self::Database) -> &'db Self::Table
    where
        Self: 'db;

    /// Returns the foreign keys associated with this column.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query foreign
    ///   keys from.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let id_column = host_table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = host_table.column("name", &db).expect("Column 'name' should exist");
    /// let id_fks = id_column.foreign_keys(&db).collect::<Vec<_>>();
    /// let name_fks = name_column.foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(id_fks.len(), 1);
    /// assert_eq!(name_fks.len(), 0);
    /// # Ok(())
    /// # }
    /// ```
    fn foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db,
    {
        ColumnLike::table(self, database)
            .foreign_keys(database)
            .filter(move |fk| fk.host_columns(database).any(|col| col == self))
    }
}
