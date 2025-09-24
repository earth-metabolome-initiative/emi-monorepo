//! Submodule providing the [`PostgresType`] trait used throughout the
//! `pg_diesel` crate which associates a postgres string type to DB objects.

use diesel::PgConnection;

use crate::models::PgType;

/// Trait to be implemented by structs derived from database tables to indicate
/// which diesel type should be used in the diesel schema to represent their
/// type.
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
