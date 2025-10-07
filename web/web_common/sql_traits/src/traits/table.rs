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
    type UniqueIndex: UniqueIndexLike;
    /// The foreign key type of the table.
    type ForeignKey: ForeignKeyLike<Table = Self, Column = Self::Column, Database = Self::Database>;

    /// Returns the name of the table.
    fn table_name(&self) -> &str;

    /// Iterates over the columns of the column using the provided schema.
    fn columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column>;

    /// Iterates over the primary key columns of the table using the provided
    /// schema.
    fn primary_key_columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column>;

    /// Returns the primary key column of the table.
    ///
    /// # Panics
    ///
    /// * If the table does not have exactly one primary key column.
    fn primary_key_column(&self, database: &Self::Database) -> Self::Column {
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
    fn is_primary_key_column(&self, database: &Self::Database, column: &Self::Column) -> bool {
        self.primary_key_columns(database).all(|col| &col == column)
    }

    /// Returns whether the table has a primary key.
    fn has_primary_key(&self, database: &Self::Database) -> bool {
        self.primary_key_columns(database).next().is_some()
    }

    /// Returns whether the table has a composite primary key.
    fn has_composite_primary_key(&self, database: &Self::Database) -> bool {
        self.primary_key_columns(database).nth(1).is_some()
    }

    /// Iterates over the check constraints of the table using the provided
    /// schema.
    fn check_constraints(
        &self,
        database: &Self::Database,
    ) -> impl Iterator<Item = Self::CheckConstraint>;

    /// Iterates over the unique indexes of the table using the provided
    /// schema.
    fn unique_indexes(&self, database: &Self::Database) -> impl Iterator<Item = Self::UniqueIndex>;

    /// Iterates over the foreign keys of the table using the provided schema.
    fn foreign_keys(&self, database: &Self::Database) -> impl Iterator<Item = Self::ForeignKey>;

    /// Returns the tables which are extended by the current table via foreign
    /// keys.
    fn extended_tables<'db>(&'db self, database: &'db Self::Database) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let primary_key_column = self.primary_key_column(database);
        let mut referenced_tables =
            self.referenced_tables_via_column(database, &primary_key_column);

        // We remove self from the list of referenced tables if present.
        referenced_tables.retain(|&table| table != self);

        referenced_tables
    }

    /// Returns the unique tables which are extended by either the current
    /// table or any of the tables it extends via foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
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
            if fk.host_columns(database, self).all(|col| &col == column)
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

    /// Returns whether the column is compatible with another column.
    fn is_compatible_with(
        &self,
        database: &Self::Database,
        host_column: &Self::Column,
        other_table: &Self,
        other_column: &Self::Column,
    ) -> bool {
        debug_assert!(
            self.columns(database).any(|col| &col == host_column),
            "Local column {:?} does not belong to table {:?}",
            host_column.column_name(),
            self.table_name()
        );
        debug_assert!(
            other_table.columns(database).any(|col| &col == other_column),
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
