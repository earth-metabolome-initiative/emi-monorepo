use diesel::PgConnection;
use pg_diesel::models::{KeyColumnUsage, PgIndex};

use crate::{functions::is_same_as_index, traits::Extension};

// TODO! SAME AS MUST HAVE ON UPDATE CASCADE!

/// Returns whether this key column usage is an associated same-as
/// constraint.
///
/// # Arguments
///
/// * `include_local_primary_key` - Whether to include the local primary key in
///   the constraint
/// * `conn` - A mutable reference to a `PgConnection`
///
/// # Errors
///
/// * If an error occurs while querying the database
pub fn is_associated_same_as_foreign_key(
    foreign_key: &KeyColumnUsage,
    include_local_primary_key: bool,
    conn: &mut PgConnection,
) -> Result<Option<PgIndex>, diesel::result::Error> {
    if !include_local_primary_key && foreign_key.includes_local_primary_key(conn)? {
        return Ok(None);
    }
    if !foreign_key
        .local_columns(conn)?
        .iter()
        .any(|c| c.requires_partial_builder(conn).ok().flatten().is_some())
    {
        return Ok(None);
    }

    let foreign_table = foreign_key.foreign_table(conn)?;

    let table = foreign_key.local_table(conn)?;

    if table.is_extension_of(&foreign_table, conn)? {
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


/// Returns the associated same-as foreign keys of the table, if any.
///
/// # Arguments
///
/// * `include_local_primary_key` - Whether to include the local primary key in
///   the constraint.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the table cannot be loaded from the database.
pub fn associated_same_as_foreign_keys(
    &self,
    include_local_primary_key: bool,
    conn: &mut PgConnection,
) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
    let mut associated_foreign_key = Vec::new();
    for foreign_key in self.foreign_keys(conn)?.as_ref() {
        if foreign_key.is_associated_same_as_constraint(include_local_primary_key, conn)?.is_some()
        {
            associated_foreign_key.push(foreign_key.clone());
        }
    }

    Ok(associated_foreign_key)
}


/// Returns the associated tables this table references via foreign keys, if
/// any.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Implementative details
///
/// A table referenced by another table is considered associated if any
/// columns of the latter table referencing the former table are not
/// part of the primary key of the latter table, and still require a
/// partial builder.
///
/// # Errors
///
/// * If the table cannot be loaded from the database.
pub(crate) fn associated_tables(
    &self,
    include_local_primary_key: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Arc<Table>>, WebCodeGenError> {
    let mut associated_tables = Vec::new();
    for foreign_key in self.associated_same_as_foreign_keys(include_local_primary_key, conn)? {
        let foreign_table = foreign_key.foreign_table(conn)?;
        associated_tables.push(foreign_table);
    }

    associated_tables.sort_unstable();
    associated_tables.dedup();

    Ok(associated_tables)
}
