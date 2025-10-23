//! Submodule providing the `PgSequence` struct representing a row of the
//! `pg_sequence` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_sequence` table.
///
/// The `pg_sequence` catalog contains information about sequences. Each
/// sequence has a row here describing its parameters like start value,
/// increment, and bounds.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-sequence.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_sequence::pg_sequence)]
pub struct PgSequence {
    /// OID of the sequence.
    pub seqrelid: u32,
    /// OID of the data type.
    pub seqtypid: u32,
    /// Start value.
    pub seqstart: i64,
    /// Increment value.
    pub seqincrement: i64,
    /// Maximum value.
    pub seqmax: i64,
    /// Minimum value.
    pub seqmin: i64,
    /// Cache size.
    pub seqcache: i64,
    /// Whether cycles.
    pub seqcycle: bool,
}
