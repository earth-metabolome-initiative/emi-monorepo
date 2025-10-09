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


/// Returns the ancestral same-as foreign keys of the table, if any.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the table cannot be loaded from the database.
pub fn ancestral_same_as_foreign_keys(
    &self,
    conn: &mut PgConnection,
) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
    let mut ancestral_foreign_key = Vec::new();
    for foreign_key in self.foreign_keys(conn)?.as_ref() {
        if foreign_key.is_ancestral_same_as_constraint(conn)?.is_some() {
            ancestral_foreign_key.push(foreign_key.clone());
        }
    }

    ancestral_foreign_key.sort_unstable();
    ancestral_foreign_key.dedup();

    Ok(ancestral_foreign_key)
}
