//! Trait for types with PostgreSQL Object Identifiers (OIDs).
//!
//! This module defines the [`HasOid`] trait for database types that have a
//! PostgreSQL Object Identifier. OIDs are unique identifiers assigned by
//! PostgreSQL to database objects like tables, types, functions, and indexes.
//!
//! ## Usage in Caching
//!
//! This trait is used by the caching infrastructure (particularly the
//! `oid_auto_cached` macro) to derive cache keys from OIDs, enabling efficient
//! memoization of database queries.

/// Trait for types that have a PostgreSQL Object Identifier (OID).
///
/// OIDs are unsigned 32-bit integers that uniquely identify database objects
/// within a PostgreSQL cluster. This trait provides a uniform way to extract
/// the OID from various system catalog types.
///
/// ## Implementors
///
/// This trait is implemented for catalog types such as:
/// - `PgType`, `PgProc`, `PgConstraint` (using their `oid` field)
/// - `PgIndex` (using its `indexrelid` field)
/// - `PgDescription` (using its `objoid` field)
pub trait HasOid {
    /// Returns the OID of the struct.
    fn oid(&self) -> u32;
}
