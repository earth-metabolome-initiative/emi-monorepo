//! Submodule providing the `PgShmemAllocation` struct representing a row of the
//! `pg_shmem_allocations` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_shmem_allocations` view.
///
/// The `pg_shmem_allocations` view shows information about shared memory
/// allocations made by the PostgreSQL server.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-shmem-allocations.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_shmem_allocations::pg_shmem_allocations)]
pub struct PgShmemAllocation {
    /// Allocation name.
    pub name: Option<String>,
    /// Memory offset.
    pub off: Option<i64>,
    /// Requested size.
    pub size: Option<i64>,
    /// Allocated size.
    pub allocated_size: Option<i64>,
}
