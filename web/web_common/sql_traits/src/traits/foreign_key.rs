//! Submodule definining the `ForeignKeyLike` trait for SQL referenced keys.

use sqlparser::ast::MatchKind;

use crate::traits::{ColumnLike, DatabaseLike, TableLike, UniqueIndexLike};

/// A foreign key constraint is a rule that specifies a relationship between
/// two tables. This trait represents such a foreign key constraint in a
/// database-agnostic way.
pub trait ForeignKeyLike: Eq {
    /// The column type associated with the foreign key.
    type Column: ColumnLike;
    /// The table type associated with the foreign key.
    type Table: TableLike<
            Column = Self::Column,
            Database = Self::Database,
            ForeignKey = Self,
            UniqueIndex = Self::UniqueIndex,
        >;
    /// Unique index type associated with the foreign key.
    type UniqueIndex: UniqueIndexLike<Database = Self::Database, Table = Self::Table, Column = Self::Column>;
    /// The database type associated with the foreign key.
    type Database: DatabaseLike<Table = Self::Table, Column = Self::Column>;

    /// Returns the name of the foreign key, if it has one.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     CONSTRAINT fk_host_ref FOREIGN KEY (id) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let foreign_keys: Vec<_> = host_table.foreign_keys(&db).collect();
    /// let named_fk = &foreign_keys[0];
    /// let unnamed_fk = &foreign_keys[1];
    /// assert_eq!(named_fk.foreign_key_name(), Some("fk_host_ref"));
    /// assert_eq!(unnamed_fk.foreign_key_name(), None);
    /// # Ok(())
    /// # }
    /// ```
    fn foreign_key_name(&self) -> Option<&str>;

    /// Returns whether the foreign key is on delete cascade.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id) ON DELETE CASCADE,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let foreign_keys: Vec<_> = host_table.foreign_keys(&db).collect();
    /// let cascade_fk = &foreign_keys[0];
    /// let normal_fk = &foreign_keys[1];
    /// assert!(cascade_fk.on_delete_cascade(&db));
    /// assert!(!normal_fk.on_delete_cascade(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn on_delete_cascade(&self, database: &Self::Database) -> bool;

    /// Returns the referenced table that the foreign key points to.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let referenced_table = foreign_key.referenced_table(&db);
    /// assert_eq!(referenced_table.table_name(), "referenced_table");
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table;

    /// Returns an iterator over the columns in the host table that are part of
    /// the foreign key.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_table` - A reference to the host table that contains the foreign
    ///   key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id1 INT, id2 INT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE host_table (
    ///     ref_id1 INT,
    ///     ref_id2 INT,
    ///     FOREIGN KEY (ref_id1, ref_id2) REFERENCES referenced_table(id1, id2)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let host_column_names: Vec<&str> =
    ///     foreign_key.host_columns(&db, &host_table).map(|col| col.column_name()).collect();
    /// assert_eq!(host_column_names, vec!["ref_id1", "ref_id2"]);
    /// # Ok(())
    /// # }
    /// ```
    fn host_columns<'db>(
        &'db self,
        database: &'db Self::Database,
        host_table: &'db Self::Table,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db;

    /// Returns whether the foreign key is composite (i.e., consists of more
    /// than one column).
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_table` - A reference to the host table that contains the foreign
    ///   key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id1 INT, id2 INT, name TEXT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE single_fk_table (
    ///     ref_id INT,
    ///     FOREIGN KEY (ref_id) REFERENCES referenced_table(id1)
    /// );
    /// CREATE TABLE composite_fk_table (
    ///     ref_id1 INT,
    ///     ref_id2 INT,
    ///     FOREIGN KEY (ref_id1, ref_id2) REFERENCES referenced_table(id1, id2)
    /// );
    /// "#,
    /// )?;
    /// let single_fk_table = db.table(None, "single_fk_table");
    /// let composite_fk_table = db.table(None, "composite_fk_table");
    /// let single_fk = single_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let composite_fk =
    ///     composite_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(!single_fk.is_composite(&db, &single_fk_table));
    /// assert!(composite_fk.is_composite(&db, &composite_fk_table));
    /// # Ok(())
    /// # }
    /// ```
    fn is_composite(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        self.host_columns(database, host_table).nth(1).is_some()
    }

