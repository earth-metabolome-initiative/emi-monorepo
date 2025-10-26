//! Submodule providing the `PgDatabase` struct which holds data queried from
//! the PostgreSQL information schema and implements the
//! [`DatabaseLike`](sql_traits::prelude::DatabaseLike) trait.

use sql_traits::structs::GenericDB;

use crate::models::{CheckConstraint, Column, KeyColumnUsage, PgIndex, PgProc, Table};
mod key_column_usage_metadata;
pub use key_column_usage_metadata::KeyColumnUsageMetadata;
mod pg_proc_metadata;
pub use pg_proc_metadata::PgProcMetadata;

mod builder;
pub use builder::PgDatabaseBuilder;

/// Type alias representing a PostgreSQL database with loaded metadata.
///
/// This is a specialization of [`GenericDB`] configured for PostgreSQL, using:
/// - [`Table`](crate::models::Table) from `information_schema.tables` as the
///   table type
/// - [`Column`](crate::models::Column) from `information_schema.columns` as the
///   column type
/// - [`PgIndex`](crate::models::PgIndex) from `pg_catalog.pg_index` as the
///   unique index type
/// - [`KeyColumnUsage`](crate::models::KeyColumnUsage) from
///   `information_schema.key_column_usage` as the foreign key type
/// - [`PgProc`](crate::models::PgProc) from `pg_catalog.pg_proc` as the
///   function type
/// - [`CheckConstraint`](crate::models::CheckConstraint) from
///   `information_schema.check_constraints` as the check constraint type
///
/// The `PgDatabase` implements
/// [`DatabaseLike`](sql_traits::prelude::DatabaseLike), providing methods to
/// iterate over tables, columns, foreign keys, and other database objects.
pub type PgDatabase = GenericDB<Table, Column, PgIndex, KeyColumnUsage, PgProc, CheckConstraint>;
