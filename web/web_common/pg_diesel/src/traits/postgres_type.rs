//! Trait for types that can resolve their PostgreSQL type information.
//!
//! This module defines the [`PostgresType`] trait for database objects that
//! have an associated PostgreSQL data type. The trait provides a way to obtain
//! complete [`PgType`] metadata, which is used for:
//! - Diesel type mapping during schema generation
//! - Type resolution in code generators
//! - Understanding column types and their properties

use diesel::PgConnection;

use crate::models::PgType;

/// Trait for database objects that can resolve their PostgreSQL type.
///
/// This trait provides a unified interface for obtaining [`PgType`]
/// information, whether from a direct type reference or by querying the
/// database.
///
/// ## Implementors
///
/// - [`PgType`]: Returns itself directly (no database query needed)
/// - [`Column`](crate::models::Column): Queries `pg_catalog.pg_type` based on
///   the column's type OID
///
/// ## Usage
///
/// The trait is used throughout the crate to resolve type names for Diesel
/// schema generation and to understand the properties of column types (size,
/// category, etc.).
pub trait PostgresType {
    /// Returns the name of the postgres type associated to the current DB
    /// object.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    fn postgres_type(&self, conn: &mut PgConnection) -> Result<PgType, diesel::result::Error>;
}
