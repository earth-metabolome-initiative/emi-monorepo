//! Submodule providing the `PgShdepend` struct representing a row of the
//! `pg_shdepend` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_shdepend` table.
///
/// The `pg_shdepend` catalog records dependency relationships between database
/// objects and shared objects such as roles. This allows the system to prevent
/// dropping shared objects that are still needed.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-shdepend.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_shdepend::pg_shdepend)]
pub struct PgShdepend {
    /// Database OID.
    pub dbid: u32,
    /// System catalog OID.
    pub classid: u32,
    /// Dependent object OID.
    pub objid: u32,
    /// Sub-object ID.
    pub objsubid: i32,
    /// Referenced catalog OID.
    pub refclassid: u32,
    /// Referenced object OID.
    pub refobjid: u32,
    /// Dependency type.
    pub deptype: String,
}
