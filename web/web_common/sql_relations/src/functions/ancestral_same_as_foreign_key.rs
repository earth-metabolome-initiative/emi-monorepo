use diesel::PgConnection;
use pg_diesel::models::{KeyColumnUsage, PgIndex};

use crate::{functions::is_same_as_index, traits::Extension};

/// Returns whether this key column usage is an ancestral same-as
/// constraint.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a `PgConnection`
///
/// # Errors
///
/// * If an error occurs while querying the database
pub fn is_ancestral_same_as_foreign_key(
    foreign_key: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Option<PgIndex>, diesel::result::Error> {
    if !foreign_key.includes_local_primary_key(conn)? {
        return Ok(None);
    }

    let foreign_table = foreign_key.foreign_table(conn)?;
    let table = foreign_key.local_table(conn)?;

    if !table.is_extension_of(&foreign_table, conn)? {
        return Ok(None);
    }

    let Some(foreign_unique_constraint) = foreign_key.is_foreign_unique_key(conn)? else {
        return Ok(None);
    };

    Ok(if is_same_as_index(&foreign_unique_constraint, conn)? {
        // If the foreign unique constraint is a same-as constraint, we return it
        Some(foreign_unique_constraint)
    } else {
        // Otherwise, we return None
        None
    })
}
