//! Core traits for PostgreSQL metadata types.
//!
//! This module defines traits that are implemented by various model structs
//! to provide common functionality:
//!
//! - [`HasOid`]: For types that have a PostgreSQL Object Identifier (OID)
//! - [`PostgresType`]: For types that can resolve their PostgreSQL type
//!   information
//!
//! These traits are used by the caching infrastructure and type resolution
//! logic.

pub mod oid;
pub use oid::HasOid;
pub mod postgres_type;
pub use postgres_type::PostgresType;
