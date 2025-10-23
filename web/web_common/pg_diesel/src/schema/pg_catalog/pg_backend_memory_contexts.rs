//! Submodule for the `pg_catalog.pg_backend_memory_contexts` view schema.

diesel::table! {
    /// `pg_catalog.pg_backend_memory_contexts` — view of backend memory contexts.
    /// Shows memory context information for the current backend process.
    pg_catalog.pg_backend_memory_contexts (name, ident) {
        /// Name of the memory context (part of composite primary key).
        name -> Nullable<Text>,
        /// Identification string of the memory context (part of composite primary key).
        ident -> Nullable<Text>,
        /// Name of the parent memory context.
        parent -> Nullable<Text>,
        /// Nesting level of the memory context.
        level -> Nullable<Integer>,
        /// Total number of bytes allocated in this context.
        total_bytes -> Nullable<BigInt>,
        /// Total number of blocks allocated in this context.
        total_nblocks -> Nullable<BigInt>,
        /// Number of free bytes in this context.
        free_bytes -> Nullable<BigInt>,
        /// Number of free chunks in this context.
        free_chunks -> Nullable<BigInt>,
        /// Number of used bytes in this context.
        used_bytes -> Nullable<BigInt>,
    }
}
