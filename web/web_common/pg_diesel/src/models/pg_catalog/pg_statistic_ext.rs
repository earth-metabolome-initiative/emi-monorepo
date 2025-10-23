//! Submodule providing the `PgStatisticExt` struct representing a row of the
//! `pg_statistic_ext` table in `PostgreSQL`.

use diesel::{Identifiable, Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_statistic_ext` table.
///
/// The `pg_statistic_ext` table holds definitions of extended planner
/// statistics, which are used for more accurate query planning.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-statistic-ext.html).
#[derive(
    Identifiable, Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_statistic_ext::pg_statistic_ext)]
#[diesel(primary_key(oid))]
pub struct PgStatisticExt {
    /// Object identifier.
    pub oid: u32,
    /// Related table OID.
    pub stxrelid: u32,
    /// Statistics object name.
    pub stxname: String,
    /// Namespace OID.
    pub stxnamespace: u32,
    /// Owner OID.
    pub stxowner: u32,
    /// Statistics target.
    pub stxstattarget: Option<i16>,
    /// Statistics kinds.
    pub stxkind: Vec<String>,
    /// Expression tree.
    pub stxexprs: Option<String>,
}
