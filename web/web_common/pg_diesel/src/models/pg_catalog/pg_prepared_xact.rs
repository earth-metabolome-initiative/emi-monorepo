//! Submodule providing the `PgPreparedXact` struct representing a row of the
//! `pg_prepared_xacts` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_prepared_xacts` view.
///
/// The `pg_prepared_xacts` view displays information about transactions that
/// are currently prepared for two-phase commit. These are transactions that
/// have been prepared with PREPARE TRANSACTION and are waiting for either
/// COMMIT PREPARED or ROLLBACK PREPARED.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-prepared-xacts.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_prepared_xacts::pg_prepared_xacts)]
pub struct PgPreparedXact {
    /// Transaction ID.
    pub transaction: Option<u32>,
    /// Global transaction identifier.
    pub gid: Option<String>,
    /// Time when prepared.
    pub prepared: Option<std::time::SystemTime>,
    /// Owner of the transaction.
    pub owner: Option<String>,
    /// Database name.
    pub database: Option<String>,
}
