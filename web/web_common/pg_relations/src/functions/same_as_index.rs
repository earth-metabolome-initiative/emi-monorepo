//! Submodule defining an abstraction for determining if a PostgreSQL index
//! can be used to define a "same-as" relationship between two tables.

use diesel::PgConnection;
use pg_diesel::models::{Column, PgIndex};

pub(crate) fn is_same_as_index(
    index: &PgIndex,
    conn: &mut PgConnection,
) -> Result<bool, diesel::result::Error> {
    // If the index is not unique, it cannot be a same-as index
    if !index.is_unique() {
        return Ok(false);
    }

    // Next, we retrieve the columns associated with the index.
    let columns = index.columns(conn)?;

    // We retrieve the table that this index belongs to.
    let table = index.table(conn)?;

    // We expect that all of the columns in the primary key of the table are also in
    // the index.
    let primary_key_columns = table.primary_key_columns(conn)?;

    // If any of the primary key columns are not in the index, it cannot be a
    // same-as index
    if !primary_key_columns.iter().all(|pk_col| columns.contains(pk_col)) {
        return Ok(false);
    }

    // There needs to be a foreign key constraint which includes all of the
    // remaining columns in the index which refers to some other table's
    // primary key.
    let mut non_primary_key_columns: Vec<Column> =
        columns.iter().filter(|col| !primary_key_columns.contains(col)).cloned().collect();

    if non_primary_key_columns.is_empty() {
        return Ok(true);
    }

    non_primary_key_columns.sort_unstable();

    // We search for a foreign key constraint that includes all of these columns,
    // and that refers to a primary key of another table. If we find any
    // foreign key which satisfies this condition, then we can conclude that
    // the index is a same-as index.

    let foreign_keys_arc = table.foreign_keys(conn)?;
    for foreign_key in foreign_keys_arc.iter() {
        // If the foreign key does not refer to a foreign's table primary key, it cannot
        // be a same-as index.
        if !foreign_key.is_foreign_primary_key(conn)? {
            continue;
        }
        // If the foreign key does not include all of the non-primary key columns, it
        // cannot be a same-as index.
        let mut fk_columns = foreign_key.local_columns(conn)?.clone();
        fk_columns.sort_unstable();

        if non_primary_key_columns == fk_columns {
            return Ok(true);
        }
    }

    Ok(false)
}
