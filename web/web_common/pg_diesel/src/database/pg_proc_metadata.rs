//! Metadata for PostgreSQL functions and procedures.
//!
//! This module provides [`PgProcMetadata`], which encapsulates type information
//! for a PostgreSQL function or procedure, including:
//! - Argument types (from `pg_proc.proargtypes`)
//! - Return type (from `pg_proc.prorettype`)
//!
//! This metadata is used by the [`PgDatabase`](crate::database::PgDatabase) to
//! provide function introspection through the `sql_traits` trait system.

use diesel::PgConnection;

use crate::models::{PgProc, PgType};

#[derive(Debug, Clone)]
/// Struct collecting metadata about a PostgreSQL function represented by a
/// [`PgProc`](crate::models::PgProc) entry.
pub struct PgProcMetadata {
    /// The argument types.
    argument_types: Vec<PgType>,
    /// The return type.
    return_type: Option<PgType>,
}

impl PgProcMetadata {
    /// Creates a new `PgProcMetadata` instance from a `PgProc` and database
    /// connection.
    ///
    /// # Arguments
    ///
    /// * `pg_proc` - The PostgreSQL function to get metadata for.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// Returns an error if the type information cannot be retrieved from the
    /// database.
    pub fn new(pg_proc: &PgProc, conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        let argument_types = pg_proc.argument_types(conn)?;

        let return_type =
            if pg_proc.prorettype == 0 { None } else { Some(pg_proc.return_type(conn)?) };

        Ok(Self { argument_types, return_type })
    }

    /// Returns the argument types.
    pub fn argument_types(&self) -> &[PgType] {
        &self.argument_types
    }

    /// Returns the return type.
    pub fn return_type(&self) -> Option<&PgType> {
        self.return_type.as_ref()
    }
}
