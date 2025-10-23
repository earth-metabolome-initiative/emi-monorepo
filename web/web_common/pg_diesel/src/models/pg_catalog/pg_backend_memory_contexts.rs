//! PostgreSQL backend memory contexts view model.
//!
//! This module provides the `PgBackendMemoryContexts` struct for working with
//! the `pg_catalog.pg_backend_memory_contexts` system view.

/// Represents a row from the `pg_catalog.pg_backend_memory_contexts` view.
///
/// Shows memory context information for the current backend process.
/// This view provides insights into memory allocation and usage patterns
/// within the PostgreSQL backend, useful for debugging and monitoring.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_backend_memory_contexts::pg_backend_memory_contexts)]
pub struct PgBackendMemoryContexts {
    /// Name of the memory context (part of composite primary key).
    pub name: Option<String>,
    /// Identification string of the memory context (part of composite primary
    /// key).
    pub ident: Option<String>,
    /// Name of the parent memory context.
    pub parent: Option<String>,
    /// Nesting level of the memory context.
    pub level: Option<i32>,
    /// Total number of bytes allocated in this context.
    pub total_bytes: Option<i64>,
    /// Total number of blocks allocated in this context.
    pub total_nblocks: Option<i64>,
    /// Number of free bytes in this context.
    pub free_bytes: Option<i64>,
    /// Number of free chunks in this context.
    pub free_chunks: Option<i64>,
    /// Number of used bytes in this context.
    pub used_bytes: Option<i64>,
}
