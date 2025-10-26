//! Submodule defining a database which can be used in the `SynQL` struct.

use sql_traits::traits::DatabaseLike;
use synql_diesel_schema::traits::TableSchema;
use synql_models::traits::TableModelLike;

/// Trait representing a database that can be used with `SynQL`.
pub trait SynQLDatabaseLike: DatabaseLike
where
    <Self as DatabaseLike>::Table: TableSchema + TableModelLike,
{
}

impl<DB> SynQLDatabaseLike for DB
where
    DB: DatabaseLike,
    DB::Table: TableSchema + TableModelLike,
{
}
