//! Submodule providing the `PgDatabase` struct which holds data queried from
//! the PostgreSQL information schema and implements the
//! [`DatabaseLike`](sql_traits::prelude::DatabaseLike) trait.

use sql_traits::structs::GenericDB;

use crate::models::Table;
mod key_column_usage_metadata;
pub use key_column_usage_metadata::KeyColumnUsageMetadata;

mod builder;
pub use builder::PgDatabaseBuilder;

/// Struct representing a PostgreSQL database schema, implementing the
/// [`DatabaseLike`](sql_traits::prelude::DatabaseLike) trait.
pub type PgDatabase = GenericDB<Table>;
