//! Submodule providing trait definitions describing abstractions over
//! PostgreSQL relations.

pub(super) fn same_as_indices(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<PgIndex>, crate::error::Error> {
    Ok(unique_indices(table, conn)?
        .into_iter()
        .filter(|pg_index| pg_index.is_same_as(conn).unwrap_or(false))
        .collect())
}

/// Returns the subset of the table's columns which define other
/// columns' values via foreign key constraints.
///
/// # Arguments
///
/// * `include_mandatory_partial_builders` - Whether to include columns that
///   require a partial builder.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the columns cannot be loaded from the database.
pub(crate) fn foreign_definer_columns(
    &self,
    include_mandatory_partial_builders: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, WebCodeGenError> {
    let mut foreign_definer_columns = Vec::new();
    for column in self.columns(conn)?.as_ref() {
        if column.is_foreign_definer(include_mandatory_partial_builders, conn)? {
            foreign_definer_columns.push(column.clone());
        }
    }
    Ok(foreign_definer_columns)
}
