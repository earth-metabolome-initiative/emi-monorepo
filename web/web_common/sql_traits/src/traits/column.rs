//! Submodule providing a trait for describing SQL Column-like entities.

use std::{borrow::Borrow, fmt::Debug, hash::Hash};

use crate::{
    traits::{CheckConstraintLike, DatabaseLike, ForeignKeyLike, Metadata, TableLike},
    utils::normalize_postgres_type,
};

/// A trait for types that can be treated as SQL columns.
pub trait ColumnLike:
    Debug
    + Clone
    + Hash
    + Eq
    + Ord
    + Metadata
    + Borrow<<<Self as ColumnLike>::DB as DatabaseLike>::Column>
{
    /// The type of the database that this column belongs to.
    type DB: DatabaseLike<Column: Borrow<Self>>;

    /// Returns the name of the column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table").unwrap();
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
    fn column_doc<'db>(&'db self, database: &'db Self::DB) -> Option<&'db str>
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
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT, score REAL);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let score_column = table.column("score", &db).expect("Column 'score' should exist");
    /// assert_eq!(id_column.data_type(&db), "INT");
    /// assert_eq!(name_column.data_type(&db), "TEXT");
    /// assert_eq!(score_column.data_type(&db), "REAL");
    /// # Ok(())
    /// # }
    /// ```
    fn data_type<'db>(&'db self, database: &'db Self::DB) -> &'db str;

    /// Returns whether the data type of the column is generative, i.e., it
    /// generates values automatically (e.g., SERIAL in `PostgreSQL`).
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE parent (id SERIAL, name TEXT, age INT, bigg_id BIGSERIAL);
    ///     CREATE TABLE child (parent_id INT PRIMARY KEY REFERENCES parent(id), other TEXT);"#,
    /// )?;
    /// let table = db.table(None, "parent").unwrap();
    /// let child_table = db.table(None, "child").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let age_column = table.column("age", &db).expect("Column 'age' should exist");
    /// let bigg_id_column = table.column("bigg_id", &db).expect("Column 'bigg_id' should exist");
    /// let parent_id_column =
    ///     child_table.column("parent_id", &db).expect("Column 'parent_id' should exist");
    /// let other_column = child_table.column("other", &db).expect("Column 'other' should exist");
    /// assert!(id_column.is_generated(), "id column should be generative");
    /// assert!(!name_column.is_generated(), "name column should not be generative");
    /// assert!(!age_column.is_generated(), "age column should not be generative");
    /// assert!(bigg_id_column.is_generated(), "bigg_id column should be generative");
    /// assert!(!parent_id_column.is_generated(), "parent_id column should be generative");
    /// assert!(!other_column.is_generated(), "other column should not be generative");
    /// # Ok(())
    /// # }
    /// ```
    fn is_generated(&self) -> bool;

    /// Returns whether the column is the primary key of its table (not just
    /// part of it).
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
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT, age INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let age_column = table.column("age", &db).expect("Column 'age' should exist");
    /// assert!(id_column.is_primary_key(&db), "id column should be primary key");
    /// assert!(!name_column.is_primary_key(&db), "name column should not be primary key");
    /// assert!(!age_column.is_primary_key(&db), "age column should not be primary key");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn is_primary_key(&self, database: &Self::DB) -> bool {
        let table: &<Self::DB as DatabaseLike>::Table = ColumnLike::table(self, database);
        table.is_primary_key_column(database, self.borrow())
    }

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
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let serial_id_column = table.column("serial_id", &db).expect("Column 'serial_id' should exist");
    /// let bigg_id_column = table.column("bigg_id", &db).expect("Column 'bigg_id' should exist");
    /// let small_id_column = table.column("small_id", &db).expect("Column 'small_id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// assert_eq!(id_column.normalized_data_type(&db), "INT");
    /// assert_eq!(serial_id_column.normalized_data_type(&db), "INT");
    /// assert_eq!(bigg_id_column.normalized_data_type(&db), "BIGINT");
    /// assert_eq!(small_id_column.normalized_data_type(&db), "SMALLINT");
    /// assert_eq!(name_column.normalized_data_type(&db), "TEXT");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn normalized_data_type<'db>(&'db self, database: &'db Self::DB) -> &'db str {
        normalize_postgres_type(self.data_type(database))
    }

    /// Returns whether the column type is textual.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query the column
    ///   data type from.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT, description VARCHAR);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let description_column =
    ///     table.column("description", &db).expect("Column 'description' should exist");
    /// assert!(!id_column.is_textual(&db), "id column should not be textual");
    /// assert!(name_column.is_textual(&db), "name column should be textual");
    /// assert!(description_column.is_textual(&db), "description column should be textual");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn is_textual(&self, database: &Self::DB) -> bool {
        matches!(self.normalized_data_type(database), "TEXT" | "VARCHAR" | "CHAR")
    }

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
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let optional_column =
    ///     table.column("optional_field", &db).expect("Column 'optional_field' should exist");
    /// assert!(!id_column.is_nullable(&db), "id column should not be nullable");
    /// assert!(name_column.is_nullable(&db), "name column should be nullable by default");
    /// assert!(
    ///     optional_column.is_nullable(&db),
    ///     "optional_field column should be nullable by default"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_nullable(&self, database: &Self::DB) -> bool;

    /// Returns the SQL default value of the column, if any.
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
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// let created_at_column =
    ///     table.column("created_at", &db).expect("Column 'created_at' should exist");
    /// assert_eq!(
    ///     id_column.default_value(),
    ///     Some("0".to_string()),
    ///     "id column should have default value 0"
    /// );
    /// assert_eq!(name_column.default_value(), None, "name column should not have a default value");
    /// assert_eq!(
    ///     created_at_column.default_value(),
    ///     Some("NOW()".to_string()),
    ///     "created_at column should have default value NOW()"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn default_value(&self) -> Option<String>;

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
    /// let table = db.table(None, "my_table").unwrap();
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
    fn has_default(&self) -> bool {
        self.default_value().is_some()
    }

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
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let column_table = ColumnLike::table(id_column, &db);
    /// assert_eq!(column_table.table_name(), "my_table");
    /// # Ok(())
    /// # }
    /// ```
    fn table<'db>(&'db self, database: &'db Self::DB) -> &'db <Self::DB as DatabaseLike>::Table
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
    /// let host_table = db.table(None, "host_table").unwrap();
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
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        ColumnLike::table(self, database).foreign_keys(database).filter(move |fk| {
            fk.host_columns(database).map(Borrow::borrow).any(|col: &Self| col == self)
        })
    }

    /// Returns whether the column references the given table or any of its
    /// descendants.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query foreign
    ///   keys from.
    /// * `table` - A reference to the table to check references against.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent (id INT PRIMARY KEY);
    /// CREATE TABLE child (
    ///     id INT PRIMARY KEY REFERENCES parent(id)
    /// );
    /// CREATE TABLE other (
    ///     id INT PRIMARY KEY,
    ///     child_id INT REFERENCES child(id)
    /// );
    /// "#,
    /// )?;
    /// let parent_table = db.table(None, "parent").unwrap();
    /// let child_table = db.table(None, "child").unwrap();
    /// let other_table = db.table(None, "other").unwrap();
    /// let child_id_column = other_table.column("child_id", &db).unwrap();
    /// assert!(
    ///     child_id_column.references_table_pk_or_descendant(&db, parent_table),
    ///     "child_id should reference parent or its descendant"
    /// );
    /// assert!(
    ///     child_id_column.references_table_pk_or_descendant(&db, child_table),
    ///     "child_id should reference child or its descendant"
    /// );
    /// assert!(
    ///     !child_id_column.references_table_pk_or_descendant(&db, other_table),
    ///     "child_id should not reference other or its descendant"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn references_table_pk_or_descendant(
        &self,
        database: &Self::DB,
        table: &<Self::DB as DatabaseLike>::Table,
    ) -> bool {
        self.foreign_keys(database).any(|fk| {
            if !fk.is_referenced_primary_key(database) || fk.is_composite(database) {
                return false;
            }
            let referenced_table = fk.referenced_table(database);
            referenced_table == table || referenced_table.is_descendant_of(database, table)
        })
    }

    /// Returns the extension foreign keys associated with this column.
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
    /// CREATE TABLE parent (id INT PRIMARY KEY);
    /// CREATE TABLE child (
    ///     parent_id INT PRIMARY KEY REFERENCES parent(id)
    /// );
    /// "#,
    /// )?;
    /// let parent_table = db.table(None, "parent").unwrap();
    /// let child_table = db.table(None, "child").unwrap();
    /// let parent_id_column =
    ///     child_table.column("parent_id", &db).expect("Column 'parent_id' should exist");
    /// let ext_fks = parent_id_column.extension_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(ext_fks.len(), 1);
    /// let id_column = parent_table.column("id", &db).expect("Column 'id' should exist");
    /// let id_fks = id_column.extension_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(id_fks.len(), 0);
    /// # Ok(())
    /// # }
    /// ```
    fn extension_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(move |fk| fk.is_extension_foreign_key(database))
    }

    /// Returns whether the column is a foreign key, i.e. it is part of any
    /// foreign key constraint.
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
    ///    id INT REFERENCES referenced_table(id),
    ///    name TEXT
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let id_column = host_table.column("id", &db).unwrap();
    /// let name_column = host_table.column("name", &db).unwrap();
    /// assert!(id_column.is_part_of_foreign_key(&db), "id column should be a foreign key");
    /// assert!(!name_column.is_part_of_foreign_key(&db), "name column should not be a foreign key");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn is_part_of_foreign_key(&self, database: &Self::DB) -> bool {
        self.foreign_keys(database).next().is_some()
    }

    /// Returns the non-composite foreign keys associated with this column.
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
    ///    id INT,
    ///    name TEXT,
    ///    FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let id_column = host_table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = host_table.column("name", &db).expect("Column 'name' should exist");
    /// let id_fks = id_column.non_composite_foreign_keys(&db).collect::<Vec<_>>();
    /// let name_fks = name_column.non_composite_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(id_fks.len(), 1);
    /// assert_eq!(name_fks.len(), 0);
    /// # Ok(())
    /// # }
    /// ```
    fn non_composite_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(move |fk| !fk.is_composite(database))
    }

    /// Returns whether the column is compatible with another column.
    ///
    /// # Implementation Note
    /// Two columns are considered compatible if:
    /// - They have the same data type.
    /// - If they are foreign keys, they reference the same table or share an
    ///   ancestor table.
    ///
    /// If both columns are not foreign keys, they are considered compatible if
    /// they have the same data type.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `other` - The column in the other table to check.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE another_referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// CREATE TABLE another_host_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES another_referenced_table(id));
    /// CREATE TABLE compatible_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// CREATE TABLE incompatible_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES another_referenced_table(id));
    /// CREATE TABLE non_fk_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE serial_table_one (id SERIAL PRIMARY KEY, name TEXT);
    /// CREATE TABLE serial_table_two (id SERIAL PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let id_column = host_table.column("id", &db).expect("Column 'id' should exist");
    /// let compatible_table = db.table(None, "compatible_table").unwrap();
    /// let serial_table_one = db.table(None, "serial_table_one").unwrap();
    /// let serial_id_column = serial_table_one.column("id", &db).expect("Column 'id' should exist");
    /// let serial_table_two = db.table(None, "serial_table_two").unwrap();
    /// let serial_id_column_two =
    ///     serial_table_two.column("id", &db).expect("Column 'id' should exist");
    /// let compatible_id_column =
    ///     compatible_table.column("id", &db).expect("Column 'id' should exist");
    /// let incompatible_table = db.table(None, "incompatible_table").unwrap();
    /// let incompatible_id_column =
    ///     incompatible_table.column("id", &db).expect("Column 'id' should exist");
    /// let another_host_table = db.table(None, "another_host_table").unwrap();
    /// let another_id_column = another_host_table.column("id", &db).expect("Column 'id' should exist");
    /// let non_fk_table = db.table(None, "non_fk_table").unwrap();
    /// let non_fk_id_column = non_fk_table.column("id", &db).expect("Column 'id' should exist");
    /// assert!(
    ///     id_column.is_compatible_with(&db, compatible_id_column),
    ///     "Columns should be compatible as they reference the same table"
    /// );
    /// assert!(
    ///     !id_column.is_compatible_with(&db, incompatible_id_column),
    ///     "Columns should not be compatible as they reference different tables"
    /// );
    /// assert!(
    ///     !id_column.is_compatible_with(&db, another_id_column),
    ///     "Columns should not be compatible as they reference different tables"
    /// );
    /// assert!(
    ///     !id_column.is_compatible_with(&db, non_fk_id_column),
    ///     "Columns should not be compatible as one of them is not a foreign key"
    /// );
    /// assert!(
    ///     !serial_id_column.is_compatible_with(&db, serial_id_column_two),
    ///     "Columns should not be compatible as both are generative"
    /// );
    /// assert!(
    ///     serial_id_column.is_compatible_with(&db, non_fk_id_column),
    ///     "Columns should be compatible as only one is generative and they have the same data type"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_compatible_with(&self, database: &Self::DB, other: &Self) -> bool {
        let host_table: &<Self::DB as DatabaseLike>::Table = ColumnLike::table(self, database);
        let other_table: &<Self::DB as DatabaseLike>::Table = ColumnLike::table(other, database);

        // If the two columns are the same, they are compatible.
        if host_table == other_table && self == other {
            return true;
        }

        // If both columns have generative data types, they are not compatible
        // as the two values should never be the same.
        if self.is_generated() && other.is_generated() {
            return false;
        }

        if self.normalized_data_type(database) != other.normalized_data_type(database) {
            return false;
        }

        let mut local_referenced_tables =
            host_table.referenced_tables_via_column(database, self.borrow());
        let mut other_referenced_tables =
            other_table.referenced_tables_via_column(database, other.borrow());

        if local_referenced_tables.is_empty() && other_referenced_tables.is_empty() {
            // If both columns are not foreign keys, they are compatible.
            return true;
        }

        // If the columns are primary keys, we include the local table as a
        // referenced table.
        if self.is_primary_key(database) {
            local_referenced_tables.push(host_table);
        }
        if other.is_primary_key(database) {
            other_referenced_tables.push(other_table);
        }

        // We determine the set of ancestors of the referenced tables.
        let local_referenced_ancestors = local_referenced_tables
            .iter()
            .flat_map(|table| table.ancestral_extended_tables(database))
            .collect::<Vec<_>>();
        let other_referenced_ancestors = other_referenced_tables
            .iter()
            .flat_map(|table| table.ancestral_extended_tables(database))
            .collect::<Vec<_>>();

        // We extend the referenced tables with their ancestors.
        local_referenced_tables.extend(local_referenced_ancestors);
        other_referenced_tables.extend(other_referenced_ancestors);

        local_referenced_tables.iter().any(|table| other_referenced_tables.contains(table))
    }

    /// Iterates over the
    /// [`CheckConstraintLike`](crate::traits::CheckConstraintLike)s
    /// that involve this column within the table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query check
    ///   constraints from.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     "CREATE TABLE my_table (id INT, age INT CHECK (age >= 0), score INT CHECK (score BETWEEN 0 AND 100));",
    /// )?;
    ///
    /// let table = db.table(None, "my_table").unwrap();
    /// let age_column = table.column("age", &db).expect("Column 'age' should exist");
    /// let score_column = table.column("score", &db).expect("Column 'score' should exist");
    ///
    /// let age_checks = age_column.check_constraints(&db).collect::<Vec<_>>();
    /// let score_checks = score_column.check_constraints(&db).collect::<Vec<_>>();
    ///
    /// assert_eq!(age_checks.len(), 1, "age column should have one check constraint");
    /// assert_eq!(score_checks.len(), 1, "score column should have one check constraint");
    /// # Ok(())
    /// # }
    /// ```
    fn check_constraints<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::CheckConstraint> + 'db {
        let table: &<Self::DB as DatabaseLike>::Table = ColumnLike::table(self, database);
        table
            .check_constraints(database)
            .filter(|check| check.involves_column(database, self.borrow()))
    }

    /// Returns whether the column has any check constraints.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query check
    ///   constraints from.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db =
    ///     ParserDB::try_from("CREATE TABLE my_table (id INT, age INT CHECK (age >= 0), score INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let age_column = table.column("age", &db).expect("Column 'age' should exist");
    /// let score_column = table.column("score", &db).expect("Column 'score' should exist");
    /// assert!(age_column.has_check_constraints(&db), "age column should have check constraints");
    /// assert!(
    ///     !score_column.has_check_constraints(&db),
    ///     "score column should not have check constraints"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_check_constraints(&self, database: &Self::DB) -> bool {
        self.check_constraints(database).next().is_some()
    }

    #[inline]
    /// Returns an iterator over the non-tautological
    /// [`CheckConstraintLike`](crate::traits::CheckConstraintLike)s
    /// that involve this column within the table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query check
    ///   constraints from.
    fn non_tautological_check_constraints<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::CheckConstraint> + 'db {
        self.check_constraints(database).filter(move |check| !check.is_tautology(database))
    }

    #[inline]
    /// Returns whether the column has non-tautological check constraints.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query check
    ///   constraints from.
    fn has_non_tautological_check_constraints(&self, database: &Self::DB) -> bool {
        self.non_tautological_check_constraints(database).next().is_some()
    }
}
