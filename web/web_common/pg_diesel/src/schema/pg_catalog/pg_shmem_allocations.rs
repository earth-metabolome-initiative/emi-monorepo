//! Submodule for the `pg_catalog.pg_shmem_allocations` view schema.

diesel::table! {
    /// `pg_catalog.pg_shmem_allocations` — view showing shared memory allocations.
    /// Each row represents a shared memory segment allocated by the server.
    /// Uses `name` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_shmem_allocations (name) {
        /// Name of the shared memory allocation.
        name -> Nullable<Text>,
        /// Offset of the allocation in shared memory.
        off -> Nullable<BigInt>,
        /// Requested size of the allocation in bytes.
        size -> Nullable<BigInt>,
        /// Actual allocated size in bytes (may be larger due to alignment).
        allocated_size -> Nullable<BigInt>,
    }
}
