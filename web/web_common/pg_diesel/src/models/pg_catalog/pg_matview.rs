//! Submodule providing the `PgMatview` struct representing a row of the
//! `pg_matviews` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_matviews` view.
///
/// The `pg_matviews` view provides information about materialized views
/// in the database. Materialized views are snapshots of query results that
/// can be refreshed periodically.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-matviews.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_matviews::pg_matviews)]
pub struct PgMatview {
    /// Schema name.
    pub schemaname: Option<String>,
    /// Materialized view name.
    pub matviewname: Option<String>,
    /// Owner name.
    pub matviewowner: Option<String>,
    /// Tablespace name.
    pub tablespace: Option<String>,
    /// Whether the materialized view has indexes.
    pub hasindexes: Option<bool>,
    /// Whether the materialized view is populated.
    pub ispopulated: Option<bool>,
    /// SQL definition of the materialized view.
    pub definition: Option<String>,
}