    /// Returns the match kind of the foreign key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// use sqlparser::ast::MatchKind;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id) MATCH FULL
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert_eq!(foreign_key.match_kind(&db), MatchKind::Full);
    /// # Ok(())
    /// # }
    /// ```
    fn match_kind(&self, database: &Self::Database) -> MatchKind;

    /// Returns whether the foreign key is labelled with a `MATCH FULL` clause.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE full_match_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id) MATCH FULL
    /// );
    /// CREATE TABLE normal_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let full_match_table = db.table(None, "full_match_table");
    /// let normal_table = db.table(None, "normal_table");
    /// let full_match_fk =
    ///     full_match_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let normal_fk = normal_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(full_match_fk.match_full(&db));
    /// assert!(!normal_fk.match_full(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn match_full(&self, database: &Self::Database) -> bool {
        matches!(self.match_kind(database), MatchKind::Full)
    }

    /// Returns whether the foreign key can be potentially not enforced.
    ///
    /// # Implementation note
    ///
    /// A foreign key can be potentially not enforced if any of its columns is
    /// nullable, and it is not labelled with a `MATCH FULL` clause.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_table` - A reference to the host table that contains the foreign
    ///   key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE nullable_fk_table (
    ///     id INT NULL,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE not_null_fk_table (
    ///     id INT NOT NULL,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE nullable_match_full_table (
    ///     id INT NULL,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id) MATCH FULL
    /// );
    /// "#,
    /// )?;
    /// let nullable_fk_table = db.table(None, "nullable_fk_table");
    /// let not_null_fk_table = db.table(None, "not_null_fk_table");
    /// let nullable_match_full_table = db.table(None, "nullable_match_full_table");
    /// let nullable_fk =
    ///     nullable_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let not_null_fk =
    ///     not_null_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let nullable_match_full_fk =
    ///     nullable_match_full_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(
    ///     nullable_fk.is_nullable(&db, &nullable_fk_table),
    ///     "Nullable FK without MATCH FULL should be nullable"
    /// );
    /// assert!(
    ///     !not_null_fk.is_nullable(&db, &not_null_fk_table),
    ///     "NOT NULL FK should not be nullable"
    /// );
    /// assert!(
    ///     !nullable_match_full_fk.is_nullable(&db, &nullable_match_full_table),
    ///     "Nullable FK with MATCH FULL should not be nullable"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_nullable(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        self.host_columns(database, host_table)
            .any(|col: &Self::Column| ColumnLike::is_nullable(col))
            && !self.match_full(database)
    }

    /// Returns an iterator over the columns in the referenced table that are
    /// part of the foreign key.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id1 INT, id2 INT, name TEXT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE host_table (
    ///     ref_id1 INT,
    ///     ref_id2 INT,
    ///     FOREIGN KEY (ref_id1, ref_id2) REFERENCES referenced_table(id1, id2)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let referenced_column_names: Vec<&str> =
    ///     foreign_key.referenced_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(referenced_column_names, vec!["id1", "id2"]);
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db;

    /// Returns whether the foreign key is self-referential, i.e., the host
    /// table is the same as the referenced table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_table` - A reference to the host table that contains the foreign
    ///   key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE self_ref_table (
    ///     id INT PRIMARY KEY,
    ///     parent_id INT,
    ///     FOREIGN KEY (parent_id) REFERENCES self_ref_table(id)
    /// );
    /// CREATE TABLE other_table (id INT PRIMARY KEY);
    /// CREATE TABLE normal_ref_table (
    ///     id INT,
    ///     other_id INT,
    ///     FOREIGN KEY (other_id) REFERENCES other_table(id)
    /// );
    /// "#,
    /// )?;
    /// let self_ref_table = db.table(None, "self_ref_table");
    /// let normal_ref_table = db.table(None, "normal_ref_table");
    /// let self_ref_fk = self_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let normal_ref_fk =
    ///     normal_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(self_ref_fk.is_self_referential(&db, &self_ref_table));
    /// assert!(!normal_ref_fk.is_self_referential(&db, &normal_ref_table));
    /// # Ok(())
    /// # }
    /// ```
    fn is_self_referential(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        let referenced_table = self.referenced_table(database);
        host_table == referenced_table
    }

    /// Returns whether the foreign key references the primary key of the
    /// referenced table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE pk_ref_table (
    ///     ref_id INT,
    ///     FOREIGN KEY (ref_id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE non_pk_ref_table (
    ///     ref_name TEXT,
    ///     FOREIGN KEY (ref_name) REFERENCES referenced_table(name)
    /// );
    /// "#,
    /// )?;
    /// let pk_ref_table = db.table(None, "pk_ref_table");
    /// let non_pk_ref_table = db.table(None, "non_pk_ref_table");
    /// let pk_ref_fk = pk_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let non_pk_ref_fk =
    ///     non_pk_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(pk_ref_fk.is_referenced_primary_key(&db));
    /// assert!(!non_pk_ref_fk.is_referenced_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn is_referenced_primary_key(&self, database: &Self::Database) -> bool {
        let referenced_table = self.referenced_table(database);
        let mut pk_columns = referenced_table.primary_key_columns(database);
        let mut fk_columns = self.referenced_columns(database);

        while let (Some(fk_col), Some(pk_col)) = (fk_columns.next(), pk_columns.next()) {
            if fk_col != pk_col {
                return false;
            }
        }

        // We check that there are no remaining columns in either iterator.
        fk_columns.next().is_none() && pk_columns.next().is_none()
    }

    /// Returns the unique index in the referenced table that the foreign key
    /// references, if any, as the foreign key might reference the primary key
    /// of the referenced table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE referenced_table_with_unique (id INT PRIMARY KEY, name TEXT, UNIQUE(name));
    /// CREATE TABLE pk_ref_table (
    ///     ref_id INT,
    ///     FOREIGN KEY (ref_id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE non_pk_ref_table (
    ///     ref_name TEXT,
    ///     FOREIGN KEY (ref_name) REFERENCES referenced_table_with_unique(name)
    /// );
    /// "#,
    /// )?;
    /// let pk_ref_table = db.table(None, "pk_ref_table");
    /// let non_pk_ref_table = db.table(None, "non_pk_ref_table");
    /// let pk_ref_fk = pk_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let non_pk_ref_fk =
    ///     non_pk_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(pk_ref_fk.is_referenced_primary_key(&db));
    /// assert!(!non_pk_ref_fk.is_referenced_primary_key(&db));
    /// assert!(non_pk_ref_fk.is_referenced_unique_key(&db).is_some());
    /// assert!(pk_ref_fk.is_referenced_unique_key(&db).is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn is_referenced_unique_key<'db>(
        &self,
        database: &'db Self::Database,
    ) -> Option<&'db Self::UniqueIndex>
    where
        Self: 'db,
    {
        let referenced_table = self.referenced_table(database);
        let fk_columns: Vec<_> = self.referenced_columns(database).collect();
        referenced_table.unique_indices(database).find(|index| {
            let index_columns: Vec<_> = index.columns(database, referenced_table).collect();
            index_columns.len() == fk_columns.len()
                && index_columns.iter().all(|col| fk_columns.contains(col))
        })
    }

    /// Returns whether the foreign key locally matches the primary key of the
    /// host table.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_table` - A reference to the host table that contains the foreign
    ///   key.
    fn is_host_primary_key(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        let mut pk_columns = host_table.primary_key_columns(database);
        let mut fk_columns = self.host_columns(database, host_table);
        while let (Some(fk_col), Some(pk_col)) = (fk_columns.next(), pk_columns.next()) {
            if fk_col != pk_col {
                return false;
            }
        }
        // We check that there are no remaining columns in either iterator.
        fk_columns.next().is_none() && pk_columns.next().is_none()
    }

    /// Returns whether the foreign key includes (but does not necessarily
    /// match) all the primary key columns of the host table.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_table` - A reference to the host table that contains the foreign
    ///   key.
    fn includes_host_primary_key(
        &self,
        database: &Self::Database,
        host_table: &Self::Table,
    ) -> bool {
        let pk_columns: Vec<_> = host_table.primary_key_columns(database).collect();
        let fk_columns: Vec<_> = self.host_columns(database, host_table).collect();
        pk_columns.iter().all(|pk| fk_columns.contains(pk))
    }

    /// Returns whether the foreign key includes (but does not necessarily
    /// match) all the primary key columns of the referenced table.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    fn includes_referenced_primary_key(&self, database: &Self::Database) -> bool {
        let referenced_table = self.referenced_table(database);
        let pk_columns: Vec<_> = referenced_table.primary_key_columns(database).collect();
        let fk_columns: Vec<_> = self.referenced_columns(database).collect();
        pk_columns.iter().all(|pk| fk_columns.contains(pk))
    }

    /// Returns whether the foreign key is an "extension" foreign key, i.e., it
    /// references the primary key of another table, and the host table is not
    /// self-referential.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_table` - A reference to the host table that contains the foreign
    ///   key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE extension_table (
    ///     id INT PRIMARY KEY,
    ///     extra_field TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id)
    /// );
    /// CREATE TABLE reference_table (
    ///     id INT PRIMARY KEY,
    ///     parent_id INT,
    ///     FOREIGN KEY (parent_id) REFERENCES parent_table(id)
    /// );
    /// CREATE TABLE self_ref_table (
    ///     id INT PRIMARY KEY,
    ///     FOREIGN KEY (id) REFERENCES self_ref_table(id)
    /// );
    /// "#,
    /// )?;
    /// let extension_table = db.table(None, "extension_table");
    /// let reference_table = db.table(None, "reference_table");
    /// let self_ref_table = db.table(None, "self_ref_table");
    /// let extension_fk = extension_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let reference_fk = reference_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let self_ref_fk = self_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(extension_fk.is_extension_foreign_key(&db, &extension_table), "Should be extension FK");
    /// assert!(
    ///     !reference_fk.is_extension_foreign_key(&db, &reference_table),
    ///     "parent_id is not the primary key"
    /// );
    /// assert!(
    ///     !self_ref_fk.is_extension_foreign_key(&db, &self_ref_table),
    ///     "Self-referential FK should not be extension FK"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_extension_foreign_key(
        &self,
        database: &Self::Database,
        host_table: &Self::Table,
    ) -> bool {
        self.is_host_primary_key(database, host_table)
            && self.is_referenced_primary_key(database)
            && !self.is_self_referential(database, host_table)
    }

    /// Returns whether the key is a singleton foreign key, i.e. it is the only
    /// foreign key to refer to a particular foreign table within the context
    /// of its table of definition.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_table` - A reference to the host table that contains the foreign
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (
    ///     id1 INT,
    ///     id2 INT,
    ///     FOREIGN KEY (id1) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id2) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE singleton_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table");
    /// let singleton_table = db.table(None, "singleton_table");
    /// let fks: Vec<_> = host_table.foreign_keys(&db).collect();
    /// let fk1 = &fks[0];
    /// let fk2 = &fks[1];
    /// let singleton_fk = singleton_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(!fk1.is_singleton(&db, &host_table), "Not a singleton FK");
    /// assert!(!fk2.is_singleton(&db, &host_table), "Not a singleton FK");
    /// assert!(singleton_fk.is_singleton(&db, &singleton_table), "Should be a singleton FK");
    /// # Ok(())
    /// # }
    /// ```
    fn is_singleton(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        let foreign_table = self.referenced_table(database);
        host_table
            .foreign_keys(database)
            .all(|fk| fk == self || fk.referenced_table(database) != foreign_table)
    }
}
