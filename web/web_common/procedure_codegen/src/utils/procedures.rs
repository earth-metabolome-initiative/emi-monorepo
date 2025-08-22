//! Submodule defining a function to return the set of `Procedure`s tables
//! defined in the database.

use diesel::PgConnection;
use webcodegen::Table;

/// Returns the set of `Procedure`s tables defined in the database.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
///
/// # Errors
///
/// * If the database query fails, returns a `diesel::result::Error`.
pub(crate) fn procedures(_conn: &mut PgConnection) -> Result<Vec<Table>, diesel::result::Error> {
    todo!("Implement procedures function");
}
