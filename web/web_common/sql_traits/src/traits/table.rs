//! Submodule providing a trait for describing SQL Table-like entities.

use std::hash::Hash;

use crate::traits::{
    CheckConstraintLike, ColumnLike, DatabaseLike, ForeignKeyLike, Metadata, UniqueIndexLike,
};

/// A trait for types that can be treated as SQL tables.
pub trait TableLike: Hash + Ord + Eq + Metadata {
    /// The database type the table belongs to.
    type Database: DatabaseLike<Table = Self, Column = Self::Column, ForeignKey = Self::ForeignKey>;
    /// The column type of the table.
    type Column: ColumnLike;
    /// The check constraint type of the table.
    type CheckConstraint: CheckConstraintLike;
    /// The unique index type of the table.
    type UniqueIndex: UniqueIndexLike<Table = Self, Database = Self::Database, Column = Self::Column>;
    /// The foreign key type of the table.
    type ForeignKey: ForeignKeyLike<
            Table = Self,
            Column = Self::Column,
            Database = Self::Database,
            UniqueIndex = Self::UniqueIndex,
        >;

    /// Returns the name of the table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE mytable (id INT);")?;
    /// let table = db.table(None, "mytable");
    /// assert_eq!(table.table_name(), "mytable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_name(&self) -> &str;

    /// Returns the documentation of the table, if any.
    fn table_doc<'db>(&'db self, database: &'db Self::Database) -> Option<&'db str>
    where
        Self: 'db;

    /// The schema name of the table, if it has one.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE my_schema.my_table_with_schema (id INT);
    /// CREATE TABLE my_table (id INT);"#,
    /// )?;
    /// let table_no_schema = db.table(None, "my_table");
    /// assert_eq!(table_no_schema.table_schema(), None);
    /// let table_with_schema = db.table(Some("my_schema"), "my_table_with_schema");
    /// assert_eq!(table_with_schema.table_schema(), Some("my_schema"));
    /// # Ok(())
    /// # }
    /// ```
    fn table_schema(&self) -> Option<&str>;

    /// Iterates over the columns of the table using the provided schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table");
    /// let column_names: Vec<&str> = table.columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(column_names, vec!["id", "name"]);
    /// # Ok(())
    /// # }
    /// ```
    fn columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db;

