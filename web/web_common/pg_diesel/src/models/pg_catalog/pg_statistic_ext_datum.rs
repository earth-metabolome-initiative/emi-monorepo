//! Submodule providing the `PgStatisticExtDatum` struct representing a row of
//! the `pg_statistic_ext_data` table in `PostgreSQL`.

use diesel::{Identifiable, Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_statistic_ext_data` table.
///
/// The `pg_statistic_ext_data` table holds data for extended planner statistics
/// objects, containing the actual statistical values.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-statistic-ext-data.html).
#[derive(
    Identifiable, Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_statistic_ext_data::pg_statistic_ext_data)]
#[diesel(primary_key(stxoid, stxdinherit))]
pub struct PgStatisticExtDatum {
    /// Statistics object OID.
    pub stxoid: u32,
    /// Inherited stats flag.
    pub stxdinherit: bool,
}
