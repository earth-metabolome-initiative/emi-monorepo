//! Submodule providing the `PgSequenceView` struct representing a row of the
//! `pg_sequences` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_sequences` view.
///
/// The `pg_sequences` view provides a user-friendly representation of
/// sequences, showing names and configuration parameters rather than just OIDs.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-sequences.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_sequences::pg_sequences)]
pub struct PgSequenceView {
    /// Schema name.
    pub schemaname: Option<String>,
    /// Sequence name.
    pub sequencename: Option<String>,
    /// Owner name.
    pub sequenceowner: Option<String>,
    /// OID of data type.
    pub data_type: Option<u32>,
    /// Start value.
    pub start_value: Option<i64>,
    /// Minimum value.
    pub min_value: Option<i64>,
    /// Maximum value.
    pub max_value: Option<i64>,
    /// Increment value.
    pub increment_by: Option<i64>,
    /// Whether cycles.
    pub cycle: Option<bool>,
    /// Cache size.
    pub cache_size: Option<i64>,
    /// Last value.
    pub last_value: Option<i64>,
}