    /// Returns the corresponding column by name, if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column to retrieve.
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// assert_eq!(id_column.column_name(), "id");
    /// let non_existent_column = table.column("non_existent", &db);
    /// assert!(non_existent_column.is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn column<'db>(
        &'db self,
        name: &str,
        database: &'db Self::Database,
    ) -> Option<&'db Self::Column>
    where
        Self: 'db,
    {
        TableLike::columns(self, database).find(|col| col.column_name() == name)
    }

    /// Returns whether the provided column belongs to this table.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to check.
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE table1 (id INT, name TEXT);
    /// CREATE TABLE table2 (id INT, description TEXT);
    /// "#,
    /// )?;
    /// let table1 = db.table(None, "table1");
    /// let table2 = db.table(None, "table2");
    /// let table1_id = table1.column("id", &db).expect("Column 'id' should exist in table1");
    /// let table2_id = table2.column("id", &db).expect("Column 'id' should exist in table2");
    /// assert!(table1.has_column(table1_id, &db));
    /// assert!(!table1.has_column(table2_id, &db));
    /// assert!(table2.has_column(table2_id, &db));
    /// assert!(!table2.has_column(table1_id, &db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_column(&self, column: &Self::Column, database: &Self::Database) -> bool {
        TableLike::columns(self, database).any(|col| col == column)
    }

    /// Iterates over the primary key columns of the table using the provided
    /// schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_composite_pk_table (id1 INT, id2 INT, name TEXT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE my_no_pk_table (id INT, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let pk_columns: Vec<&str> =
    ///     table.primary_key_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(pk_columns, vec!["id"]);
    /// let composite_pk_table = db.table(None, "my_composite_pk_table");
    /// let composite_pk_columns: Vec<&str> =
    ///     composite_pk_table.primary_key_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(composite_pk_columns, vec!["id1", "id2"]);
    /// let no_pk_table = db.table(None, "my_no_pk_table");
    /// let no_pk_columns: Vec<&str> =
    ///     no_pk_table.primary_key_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(no_pk_columns, Vec::<&str>::new());
    /// # Ok(())
    /// # }
    /// ```
    fn primary_key_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db;

    /// Returns the primary key column of the table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Panics
    ///
    /// * If the table does not have exactly one primary key column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let pk_column = table.primary_key_column(&db);
    /// assert_eq!(pk_column.column_name(), "id");
    /// # Ok(())
    /// # }
    /// ```
    fn primary_key_column<'db>(&'db self, database: &'db Self::Database) -> &'db Self::Column {
        let mut pk_columns = self.primary_key_columns(database);
        let pk_column = pk_columns.next().expect("Table has no primary key column");
        if pk_columns.next().is_some() {
            panic!("Table has a composite primary key");
        }
        pk_column
    }

    /// Returns whether the primary key of the table is generated (i.e.,
    /// auto-incrementing).
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    /// belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id SERIAL PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_no_gen_pk_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// assert!(table.has_generated_primary_key(&db));
    /// let no_gen_pk_table = db.table(None, "my_no_gen_pk_table");
    /// assert!(!no_gen_pk_table.has_generated_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_generated_primary_key(&self, database: &Self::Database) -> bool {
        self.primary_key_columns(database).all(|col| col.is_generated())
            && self.has_primary_key(database)
    }

    /// Returns a vector with the normalized data types of the primary key
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    /// belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id SERIAL PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_composite_pk_table (id1 INT, id2 BIGSERIAL, name TEXT, PRIMARY KEY (id1, id2));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let pk_types = table.primary_key_type(&db);
    /// assert_eq!(pk_types, vec!["INT"]);
    /// let composite_pk_table = db.table(None, "my_composite_pk_table");
    /// let composite_pk_types = composite_pk_table.primary_key_type(&db);
    /// assert_eq!(composite_pk_types, vec!["INT", "BIGINT"]);
    /// # Ok(())
    /// # }
    /// ```
    fn primary_key_type(&self, database: &Self::Database) -> Vec<String> {
        self.primary_key_columns(database).map(|col| col.normalized_data_type()).collect()
    }

    /// Returns whether the provided column is the primary key column of the
    /// table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `column` - A reference to the column to check.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// assert!(table.is_primary_key_column(&db, id_column));
    /// assert!(!table.is_primary_key_column(&db, name_column));
    /// # Ok(())
    /// # }
    /// ```
    fn is_primary_key_column(&self, database: &Self::Database, column: &Self::Column) -> bool {
        self.primary_key_columns(database).all(|col| col == column)
            && self.has_primary_key(database)
    }

    /// Returns whether the table has a primary key.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_no_pk_table (id INT, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// assert!(table.has_primary_key(&db));
    /// let no_pk_table = db.table(None, "my_no_pk_table");
    /// assert!(!no_pk_table.has_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_primary_key(&self, database: &Self::Database) -> bool {
        self.primary_key_columns(database).next().is_some()
    }

    /// Returns whether the table has a composite primary key.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_composite_pk_table (id1 INT, id2 INT, name TEXT, PRIMARY KEY (id1, id2));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// assert!(!table.has_composite_primary_key(&db));
    /// let composite_pk_table = db.table(None, "my_composite_pk_table");
    /// assert!(composite_pk_table.has_composite_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_composite_primary_key(&self, database: &Self::Database) -> bool {
        self.primary_key_columns(database).nth(1).is_some()
    }

    /// Iterates over the check constraints of the table using the provided
    /// schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT CHECK (id > 0), name TEXT, CHECK (length(name) > 0));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let check_constraints: Vec<_> =
    ///     table.check_constraints(&db).map(|cc| cc.expression().to_string()).collect();
    /// assert_eq!(check_constraints, vec!["id > 0", "length(name) > 0"]);
    /// # Ok(())
    /// # }
    /// ```
    fn check_constraints<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::CheckConstraint>
    where
        Self: 'db;

    /// Iterates over the unique indexes of the table using the provided
    /// schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT UNIQUE, name TEXT, UNIQUE (name));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let unique_indices: Vec<_> = table
    ///     .unique_indices(&db)
    ///     .map(|ui| ui.columns(&db).map(|col| col.column_name()).collect::<Vec<_>>())
    ///     .collect();
    /// assert_eq!(unique_indices, vec![vec!["id"], vec!["name"]]);
    /// # Ok(())
    /// # }
    /// ```
    fn unique_indices<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::UniqueIndex>
    where
        Self: 'db;

    /// Iterates over the foreign keys of the table using the provided schema.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    /// ```
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT, name TEXT, FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let foreign_keys = host_table.foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(foreign_keys.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db;

    /// Returns whether the table has any foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table_with_fk (id INT, name TEXT, FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// CREATE TABLE host_table_without_fk (id INT, name TEXT);
    /// "#,
    /// )?;
    /// let referenced_table = db.table(None, "referenced_table");
    /// assert!(!referenced_table.has_foreign_keys(&db));
    /// let host_table_with_fk = db.table(None, "host_table_with_fk");
    /// assert!(host_table_with_fk.has_foreign_keys(&db));
    /// let host_table_without_fk = db.table(None, "host_table_without_fk");
    /// assert!(!host_table_without_fk.has_foreign_keys(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_foreign_keys(&self, database: &Self::Database) -> bool {
        self.foreign_keys(database).next().is_some()
    }

    /// Returns whether the table has non-self-referential foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY);
    /// CREATE TABLE child_table (id INT PRIMARY KEY, parent_id INT REFERENCES parent_table(id));
    /// CREATE TABLE self_ref_table (id INT PRIMARY KEY, parent_id INT REFERENCES self_ref_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table");
    /// assert!(child_table.has_non_self_referential_foreign_keys(&db));
    /// let self_ref_table = db.table(None, "self_ref_table");
    /// assert!(self_ref_table.has_non_self_referential_foreign_keys(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_non_self_referential_foreign_keys(&self, database: &Self::Database) -> bool {
        self.foreign_keys(database)
            .filter(move |fk| !fk.is_self_referential(database))
            .next()
            .is_some()
    }

    /// Iterates over the foreign keys in the current table which refer to
    /// ancestors of the provided table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    /// belongs.
    /// * `table` - A reference to the table whose ancestors are to be
    ///   considered.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY REFERENCES parent_table(id));
    /// CREATE TABLE other_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT PRIMARY KEY,
    ///     parent_id INT REFERENCES parent_table(id),
    ///     grandparent_id INT REFERENCES grandparent_table(id),
    ///     other_id INT REFERENCES other_table(id)
    /// );
    /// "#,
    /// )?;
    ///
    /// let host_table = db.table(None, "host_table");
    /// let child_table = db.table(None, "child_table");
    /// let fks_to_ancestors = host_table.foreign_keys_to_ancestors_of(&db, child_table);
    /// assert_eq!(fks_to_ancestors.count(), 2);
    /// # Ok(())
    /// # }
    /// ```
    fn foreign_keys_to_ancestors_of<'db>(
        &'db self,
        database: &'db Self::Database,
        table: &'db Self,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db,
    {
        let ancestors = table.ancestral_extended_tables(database);
        self.foreign_keys(database).filter(move |fk| {
            let referenced_table = fk.referenced_table(database);
            ancestors.iter().any(|ancestor| ancestor == &referenced_table)
                && fk.is_referenced_primary_key(database)
        })
    }

    /// Returns a vector with the (deduplicated) tables which are referenced by
    /// the current table via foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table1 (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE referenced_table2 (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table1(id),
    ///     FOREIGN KEY (id) REFERENCES referenced_table2(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let referenced_tables = host_table.referenced_tables(&db);
    /// assert_eq!(referenced_tables.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_tables<'db>(&'db self, database: &'db Self::Database) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let mut referenced_tables = Vec::new();

        for foreign_key in self.foreign_keys(database) {
            let referenced_table = foreign_key.referenced_table(database);
            referenced_tables.push(referenced_table);
        }

        referenced_tables.sort_unstable();
        referenced_tables.dedup();

        referenced_tables
    }

    /// Returns a vector with the (deduplicated) tables which are referenced by
    /// the current table via foreign keys, and which are not the table itself.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    fn non_self_referenced_tables<'db>(&'db self, database: &'db Self::Database) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let mut referenced_tables = self.referenced_tables(database);
        referenced_tables.retain(|&table| table != self);
        referenced_tables
    }

    /// Returns the foreign keys which are used to define extensions.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT PRIMARY KEY REFERENCES referenced_table(id), name TEXT);
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let extension_fks = host_table.extension_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(extension_fks.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn extension_foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(|fk| fk.is_extension_foreign_key(database))
    }

    /// Returns the tables which are extended by the current table via foreign
    /// keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE extended_table (id INT PRIMARY KEY);
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT PRIMARY KEY,
    ///     other_id INT,
    ///     FOREIGN KEY (other_id) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id) REFERENCES extended_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let extended_tables = host_table.extended_tables(&db);
    /// assert_eq!(extended_tables.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn extended_tables<'db>(&'db self, database: &'db Self::Database) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        self.extension_foreign_keys(database).map(|fk| fk.referenced_table(database)).collect()
    }

    /// Returns the unique tables which are extended by either the current
    /// table or any of the tables it extends via foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table");
    /// let ancestral_tables = child_table.ancestral_extended_tables(&db);
    /// assert_eq!(ancestral_tables.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    fn ancestral_extended_tables<'db>(&'db self, database: &'db Self::Database) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let extension_tables = self.extended_tables(database);
        let mut ancestral_tables = extension_tables.clone();

        for table in &extension_tables {
            let mut parent_ancestral_tables = table.ancestral_extended_tables(database);
            ancestral_tables.append(&mut parent_ancestral_tables);
        }

        ancestral_tables.sort_unstable();
        ancestral_tables.dedup();

        ancestral_tables
    }

    /// Returns the tables referenced in foreign keys of the current table via
    /// the provided column.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `column` - A reference to the column in the current table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let id_column = host_table.column("id", &db).expect("Column 'id' should exist");
    /// let referenced_tables = host_table.referenced_tables_via_column(&db, id_column);
    /// assert_eq!(referenced_tables.len(), 1);
    /// let name_column = host_table.column("name", &db).expect("Column 'name' should exist");
    /// let no_referenced_tables = host_table.referenced_tables_via_column(&db, name_column);
    /// assert_eq!(no_referenced_tables.len(), 0);
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_tables_via_column<'db>(
        &'db self,
        database: &'db Self::Database,
        column: &Self::Column,
    ) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let mut referenced_tables = Vec::new();

        for fk in self.foreign_keys(database) {
            if fk.host_columns(database).all(|col| col == column)
                && fk.is_referenced_primary_key(database)
            {
                let referenced_table = fk.referenced_table(database);
                referenced_tables.push(referenced_table);
            }
        }

        referenced_tables.sort_unstable();
        referenced_tables.dedup();

        referenced_tables
    }

    /// Returns whether the table is a descendant of another table, i.e., if it
    /// extends the other table either directly or some other table which
    /// extends the other table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `other` - The other table to check against.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table");
    /// let parent_table = db.table(None, "parent_table");
    /// assert!(child_table.is_descendant_of(&db, parent_table));
    /// assert!(!parent_table.is_descendant_of(&db, child_table));
    /// # Ok(())
    /// # }
    /// ```
    fn is_descendant_of(&self, database: &Self::Database, other: &Self) -> bool {
        self.ancestral_extended_tables(database).contains(&other)
    }

    /// Returns whether the table shares any ancestor with the given table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `other` - The other table to check against.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// CREATE TABLE unrelated_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table");
    /// let parent_table = db.table(None, "parent_table");
    /// let grandparent_table = db.table(None, "grandparent_table");
    /// let unrelated_table = db.table(None, "unrelated_table");
    /// assert!(
    ///     child_table.shares_ancestors_with(&db, parent_table),
    ///     "Child should share ancestors with parent"
    /// );
    /// assert!(
    ///     child_table.shares_ancestors_with(&db, grandparent_table),
    ///     "Child should share ancestors with grandparent"
    /// );
    /// assert!(
    ///     !child_table.shares_ancestors_with(&db, unrelated_table),
    ///     "Child should not share ancestors with unrelated"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn shares_ancestors_with(&self, database: &Self::Database, other: &Self) -> bool {
        let self_ancestors = self.ancestral_extended_tables(database);
        let other_ancestors = other.ancestral_extended_tables(database);

        self_ancestors.iter().any(|table| other_ancestors.contains(table))
            || self == other
            || self_ancestors.contains(&other)
            || other_ancestors.contains(&self)
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
    /// * `host_column` - The column in the current table to check.
    /// * `other_table` - The other table to check against.
    /// * `other_column` - The column in the other table to check.
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
    /// CREATE TABLE host_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// CREATE TABLE another_host_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES another_referenced_table(id));
    /// CREATE TABLE compatible_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// CREATE TABLE incompatible_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES another_referenced_table(id));
    /// CREATE TABLE non_fk_table (id INT, name TEXT);
    /// CREATE TABLE serial_table_one (id SERIAL PRIMARY KEY, name TEXT);
    /// CREATE TABLE serial_table_two (id SERIAL PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let id_column = host_table.column("id", &db).expect("Column 'id' should exist");
    /// let compatible_table = db.table(None, "compatible_table");
    /// let serial_table_one = db.table(None, "serial_table_one");
    /// let serial_id_column = serial_table_one.column("id", &db).expect("Column 'id' should exist");
    /// let serial_table_two = db.table(None, "serial_table_two");
    /// let serial_id_column_two =
    ///     serial_table_two.column("id", &db).expect("Column 'id' should exist");
    /// let compatible_id_column =
    ///     compatible_table.column("id", &db).expect("Column 'id' should exist");
    /// let incompatible_table = db.table(None, "incompatible_table");
    /// let incompatible_id_column =
    ///     incompatible_table.column("id", &db).expect("Column 'id' should exist");
    /// let another_host_table = db.table(None, "another_host_table");
    /// let another_id_column = another_host_table.column("id", &db).expect("Column 'id' should exist");
    /// let non_fk_table = db.table(None, "non_fk_table");
    /// let non_fk_id_column = non_fk_table.column("id", &db).expect("Column 'id' should exist");
    /// assert!(
    ///     host_table.is_compatible_with(&db, id_column, compatible_table, compatible_id_column),
    ///     "Columns should be compatible as they reference the same table"
    /// );
    /// assert!(
    ///     !host_table.is_compatible_with(&db, id_column, incompatible_table, incompatible_id_column),
    ///     "Columns should not be compatible as they reference different tables"
    /// );
    /// assert!(
    ///     !host_table.is_compatible_with(&db, id_column, another_host_table, another_id_column),
    ///     "Columns should not be compatible as they reference different tables"
    /// );
    /// assert!(
    ///     !host_table.is_compatible_with(&db, id_column, non_fk_table, non_fk_id_column),
    ///     "Columns should not be compatible as one of them is not a foreign key"
    /// );
    /// assert!(
    ///     !serial_table_one.is_compatible_with(
    ///         &db,
    ///         serial_id_column,
    ///         serial_table_two,
    ///         serial_id_column_two
    ///     ),
    ///     "Columns should not be compatible as both are generative"
    /// );
    /// assert!(
    ///     serial_table_one.is_compatible_with(&db, serial_id_column, non_fk_table, non_fk_id_column),
    ///     "Columns should be compatible as only one is generative and they have the same data type"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_compatible_with(
        &self,
        database: &Self::Database,
        host_column: &Self::Column,
        other_table: &Self,
        other_column: &Self::Column,
    ) -> bool {
        debug_assert!(
            TableLike::columns(self, database).any(|col| col == host_column),
            "Local column {:?} does not belong to table {:?}",
            host_column.column_name(),
            self.table_name()
        );
        debug_assert!(
            TableLike::columns(other_table, database).any(|col| col == other_column),
            "Other column {:?} does not belong to table {:?}",
            other_column.column_name(),
            other_table.table_name()
        );

        // If the two columns are the same, they are compatible.
        if self == other_table && host_column == other_column {
            return true;
        }

        // If both columns have generative data types, they are not compatible
        // as the two values should never be the same.
        if host_column.is_generated() && other_column.is_generated() {
            return false;
        }

        if host_column.normalized_data_type() != other_column.normalized_data_type() {
            return false;
        }

        let mut local_referenced_tables = self.referenced_tables_via_column(database, host_column);
        let mut other_referenced_tables =
            other_table.referenced_tables_via_column(database, other_column);

        if local_referenced_tables.is_empty() && other_referenced_tables.is_empty() {
            // If both columns are not foreign keys, they are compatible.
            return true;
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

    /// Returns the table singleton foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT UNIQUE, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let singleton_fks = host_table.singleton_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(singleton_fks.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn singleton_foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(|fk| fk.is_singleton(database))
    }

    /// Returns the table singleton foreign keys which are not self-referential.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (
    ///     id INT PRIMARY KEY,
    ///     name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id) REFERENCES host_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let non_self_referential_singleton_fks =
    ///     host_table.non_self_referential_singleton_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(non_self_referential_singleton_fks.len(), 1);
    /// assert!(non_self_referential_singleton_fks[0].is_singleton(&db));
    /// assert!(!non_self_referential_singleton_fks[0].is_self_referential(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn non_self_referential_singleton_foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db,
    {
        self.singleton_foreign_keys(database).filter(move |fk| !fk.is_self_referential(database))
    }

    /// Returns whether the table has singleton foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///  belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT UNIQUE, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// assert!(host_table.has_singleton_foreign_keys(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_singleton_foreign_keys(&self, database: &Self::Database) -> bool {
        self.singleton_foreign_keys(database).next().is_some()
    }

    /// Returns whether the table has non-self-referential singleton foreign
    /// keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///  belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (
    ///     id INT PRIMARY KEY,
    ///     name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id) REFERENCES host_table(id)
    /// );
    /// "#,
    /// )?;
    /// let referenced_table = db.table(None, "referenced_table");
    /// assert!(!referenced_table.has_non_self_referential_singleton_foreign_keys(&db));
    /// let host_table = db.table(None, "host_table");
    /// assert!(host_table.has_non_self_referential_singleton_foreign_keys(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_non_self_referential_singleton_foreign_keys(&self, database: &Self::Database) -> bool {
        self.non_self_referential_singleton_foreign_keys(database).next().is_some()
    }

    /// Returns whether the table depends directly or indirectly on another
    /// table via foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   and the other table belong.
    /// * `other` - The other table to check against.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table");
    /// let parent_table = db.table(None, "parent_table");
    /// let grandparent_table = db.table(None, "grandparent_table");
    /// assert!(child_table.depends_on(&db, parent_table));
    /// assert!(child_table.depends_on(&db, grandparent_table));
    /// assert!(!parent_table.depends_on(&db, child_table));
    /// assert!(!grandparent_table.depends_on(&db, child_table));
    /// assert!(parent_table.depends_on(&db, grandparent_table));
    /// assert!(!grandparent_table.depends_on(&db, parent_table));
    /// assert!(child_table.depends_on(&db, child_table));
    /// assert!(parent_table.depends_on(&db, parent_table));
    /// assert!(grandparent_table.depends_on(&db, grandparent_table));
    /// # Ok(())
    /// # }
    /// ```
    fn depends_on(&self, database: &Self::Database, other: &Self) -> bool {
        if self == other {
            return true;
        }
        self.foreign_keys(database).any(|fk| {
            let referenced_table = fk.referenced_table(database);
            referenced_table == other || referenced_table.depends_on(database, other)
        })
    }
}
