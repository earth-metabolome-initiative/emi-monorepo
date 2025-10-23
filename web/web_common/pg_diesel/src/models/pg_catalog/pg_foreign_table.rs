//! Submodule providing the `PgForeignTable` struct representing a row of the
//! `pg_foreign_table` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_foreign_table` table.
///
/// The `pg_foreign_table` system catalog contains auxiliary information about
/// foreign tables. A foreign table is primarily represented by a pg_class
/// entry, just like a regular table. This catalog entry contains information
/// that is specific to foreign tables.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-foreign-table.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_foreign_table::pg_foreign_table)]
pub struct PgForeignTable {
    /// OID of the pg_class entry for this foreign table.
    pub ftrelid: u32,
    /// OID of the foreign server.
    pub ftserver: u32,
    /// Foreign table options.
    pub ftoptions: Option<Vec<String>>,
}
