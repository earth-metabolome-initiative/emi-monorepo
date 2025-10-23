//! Submodule providing the `PgSeclabel` struct representing a row of the
//! `pg_seclabel` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_seclabel` table.
///
/// The `pg_seclabel` catalog stores security labels on database objects.
/// Security labels are used by label-based mandatory access control (MAC)
/// systems like SELinux.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-seclabel.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_seclabel::pg_seclabel)]
#[diesel(primary_key(objoid, classoid, objsubid, provider))]
pub struct PgSeclabel {
    /// OID of the object.
    pub objoid: u32,
    /// OID of the system catalog.
    pub classoid: u32,
    /// Sub-object ID.
    pub objsubid: i32,
    /// Provider name.
    pub provider: String,
    /// Security label.
    pub label: String,
}
