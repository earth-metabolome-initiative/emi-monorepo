//! Submodule providing the `PgTable` struct representing a row of the
//! `pg_tables` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_tables` view.
///
/// The `pg_tables` view provides access to useful information about each
/// table in the database.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-tables.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_tables::pg_tables)]
pub struct PgTable {
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub tablename: Option<String>,
    /// Table owner.
    pub tableowner: Option<String>,
    /// Tablespace name.
    pub tablespace: Option<String>,
    /// Has indexes flag.
    pub hasindexes: Option<bool>,
    /// Has rules flag.
    pub hasrules: Option<bool>,
    /// Has triggers flag.
    pub hastriggers: Option<bool>,
    /// Row security flag.
    pub rowsecurity: Option<bool>,
}
