//! Submodule providing the `PgStatsExt` struct representing a row of the
//! `pg_stats_ext` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stats_ext` view.
///
/// The `pg_stats_ext` view provides access to extended statistics in a more
/// readable format than the underlying catalog tables.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-stats-ext.html).
///
/// Note: This struct does not derive `Hash`, `Eq`, or `Ord` because it contains
/// `f64` fields.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stats_ext::pg_stats_ext)]
pub struct PgStatsExt {
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub tablename: Option<String>,
    /// Statistics schema name.
    pub statistics_schemaname: Option<String>,
    /// Statistics name.
    pub statistics_name: Option<String>,
    /// Statistics owner.
    pub statistics_owner: Option<String>,
    /// Attribute names.
    pub attnames: Option<Vec<String>>,
    /// Expressions.
    pub exprs: Option<Vec<String>>,
    /// Statistics kinds.
    pub kinds: Option<Vec<String>>,
    /// Inherited stats flag.
    pub inherited: Option<bool>,
    /// Most common values.
    pub most_common_vals: Option<Vec<String>>,
    /// Most common value nulls flags.
    pub most_common_val_nulls: Option<Vec<bool>>,
    /// Most common frequencies.
    pub most_common_freqs: Option<Vec<f64>>,
    /// Most common base frequencies.
    pub most_common_base_freqs: Option<Vec<f64>>,
}
