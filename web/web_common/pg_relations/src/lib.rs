#![doc = include_str!("../README.md")]

pub(crate) mod functions;
mod impls;
pub mod traits;

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

/// Returns the columns and foreign keys of the table which require partial
/// builders.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the columns cannot be loaded from the database.
pub fn partial_builder_columns(
    &self,
    conn: &mut PgConnection,
) -> Result<Vec<(Column, PartialBuilderKind, KeyColumnUsage, KeyColumnUsage)>, WebCodeGenError> {
    Ok(self
        .columns(conn)?
        .as_ref()
        .iter()
        .filter_map(|column| {
            let (partial_builder_kind, potential_same_as_constraint, foreign_key) =
                column.requires_partial_builder(conn).ok().flatten()?;
            Some((column.clone(), partial_builder_kind, potential_same_as_constraint, foreign_key))
        })
        .collect())
}

/// Returns the same as indices for the table.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Returns
///
/// A vector of indices.
///
/// # Errors
///
/// * If the indices cannot be loaded from the database.
pub fn same_as_indices(&self, conn: &mut PgConnection) -> Result<Vec<PgIndex>, WebCodeGenError> {
    same_as_indices(self, conn)
}

/// Returns the same as foreign keys for the table.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the foreign keys cannot be loaded from the database.
pub fn same_as_foreign_keys(
    &self,
    conn: &mut PgConnection,
) -> Result<Vec<(KeyColumnUsage, PgIndex)>, WebCodeGenError> {
    let mut same_as_foreign_keys = Vec::new();
    for foreign_key in self.foreign_keys(conn)?.as_ref() {
        if let Some(index) = foreign_key.is_same_as_constraint(conn)? {
            same_as_foreign_keys.push((foreign_key.clone(), index));
        }
    }
    Ok(same_as_foreign_keys)
}

/// Returns the parent keys of the table.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Returns
///
/// The set of foreign key columns that have `ON DELETE CASCADE` constraint.
///
/// # Errors
///
/// * If the foreign keys cannot be loaded from the database.
pub fn parent_keys(&self, conn: &mut PgConnection) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
    let mut parent_keys = Vec::new();
    for foreign_key in self.foreign_keys(conn)?.as_ref() {
        if !foreign_key.has_on_delete_cascade(conn)?
            && foreign_key.is_foreign_primary_key(conn)?
            && !foreign_key.is_self_referential(conn)?
            && foreign_key.is_ancestral_same_as_constraint(conn)?.is_none()
        {
            parent_keys.push(foreign_key.clone());
        }
    }
    Ok(parent_keys)
}

/// Returns the parent tables of the table.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Returns
///
/// A vector of tables that are parents to the current table.
///
/// # Errors
///
/// * If the parent tables cannot be loaded from the database.
pub fn parent_tables(&self, conn: &mut PgConnection) -> Result<Vec<Arc<Table>>, WebCodeGenError> {
    let mut parent_tables = Vec::new();
    for foreign_key in self.parent_keys(conn)? {
        parent_tables.push(foreign_key.foreign_table(conn)?);
    }
    parent_tables.sort_unstable();
    parent_tables.dedup();
    Ok(parent_tables)
}

/// Returns the foreign key columns which point to the current table.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the foreign keys cannot be loaded from the database.
pub fn homogeneous_parent_columns(
    &self,
    conn: &mut PgConnection,
) -> Result<Vec<Arc<Vec<Column>>>, WebCodeGenError> {
    let mut homogeneous_parent_columns = Vec::new();
    for foreign_key in self.foreign_keys(conn)?.as_ref() {
        let foreign_table = foreign_key.foreign_table(conn)?;
        if foreign_table.as_ref() == self && foreign_key.is_foreign_primary_key(conn)? {
            homogeneous_parent_columns.push(foreign_key.columns(conn)?);
        }
    }

    Ok(homogeneous_parent_columns)
}

/// Returns whether the table has parent tables.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Returns
///
/// A boolean indicating whether the table has parent tables.
///
/// # Errors
///
/// * If the foreign keys cannot be loaded from the database.
pub fn has_parents(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
    Ok(!self.parent_keys(conn)?.is_empty())
}

/// Returns whether the table has singleton foreign keys.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the foreign keys cannot be loaded from the database.
pub fn has_singleton_foreign_keys(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
    Ok(self.foreign_keys(conn)?.iter().any(|fk| fk.is_singleton(conn).unwrap_or(false)))
}

/// Returns the table singleton foreign keys.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the foreign keys cannot be loaded from the database.
pub fn singleton_foreign_keys(
    &self,
    conn: &mut PgConnection,
) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
    Ok(self
        .foreign_keys(conn)?
        .iter()
        .filter(|fk| fk.is_singleton(conn).unwrap_or(false))
        .cloned()
        .collect())
}
