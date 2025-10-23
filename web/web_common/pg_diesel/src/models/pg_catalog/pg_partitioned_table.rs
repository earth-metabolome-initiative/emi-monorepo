//! Submodule providing the `PgPartitionedTable` struct representing a row of
//! the `pg_partitioned_table` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_partitioned_table` table.
///
/// The `pg_partitioned_table` catalog contains partitioning information for
/// tables that are partitioned. It describes the partitioning strategy and
/// related metadata.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-partitioned-table.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_partitioned_table::pg_partitioned_table)]
pub struct PgPartitionedTable {
    /// OID of the partitioned table.
    pub partrelid: u32,
    /// Partitioning strategy.
    pub partstrat: String,
    /// Number of columns in partition key.
    pub partnatts: i16,
    /// OID of the default partition.
    pub partdefid: u32,
    /// Partition key expressions.
    pub partexprs: Option<String>,
}
