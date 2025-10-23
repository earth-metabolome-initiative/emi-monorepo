//! Submodule providing the `PgShseclabel` struct representing a row of the
//! `pg_shseclabel` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_shseclabel` table.
///
/// The `pg_shseclabel` table stores security labels on shared database
/// objects visible across all databases in the cluster.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-shseclabel.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_shseclabel::pg_shseclabel)]
pub struct PgShseclabel {
    /// Object OID.
    pub objoid: u32,
    /// System catalog OID.
    pub classoid: u32,
    /// Provider name.
    pub provider: String,
    /// Security label.
    pub label: String,
}
