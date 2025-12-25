//! Submodule definining the `ForeignKeyLike` trait for SQL referenced keys.

use std::{borrow::Borrow, fmt::Debug};

use sqlparser::ast::ConstraintReferenceMatchKind;

use crate::traits::{ColumnLike, DatabaseLike, Metadata, TableLike, UniqueIndexLike};

/// A foreign key constraint is a rule that specifies a relationship between
/// two tables. This trait represents such a foreign key constraint in a
/// database-agnostic way.
pub trait ForeignKeyLike:
    Debug
    + Clone
    + Eq
    + Metadata
    + Ord
    + Borrow<<<Self as ForeignKeyLike>::DB as DatabaseLike>::ForeignKey>
{
    /// The database type associated with the foreign key.
    type DB: DatabaseLike<ForeignKey: Borrow<Self>>;

    /// Returns the name of the foreign key, if it has one.
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
    ///     CONSTRAINT fk_host_ref FOREIGN KEY (id) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id) ON DELETE CASCADE,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_keys: Vec<_> = host_table.foreign_keys(&db).collect();
    /// let cascade_fk = &foreign_keys[0];
    /// let normal_fk = &foreign_keys[1];
    /// assert!(cascade_fk.on_delete_cascade(&db));
    /// assert!(!normal_fk.on_delete_cascade(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn on_delete_cascade(&self, database: &Self::DB) -> bool;

    /// Returns the host table that contains the foreign key.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let fk_host_table = foreign_key.host_table(&db);
    /// assert_eq!(fk_host_table, host_table);
    /// # Ok(())
    /// # }
    /// ```
    fn host_table<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> &'db <Self::DB as DatabaseLike>::Table
    where
        Self: 'db;

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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let referenced_table = foreign_key.referenced_table(&db);
    /// assert_eq!(referenced_table.table_name(), "referenced_table");
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_table<'db>(
        &self,
        database: &'db Self::DB,
    ) -> &'db <Self::DB as DatabaseLike>::Table;

    /// Returns an iterator over the columns in the host table that are part of
    /// the foreign key.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id1 INT, id2 INT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE host_table (
    ///     ref_id1 INT,
    ///     ref_id2 INT,
    ///     FOREIGN KEY (ref_id1, ref_id2) REFERENCES referenced_table(id1, id2)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let host_column_names: Vec<&str> =
    ///     foreign_key.host_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(host_column_names, vec!["ref_id1", "ref_id2"]);
    /// # Ok(())
    /// # }
    /// ```
    fn host_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db;

    /// Returns the host column associated with the foreign key if it consists
    /// of a single column; returns `None` if the foreign key is composite.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///   id INT,
    /// FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE composite_fk_table (
    /// ref_id1 INT,
    /// ref_id2 INT,
    /// PRIMARY KEY (ref_id1, ref_id2),
    /// FOREIGN KEY (ref_id1, ref_id2) REFERENCES composite_fk_table(ref_id1, ref_id2)
    /// );
    /// "#,
    /// )?;
    ///
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let host_column = foreign_key.host_column(&db).expect("Should have a single host column");
    /// assert_eq!(host_column.column_name(), "id");
    /// let composite_fk_table = db.table(None, "composite_fk_table").unwrap();
    /// let composite_foreign_key =
    ///     composite_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(
    ///     composite_foreign_key.host_column(&db).is_none(),
    ///     "Composite foreign key should not have a single host column"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn host_column<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> Option<&'db <Self::DB as DatabaseLike>::Column> {
        let mut host_columns = self.host_columns(database);
        let first_column = host_columns.next()?;
        if host_columns.next().is_none() { Some(first_column) } else { None }
    }

    /// Returns whether the current foreign key is the only key in the
    /// host table which employs the same set of host columns.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY);
    /// CREATE TABLE parent_table (id INT, FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE host_table (
    ///   id INT,
    ///   FOREIGN KEY (id) REFERENCES parent_table(id),
    ///   FOREIGN KEY (id) REFERENCES grandparent_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let host_columns = host_table.foreign_keys(&db).collect::<Vec<_>>();
    /// let [first_fk, second_fk] = host_columns.as_slice() else {
    ///     panic!("Expected two foreign keys");
    /// };
    /// assert!(first_fk.shares_host_tables(&db), "First foreign key should share host columns");
    /// assert!(second_fk.shares_host_tables(&db), "Second foreign key should share host columns");
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let parent_fk = parent_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(!parent_fk.shares_host_tables(&db), "Parent foreign key should not share host columns");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn shares_host_tables(&self, database: &Self::DB) -> bool {
        let mut host_columns: Vec<_> = self.host_columns(database).collect();
        host_columns.sort_unstable();
        let host_table = self.host_table(database);
        for fk in host_table.foreign_keys(database) {
            let fk: &Self = fk.borrow();
            if fk == self {
                continue;
            }
            let mut fk_host_columns: Vec<_> = fk.host_columns(database).collect();
            fk_host_columns.sort_unstable();
            if fk_host_columns == host_columns {
                return true;
            }
        }
        false
    }

    /// Returns the number of host columns in the foreign key.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id1 INT, id2 INT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE host_table (
    ///     ref_id1 INT,
    ///     ref_id2 INT,
    ///     FOREIGN KEY (ref_id1, ref_id2) REFERENCES referenced_table(id1, id2)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert_eq!(foreign_key.number_of_host_columns(&db), 2);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn number_of_host_columns(&self, database: &Self::DB) -> usize {
        self.host_columns(database).count()
    }

    /// Returns whether the foreign key is composite (i.e., consists of more
    /// than one column).
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
    /// let db = ParserDB::try_from(
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
    /// let single_fk_table = db.table(None, "single_fk_table").unwrap();
    /// let composite_fk_table = db.table(None, "composite_fk_table").unwrap();
    /// let single_fk = single_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let composite_fk =
    ///     composite_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(!single_fk.is_composite(&db));
    /// assert!(composite_fk.is_composite(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn is_composite(&self, database: &Self::DB) -> bool {
        self.host_columns(database).nth(1).is_some()
    }

    /// Returns the match kind of the foreign key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// use sqlparser::ast::ConstraintReferenceMatchKind;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id) MATCH FULL
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert_eq!(foreign_key.match_kind(&db), ConstraintReferenceMatchKind::Full);
    /// # Ok(())
    /// # }
    /// ```
    fn match_kind(&self, database: &Self::DB) -> ConstraintReferenceMatchKind;

    /// Returns whether the foreign key is labelled with a `MATCH FULL` clause.
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
    /// let full_match_table = db.table(None, "full_match_table").unwrap();
    /// let normal_table = db.table(None, "normal_table").unwrap();
    /// let full_match_fk =
    ///     full_match_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let normal_fk = normal_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(full_match_fk.match_full(&db));
    /// assert!(!normal_fk.match_full(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn match_full(&self, database: &Self::DB) -> bool {
        matches!(self.match_kind(database), ConstraintReferenceMatchKind::Full)
    }

    /// Returns whether the foreign key includes any host columns that are
    /// nullable.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE nullable_host_table (
    ///    id INT,
    ///   FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE not_null_host_table (
    ///   id INT NOT NULL,
    /// FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let nullable_host_table = db.table(None, "nullable_host_table").unwrap();
    /// let not_null_host_table = db.table(None, "not_null_host_table").unwrap();
    /// let nullable_fk =
    ///     nullable_host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let not_null_fk =
    ///     not_null_host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(
    ///     nullable_fk.has_nullable_host_columns(&db),
    ///     "FK with nullable host columns should return true"
    /// );
    /// assert!(
    ///     !not_null_fk.has_nullable_host_columns(&db),
    ///     "FK with NOT NULL host columns should return false"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn has_nullable_host_columns(&self, database: &Self::DB) -> bool {
        self.host_columns(database)
            .any(|col: &<Self::DB as DatabaseLike>::Column| ColumnLike::is_nullable(col, database))
    }

    /// Returns whether the foreign key is always enforced, i.e., it cannot be
    /// violated.
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
    /// let nullable_fk_table = db.table(None, "nullable_fk_table").unwrap();
    /// let not_null_fk_table = db.table(None, "not_null_fk_table").unwrap();
    /// let nullable_match_full_table = db.table(None, "nullable_match_full_table").unwrap();
    /// let nullable_fk =
    ///     nullable_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let not_null_fk =
    ///     not_null_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let nullable_match_full_fk =
    ///     nullable_match_full_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(
    ///     !nullable_fk.is_always_enforced(&db),
    ///     "Nullable FK without MATCH FULL should be nullable"
    /// );
    /// assert!(not_null_fk.is_always_enforced(&db), "NOT NULL FK should not be nullable");
    /// assert!(
    ///     nullable_match_full_fk.is_always_enforced(&db),
    ///     "Nullable FK with MATCH FULL should not be nullable"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_always_enforced(&self, database: &Self::DB) -> bool {
        !self.has_nullable_host_columns(database) || self.match_full(database)
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id1 INT, id2 INT, name TEXT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE host_table (
    ///     ref_id1 INT,
    ///     ref_id2 INT,
    ///     FOREIGN KEY (ref_id1, ref_id2) REFERENCES referenced_table(id1, id2)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let referenced_column_names: Vec<&str> =
    ///     foreign_key.referenced_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(referenced_column_names, vec!["id1", "id2"]);
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db;

    /// Returns the referenced column associated with the foreign key if it
    /// consists of a single column; returns `None` if the foreign key is
    /// composite.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///   ref_id INT,
    ///   FOREIGN KEY (ref_id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE composite_fk_table (
    ///   ref_id1 INT,
    ///   ref_id2 INT,
    ///   PRIMARY KEY (ref_id1, ref_id2),
    ///   FOREIGN KEY (ref_id1, ref_id2) REFERENCES composite_fk_table(ref_id1, ref_id2)
    /// );
    /// "#,
    /// )?;
    ///
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let referenced_column =
    ///     foreign_key.referenced_column(&db).expect("Should have a single referenced column");
    /// assert_eq!(referenced_column.column_name(), "id");
    /// let composite_fk_table = db.table(None, "composite_fk_table").unwrap();
    /// let composite_foreign_key =
    ///     composite_fk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(
    ///     composite_foreign_key.referenced_column(&db).is_none(),
    ///     "Composite foreign key should not have a single referenced column"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_column<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> Option<&'db <Self::DB as DatabaseLike>::Column> {
        let mut referenced_columns = self.referenced_columns(database);
        let first_column = referenced_columns.next()?;
        if referenced_columns.next().is_none() { Some(first_column) } else { None }
    }

    /// Returns whether the foreign key is self-referential, i.e., the host
    /// table is the same as the referenced table.
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
    /// let db = ParserDB::try_from(
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
    /// let self_ref_table = db.table(None, "self_ref_table").unwrap();
    /// let normal_ref_table = db.table(None, "normal_ref_table").unwrap();
    /// let self_ref_fk = self_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let normal_ref_fk =
    ///     normal_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(self_ref_fk.is_self_referential(&db));
    /// assert!(!normal_ref_fk.is_self_referential(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn is_self_referential(&self, database: &Self::DB) -> bool {
        self.host_table(database) == self.referenced_table(database)
    }

    /// Returns whether the foreign key references any of the ancestor tables
    /// of the host table in the given database schema.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent (id INT PRIMARY KEY);
    /// CREATE TABLE parent (id INT PRIMARY KEY, FOREIGN KEY (id) REFERENCES grandparent(id));
    /// CREATE TABLE child (
    ///     id INT PRIMARY KEY,
    ///     other_id INT,
    ///     FOREIGN KEY (id) REFERENCES parent(id),
    ///     FOREIGN KEY (other_id) REFERENCES grandparent(id)
    /// );
    /// CREATE TABLE other (id INT PRIMARY KEY);
    /// CREATE TABLE child_other (
    ///     id INT PRIMARY KEY,
    ///     other_id INT,
    ///     FOREIGN KEY (id) REFERENCES parent(id),
    ///     FOREIGN KEY (other_id) REFERENCES other(id)
    /// );
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child").unwrap();
    /// let foreign_keys: Vec<_> = child_table.foreign_keys(&db).collect();
    /// let parent_fk =
    ///     foreign_keys.iter().find(|fk| fk.referenced_table(&db).table_name() == "parent").unwrap();
    /// let grandparent_fk = foreign_keys
    ///     .iter()
    ///     .find(|fk| fk.referenced_table(&db).table_name() == "grandparent")
    ///     .unwrap();
    ///
    /// assert!(parent_fk.references_ancestor_table(&db));
    /// assert!(grandparent_fk.references_ancestor_table(&db));
    ///
    /// let child_other_table = db.table(None, "child_other").unwrap();
    /// let other_fk = child_other_table
    ///     .foreign_keys(&db)
    ///     .find(|fk| fk.referenced_table(&db).table_name() == "other")
    ///     .unwrap();
    /// assert!(!other_fk.references_ancestor_table(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn references_ancestor_table(&self, database: &Self::DB) -> bool {
        let host_table = self.host_table(database);
        let referenced_table = self.referenced_table(database);
        host_table.ancestral_extended_tables(database).contains(&referenced_table)
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
    /// let db = ParserDB::try_from(
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
    /// let pk_ref_table = db.table(None, "pk_ref_table").unwrap();
    /// let non_pk_ref_table = db.table(None, "non_pk_ref_table").unwrap();
    /// let pk_ref_fk = pk_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let non_pk_ref_fk =
    ///     non_pk_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(pk_ref_fk.is_referenced_primary_key(&db));
    /// assert!(!non_pk_ref_fk.is_referenced_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn is_referenced_primary_key(&self, database: &Self::DB) -> bool {
        let referenced_table = self.referenced_table(database);
        let mut pk_columns = referenced_table.primary_key_columns(database).peekable();
        let mut fk_columns = self.referenced_columns(database).peekable();

        while let (Some(fk_col), Some(pk_col)) = (fk_columns.peek(), pk_columns.peek()) {
            if fk_col != pk_col {
                return false;
            }
            fk_columns.next();
            pk_columns.next();
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE referenced_table_with_unique (id INT PRIMARY KEY, name TEXT, UNIQUE(name), UNIQUE(id, name));
    /// CREATE TABLE pk_ref_table (
    ///     ref_id INT,
    ///     FOREIGN KEY (ref_id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE non_pk_ref_table (
    ///     ref_name TEXT,
    ///     referenced_table_with_unique_id INT,
    ///     FOREIGN KEY (ref_name) REFERENCES referenced_table_with_unique(name),
    ///     FOREIGN KEY (referenced_table_with_unique_id, ref_name) REFERENCES referenced_table_with_unique(id, name)
    /// );
    /// "#,
    /// )?;
    /// let pk_ref_table = db.table(None, "pk_ref_table").unwrap();
    /// let non_pk_ref_table = db.table(None, "non_pk_ref_table").unwrap();
    /// let pk_ref_fk = pk_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let [non_pk_ref_fk, another_non_pk_ref_fk] =
    ///     non_pk_ref_table.foreign_keys(&db).collect::<Vec<_>>()[..]
    /// else {
    ///     panic!("Should have two foreign keys");
    /// };
    /// assert!(pk_ref_fk.is_referenced_primary_key(&db));
    /// assert!(!non_pk_ref_fk.is_referenced_primary_key(&db));
    /// assert!(non_pk_ref_fk.is_referenced_unique_key(&db).is_some());
    /// assert!(pk_ref_fk.is_referenced_unique_key(&db).is_some());
    /// assert!(!another_non_pk_ref_fk.is_referenced_primary_key(&db));
    /// assert!(another_non_pk_ref_fk.is_referenced_unique_key(&db).is_some());
    /// # Ok(())
    /// # }
    /// ```
    fn is_referenced_unique_key<'db>(
        &self,
        database: &'db Self::DB,
    ) -> Option<&'db <Self::DB as DatabaseLike>::UniqueIndex>
    where
        Self: 'db,
    {
        let referenced_table = self.referenced_table(database);
        let referenced_columns: Vec<_> = self.referenced_columns(database).collect();
        referenced_table.unique_indices(database).find(
            |index: &&<Self::DB as DatabaseLike>::UniqueIndex| {
                let index_columns: Vec<_> = UniqueIndexLike::columns(*index, database).collect();
                index_columns.len() == referenced_columns.len()
                    && index_columns.iter().all(|col| referenced_columns.contains(col))
            },
        )
    }

    /// Returns whether the foreign key locally matches the primary key of the
    /// host table.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE pk_host_table (
    ///     id INT PRIMARY KEY,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE non_pk_host_table (
    ///     id INT PRIMARY KEY,
    ///     ref_id INT,
    ///     FOREIGN KEY (ref_id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let pk_host_table = db.table(None, "pk_host_table").unwrap();
    /// let non_pk_host_table = db.table(None, "non_pk_host_table").unwrap();
    /// let pk_fk = pk_host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let non_pk_fk = non_pk_host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(pk_fk.is_host_primary_key(&db));
    /// assert!(!non_pk_fk.is_host_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn is_host_primary_key(&self, database: &Self::DB) -> bool {
        let mut pk_columns = self.host_table(database).primary_key_columns(database).peekable();
        let mut fk_columns = self.host_columns(database).peekable();

        while let (Some(fk_col), Some(pk_col)) = (fk_columns.peek(), pk_columns.peek()) {
            if fk_col != pk_col {
                return false;
            }
            fk_columns.next();
            pk_columns.next();
        }

        // We check that there are no remaining columns in either iterator.
        fk_columns.next().is_none() && pk_columns.next().is_none()
    }

    /// Returns whether the foreign key includes (but does not necessarily
    /// match) all the primary key columns of the host table.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE composite_pk_table (
    ///     id INT,
    ///     other_id INT,
    ///     PRIMARY KEY (id, other_id),
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// CREATE TABLE single_pk_table (
    ///     id INT PRIMARY KEY,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id)
    /// );
    /// "#,
    /// )?;
    /// let composite_pk_table = db.table(None, "composite_pk_table").unwrap();
    /// let single_pk_table = db.table(None, "single_pk_table").unwrap();
    /// let composite_fk =
    ///     composite_pk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let single_fk = single_pk_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(!composite_fk.includes_host_primary_key(&db), "FK does not include all PK columns");
    /// assert!(single_fk.includes_host_primary_key(&db), "FK includes all PK columns");
    /// # Ok(())
    /// # }
    /// ```
    fn includes_host_primary_key(&self, database: &Self::DB) -> bool {
        let pk_columns: Vec<_> = self.host_table(database).primary_key_columns(database).collect();
        let fk_columns: Vec<_> = self.host_columns(database).collect();
        pk_columns.iter().all(|pk| fk_columns.contains(pk))
    }

    /// Returns whether the foreign key includes (but does not necessarily
    /// match) all the primary key columns of the referenced table.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_composite_pk_table (id1 INT, id2 INT, name TEXT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE full_ref_table (
    ///     ref_id1 INT,
    ///     ref_id2 INT,
    ///     FOREIGN KEY (ref_id1, ref_id2) REFERENCES referenced_composite_pk_table(id1, id2)
    /// );
    /// CREATE TABLE partial_ref_table (
    ///     ref_id1 INT,
    ///     FOREIGN KEY (ref_id1) REFERENCES referenced_composite_pk_table(id1)
    /// );
    /// "#,
    /// )?;
    /// let full_ref_table = db.table(None, "full_ref_table").unwrap();
    /// let partial_ref_table = db.table(None, "partial_ref_table").unwrap();
    /// let full_fk = full_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let partial_fk = partial_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(full_fk.includes_referenced_primary_key(&db), "FK includes all referenced PK columns");
    /// assert!(
    ///     !partial_fk.includes_referenced_primary_key(&db),
    ///     "FK does not include all referenced PK columns"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn includes_referenced_primary_key(&self, database: &Self::DB) -> bool {
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
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
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
    /// let extension_table = db.table(None, "extension_table").unwrap();
    /// let reference_table = db.table(None, "reference_table").unwrap();
    /// let self_ref_table = db.table(None, "self_ref_table").unwrap();
    /// let extension_fk = extension_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let reference_fk = reference_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let self_ref_fk = self_ref_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(extension_fk.is_extension_foreign_key(&db), "Should be extension FK");
    /// assert!(!reference_fk.is_extension_foreign_key(&db), "parent_id is not the primary key");
    /// assert!(
    ///     !self_ref_fk.is_extension_foreign_key(&db),
    ///     "Self-referential FK should not be extension FK"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_extension_foreign_key(&self, database: &Self::DB) -> bool {
        self.is_host_primary_key(database)
            && self.is_referenced_primary_key(database)
            && !self.is_self_referential(database)
    }

    /// Returns whether the key is a singleton foreign key, i.e. it is the only
    /// foreign key to refer to a particular foreign table within the context
    /// of its table of definition.
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
    /// let db = ParserDB::try_from(
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
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let singleton_table = db.table(None, "singleton_table").unwrap();
    /// let fks: Vec<_> = host_table.foreign_keys(&db).collect();
    /// let fk1 = &fks[0];
    /// let fk2 = &fks[1];
    /// let singleton_fk = singleton_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// assert!(!fk1.is_singleton(&db), "Not a singleton FK");
    /// assert!(!fk2.is_singleton(&db), "Not a singleton FK");
    /// assert!(singleton_fk.is_singleton(&db), "Should be a singleton FK");
    /// # Ok(())
    /// # }
    /// ```
    fn is_singleton(&self, database: &Self::DB) -> bool {
        if self.is_self_referential(database) {
            return false;
        }
        if self.is_composite(database) {
            return false;
        }
        let foreign_table = self.referenced_table(database);
        self.host_table(database)
            .foreign_keys(database)
            .map(Borrow::borrow)
            .all(|fk: &Self| fk == self || fk.referenced_table(database) != foreign_table)
    }

    /// Returns the referenced column curresponding to the given host column in
    /// the foreign key.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    /// * `host_column` - The host column for which to find the corresponding
    ///   referenced column.
    ///
    /// # Panics
    ///
    /// If the given host column is not part of the foreign key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT, UNIQUE(id, name));
    /// CREATE TABLE host_table (
    ///     ref_id INT,
    ///     ref_name TEXT,
    ///     FOREIGN KEY (ref_id, ref_name) REFERENCES referenced_table(id, name)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_key = host_table.foreign_keys(&db).next().expect("Should have a foreign key");
    /// let ref_id_col = host_table.column("ref_id", &db).expect("Should have ref_id column");
    /// let ref_name_col = host_table.column("ref_name", &db).expect("Should have ref_name column");
    /// let referenced_id_col = foreign_key.referenced_column_for_host_column(&db, &ref_id_col);
    /// let referenced_name_col = foreign_key.referenced_column_for_host_column(&db, &ref_name_col);
    /// assert_eq!(referenced_id_col.column_name(), "id");
    /// assert_eq!(referenced_name_col.column_name(), "name");
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_column_for_host_column<'db>(
        &'db self,
        database: &'db Self::DB,
        host_column: &'db <Self::DB as DatabaseLike>::Column,
    ) -> &'db <Self::DB as DatabaseLike>::Column
    where
        Self: 'db,
    {
        self.host_columns(database)
            .zip(self.referenced_columns(database))
            .find_map(|(hc, rc)| if hc == host_column { Some(rc) } else { None })
            .expect("Host column is not part of the foreign key")
    }
}
