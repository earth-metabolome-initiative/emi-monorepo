//! Submodule defining a database which can be used in the `SynQL` struct.

use sql_relations::traits::InheritableDatabaseLike;
use sql_traits::traits::DatabaseLike;
use synql_diesel_schema::traits::TableSchema;
use synql_models::traits::TableModelLike;

/// Trait representing a database that can be used with `SynQL`.
pub trait SynQLDatabaseLike: InheritableDatabaseLike
where
    <Self as DatabaseLike>::Table: TableSchema + TableModelLike,
{
}

impl<DB> SynQLDatabaseLike for DB
where
    DB: InheritableDatabaseLike,
    DB::Table: TableSchema + TableModelLike,
{
}
