//! Submodule providing the `PgStatistic` struct representing a row of the
//! `pg_statistic` table in `PostgreSQL`.

use diesel::{Identifiable, Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_statistic` table.
///
/// The `pg_statistic` table stores statistical data about the contents of the
/// database. Rows are created by ANALYZE and subsequently used by the query
/// planner.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-statistic.html).
///
/// Note: This struct does not derive `Hash`, `Eq`, or `Ord` because it contains
/// `f32` fields.
#[derive(Identifiable, Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_statistic::pg_statistic)]
#[diesel(primary_key(starelid, staattnum, stainherit))]
pub struct PgStatistic {
    /// Table or index OID.
    pub starelid: u32,
    /// Column number.
    pub staattnum: i16,
    /// Inherited stats flag.
    pub stainherit: bool,
    /// Null fraction.
    pub stanullfrac: f32,
    /// Average width in bytes.
    pub stawidth: i32,
    /// Number of distinct values.
    pub stadistinct: f32,
    /// Statistics kind 1.
    pub stakind1: i16,
    /// Statistics kind 2.
    pub stakind2: i16,
    /// Statistics kind 3.
    pub stakind3: i16,
    /// Statistics kind 4.
    pub stakind4: i16,
    /// Statistics kind 5.
    pub stakind5: i16,
    /// Statistics operator 1.
    pub staop1: u32,
    /// Statistics operator 2.
    pub staop2: u32,
    /// Statistics operator 3.
    pub staop3: u32,
    /// Statistics operator 4.
    pub staop4: u32,
    /// Statistics operator 5.
    pub staop5: u32,
    /// Statistics collation 1.
    pub stacoll1: u32,
    /// Statistics collation 2.
    pub stacoll2: u32,
    /// Statistics collation 3.
    pub stacoll3: u32,
    /// Statistics collation 4.
    pub stacoll4: u32,
    /// Statistics collation 5.
    pub stacoll5: u32,
    /// Numerical statistics 1.
    pub stanumbers1: Option<Vec<f32>>,
    /// Numerical statistics 2.
    pub stanumbers2: Option<Vec<f32>>,
    /// Numerical statistics 3.
    pub stanumbers3: Option<Vec<f32>>,
    /// Numerical statistics 4.
    pub stanumbers4: Option<Vec<f32>>,
    /// Numerical statistics 5.
    pub stanumbers5: Option<Vec<f32>>,
}
