//! Submodule providing a trait for describing SQL Table-like entities.

use std::hash::Hash;

use crate::traits::{
    CheckConstraintLike, ColumnLike, DatabaseLike, ForeignKeyLike, UniqueIndexLike,
};

/// A trait for types that can be treated as SQL tables.
pub trait TableLike: Hash + Ord + Eq {
    /// The database type the table belongs to.
    type Database: DatabaseLike<Table = Self, Column = Self::Column, ForeignKey = Self::ForeignKey>;
    /// The column type of the table.
    type Column: ColumnLike;
    /// The check constraint type of the table.
    type CheckConstraint: CheckConstraintLike;
    /// The unique index type of the table.
    type UniqueIndex: UniqueIndexLike<Table = Self, Database = Self::Database>;
    /// The foreign key type of the table.
    type ForeignKey: ForeignKeyLike<Table = Self, Column = Self::Column, Database = Self::Database>;

    /// Returns the name of the table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql("CREATE TABLE mytable (id INT);")?;
    /// let table = db.table(None, "mytable");
    /// assert_eq!(table.table_name(), "mytable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_name(&self) -> &str;

    /// The schema name of the table, if it has one.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = SqlParserDatabase::from_sql(
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

    /// Iterates over the columns of the column using the provided schema.
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
    /// let db = SqlParserDatabase::from_sql("CREATE TABLE my_table (id INT, name TEXT);")?;
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
    /// let db = SqlParserDatabase::from_sql("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column_by_name("id", &db).expect("Column 'id' should exist");
    /// assert_eq!(id_column.column_name(), "id");
    /// let non_existent_column = table.column_by_name("non_existent", &db);
    /// assert!(non_existent_column.is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn column_by_name<'db>(
        &'db self,
        name: &str,
        database: &'db Self::Database,
    ) -> Option<&'db Self::Column>
    where
        Self: 'db,
    {
        self.columns(database).find(|col| col.column_name() == name)
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
    /// let db = SqlParserDatabase::from_sql(
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
    /// let db = SqlParserDatabase::from_sql(
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
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let id_column = table.column_by_name("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column_by_name("name", &db).expect("Column 'name' should exist");
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
    /// let db = SqlParserDatabase::from_sql(
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
    /// let db = SqlParserDatabase::from_sql(
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
    /// let db = SqlParserDatabase::from_sql(
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
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE my_table (id INT UNIQUE, name TEXT, UNIQUE (name));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table");
    /// let unique_indices: Vec<_> = table
    ///     .unique_indices(&db)
    ///     .map(|ui| ui.columns(&db, &table).map(|col| col.column_name()).collect::<Vec<_>>())
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
    /// let db = SqlParserDatabase::from_sql(
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

    /// Returns a vector with the (dedupliated) tables which are referenced by
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
    /// let db = SqlParserDatabase::from_sql(
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
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
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
        self.foreign_keys(database).filter(|fk| fk.is_extension_foreign_key(database, self))
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
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
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
    /// let db = SqlParserDatabase::from_sql(
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
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let id_column = host_table.column_by_name("id", &db).expect("Column 'id' should exist");
    /// let referenced_tables = host_table.referenced_tables_via_column(&db, id_column);
    /// assert_eq!(referenced_tables.len(), 1);
    /// let name_column = host_table.column_by_name("name", &db).expect("Column 'name' should exist");
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

        if self.is_primary_key_column(database, column) {
            referenced_tables.push(self);
        }

        for fk in self.foreign_keys(database) {
            if fk.host_columns(database, self).all(|col| col == column)
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

    /// Returns whether the table shares any ancestor with the given table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `other` - The other table to check against.
    ///
    /// # Errors
    ///
    /// * If the foreign key constraint cannot be loaded from the database.
    fn shares_ancestors_with(&self, database: &Self::Database, other: &Self) -> bool {
        let self_ancestors = self.ancestral_extended_tables(database);
        let other_ancestors = other.ancestral_extended_tables(database);

        self_ancestors.iter().any(|table| other_ancestors.contains(table))
    }

    /// Returns whether the column is compatible with another column.
    fn is_compatible_with(
        &self,
        database: &Self::Database,
        host_column: &Self::Column,
        other_table: &Self,
        other_column: &Self::Column,
    ) -> bool {
        debug_assert!(
            self.columns(database).any(|col| col == host_column),
            "Local column {:?} does not belong to table {:?}",
            host_column.column_name(),
            self.table_name()
        );
        debug_assert!(
            other_table.columns(database).any(|col| col == other_column),
            "Other column {:?} does not belong to table {:?}",
            other_column.column_name(),
            other_table.table_name()
        );

        if host_column.data_type() != other_column.data_type() {
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
}
