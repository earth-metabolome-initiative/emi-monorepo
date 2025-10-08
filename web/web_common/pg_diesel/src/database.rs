//! Submodule providing the `PgDatabase` struct which holds data queried from
//! the PostgreSQL information schema and implements the
//! [`DatabaseLike`](sql_traits::prelude::DatabaseLike) trait.

use sql_traits::traits::{DatabaseLike, TableLike};

use crate::models::{Column, KeyColumnUsage, PgIndex, Table};
mod key_column_usage_metadata;
use key_column_usage_metadata::KeyColumnUsageMetadata;
mod table_metadata;
use table_metadata::TableMetadata;
mod pg_index_metadata;
use pg_index_metadata::PgIndexMetadata;

use crate::traits::oid::HasOid;

mod builder;
pub use builder::PgDatabaseBuilder;

/// Struct representing a PostgreSQL database schema.
pub struct PgDatabase {
    /// List of tables in the database schema, sorted by schema and name.
    tables: Vec<(Table, TableMetadata)>,
    /// Sorted list of foreign keys in the database schema and their associated
    /// metadata.
    foreign_keys: Vec<(KeyColumnUsage, KeyColumnUsageMetadata)>,
    /// Sorted list of indices in the database schema and their associated
    /// metadata.
    indices: Vec<(PgIndex, PgIndexMetadata)>,
}

impl PgDatabase {
    /// Creates a new `PgDatabaseBuilder` instance with which to build a
    /// `PgDatabase`.
    pub fn new<'conn>() -> PgDatabaseBuilder<'conn> {
        PgDatabaseBuilder::default()
    }

    /// Returns the table metadata of the provided table.
    pub fn table_metadata(&self, table: &Table) -> &TableMetadata {
        self.tables
            .binary_search_by_key(&(table.table_schema(), table.table_name()), |(tbl, _)| {
                (tbl.table_schema(), tbl.table_name())
            })
            .map(|index| &self.tables[index].1)
            .expect("Table not found in the database")
    }

    /// Returns the metadata of the provided foreign key.
    pub fn foreign_key_metadata(&self, foreign_key: &KeyColumnUsage) -> &KeyColumnUsageMetadata {
        self.foreign_keys
            .binary_search_by_key(&foreign_key.constraint_name.as_str(), |(fk, _)| {
                fk.constraint_name.as_str()
            })
            .map(|index| &self.foreign_keys[index].1)
            .expect("Foreign key not found in the database")
    }

    /// Returns the metadata of the provided index.
    pub fn index_metadata(&self, index: &PgIndex) -> &PgIndexMetadata {
        self.indices
            .binary_search_by_key(&index.oid(), |(idx, _)| idx.oid())
            .map(|index| &self.indices[index].1)
            .expect("Index not found in the database")
    }
}

impl DatabaseLike for PgDatabase {
    type Column = Column;
    type Table = Table;
    type ForeignKey = KeyColumnUsage;

    fn tables(&self) -> impl Iterator<Item = &Self::Table> {
        self.tables.iter().map(|(table, _metadata)| table)
    }

    fn table(&self, schema: Option<&str>, table_name: &str) -> &Self::Table {
        // Given that the tables are sorted by schema and name, we can use binary search
        // to find the table efficiently.
        let key = (schema, table_name);

        self.tables
            .binary_search_by_key(&key, |(table, _)| (table.table_schema(), table.table_name()))
            .map(|index| &self.tables[index].0)
            .expect("Table not found in the database")
    }
}
