//! Submodule defining a database which can be used in the `SynQL` struct.

use sql_traits::traits::DatabaseLike;
use synql_diesel_schema::traits::TableSchema;
use synql_models::traits::TableModelLike;

/// Trait representing a database that can be used with `SynQL`.
pub trait SynQLDatabaseLike: DatabaseLike<Table = <Self as SynQLDatabaseLike>::SynQLTable> {
    /// Table which can be used with `SynQL`.
    type SynQLTable: TableSchema<Database = Self> + TableModelLike<Database = Self>;
}

impl<DB> SynQLDatabaseLike for DB
where
    DB: DatabaseLike,
    DB::Table: TableSchema<Database = DB> + TableModelLike<Database = DB>,
{
    type SynQLTable = DB::Table;
}
