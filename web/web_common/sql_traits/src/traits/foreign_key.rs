//! Submodule definining the `ForeignKeyLike` trait for SQL referenced keys.

use sqlparser::ast::MatchKind;

use crate::traits::{ColumnLike, DatabaseLike, TableLike};

/// A foreign key constraint is a rule that specifies a relationship between
/// two tables. This trait represents such a foreign key constraint in a
/// database-agnostic way.
pub trait ForeignKeyLike {
    /// The column type associated with the foreign key.
    type Column: ColumnLike;
    /// The table type associated with the foreign key.
    type Table: TableLike<Column = Self::Column, Database = Self::Database>;
    /// The database type associated with the foreign key.
    type Database: DatabaseLike<Table = Self::Table, Column = Self::Column>;

    /// Returns the name of the foreign key, if it has one.
    fn foreign_key_name(&self) -> Option<&str>;

    /// Returns whether the foreign key is on delete cascade.
    fn on_delete_cascade(&self, database: &Self::Database) -> bool;

    /// Returns the referenced table that the foreign key points to.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
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
    fn is_composite(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        self.host_columns(database, host_table).nth(1).is_some()
    }

    /// Returns the match kind of the foreign key.
    fn match_kind(&self, database: &Self::Database) -> MatchKind;

    /// Returns whether the foreign key is labelled with a `MATCH FULL` clause.
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
    fn is_nullable(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        self.host_columns(database, host_table).any(|col| col.is_nullable())
            && !self.match_full(database)
    }

    /// Returns an iterator over the columns in the referenced table that are
    /// part of the foreign key.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
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
    fn is_extension_foreign_key(
        &self,
        database: &Self::Database,
        host_table: &Self::Table,
    ) -> bool {
        self.is_host_primary_key(database, host_table)
            && self.is_referenced_primary_key(database)
            && !self.is_self_referential(database, host_table)
    }
}
