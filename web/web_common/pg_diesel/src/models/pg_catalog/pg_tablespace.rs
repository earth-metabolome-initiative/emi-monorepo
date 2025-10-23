//! Submodule providing the `PgTablespace` struct representing a row of the
//! `pg_tablespace` table in `PostgreSQL`.

use diesel::{Identifiable, Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_tablespace` table.
///
/// The `pg_tablespace` table stores information about the available
/// tablespaces.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-tablespace.html).
#[derive(
    Identifiable, Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_tablespace::pg_tablespace)]
#[diesel(primary_key(oid))]
pub struct PgTablespace {
    /// Object identifier.
    pub oid: u32,
    /// Tablespace name.
    pub spcname: String,
    /// Owner OID.
    pub spcowner: u32,
    /// Access control list.
    pub spcacl: Option<Vec<String>>,
    /// Tablespace options.
    pub spcoptions: Option<Vec<String>>,
}
