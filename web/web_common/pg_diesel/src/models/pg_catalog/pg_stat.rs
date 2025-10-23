//! Submodule providing the `PgStat` struct representing a row of the
//! `pg_stats` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stats` view.
///
/// The `pg_stats` view provides access to the information stored in the
/// `pg_statistic` catalog in a more readable format.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-stats.html).
///
/// Note: This struct does not derive `Hash`, `Eq`, or `Ord` because it contains
/// `f32` fields.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stats::pg_stats)]
pub struct PgStat {
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub tablename: Option<String>,
    /// Column name.
    pub attname: Option<String>,
    /// Inherited stats flag.
    pub inherited: Option<bool>,
    /// Null fraction.
    pub null_frac: Option<f32>,
    /// Average width.
    pub avg_width: Option<i32>,
    /// Number of distinct values.
    pub n_distinct: Option<f32>,
    /// Most common value frequencies.
    pub most_common_freqs: Option<Vec<f32>>,
    /// Correlation coefficient.
    pub correlation: Option<f32>,
    /// Most common element frequencies.
    pub most_common_elem_freqs: Option<Vec<f32>>,
    /// Element count histogram.
    pub elem_count_histogram: Option<Vec<f32>>,
    /// Range empty fraction.
    pub range_empty_frac: Option<f32>,
}
