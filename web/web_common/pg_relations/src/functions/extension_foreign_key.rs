//! Submodule defining an abstraction for determining if a PostgreSQL foreign key
//! can be used to define a "extension" relationship between two tables.

use diesel::PgConnection;
use pg_diesel::models::KeyColumnUsage;

/// Returns whether the given foreign key constraint defines an extension relationship,
/// i.e., it has the same primary key as another table and possibly additional columns,
/// which is defined by a primary key which also is a foreign key to another table's primary key.
///
/// # Arguments
///
/// * `foreign_key` - The foreign key constraint to check.
/// * `conn` - The database connection.
/// 
/// # Errors
/// 
/// * If the foreign key constraint cannot be analyzed.
pub(crate) fn is_extension_foreign_key(
    foreign_key: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<bool, diesel::result::Error> {
    Ok(foreign_key.is_local_primary_key(conn)?
        && foreign_key.is_foreign_primary_key(conn)?
        && !foreign_key.is_self_referential(conn)?)
}
