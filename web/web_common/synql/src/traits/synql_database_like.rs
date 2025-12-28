//! Submodule defining a database which can be used in the `SynQL` struct.

use sql_traits::traits::DatabaseLike;

use crate::traits::TableSynLike;

/// Trait representing a database that can be used with `SynQL`.
pub trait SynQLDatabaseLike: DatabaseLike
where
    <Self as DatabaseLike>::Table: TableSynLike,
{
}

impl<DB> SynQLDatabaseLike for DB
where
    DB: DatabaseLike,
    DB::Table: TableSynLike,
{
}
