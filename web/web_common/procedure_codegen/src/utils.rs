//! Submodule providing utils employed by the procedure codegen.

use diesel::PgConnection;
use webcodegen::KeyColumnUsage;

pub(crate) const ASSETS_TABLE_NAME: &str = "assets";
pub(crate) const ASSET_MODELS_TABLE_NAME: &str = "asset_models";
pub(crate) const PROCEDURE_ASSETS_TABLE_NAME: &str = "procedure_assets";
pub(crate) const PROCEDURE_TEMPLATE_ASSETS_TABLE_NAME: &str = "procedure_template_asset_models";

/// Returns whether the provided column is a foreign key to the specified table
/// or any table descending from it.
///
/// # Arguments
///
/// * `column` - The column to check.
/// * `conn` - The database connection to use.
/// * `table_name` - The name of the table to check against.
///
/// # Errors
///
/// * Returns a `diesel::result::Error` if there is an error querying the
///   database.
fn is_foreign_primary_key_to_table(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
    table_name: &str,
) -> Result<Option<KeyColumnUsage>, diesel::result::Error> {
    for foreign_key in column.foreign_keys(conn)? {
        if !foreign_key.is_foreign_primary_key(conn)? {
            continue;
        }
        let number_of_columns = foreign_key.columns(conn)?.len();
        if number_of_columns != 1 {
            continue;
        }
        if let Some(foreign_table) = foreign_key.foreign_table(conn)? {
            if foreign_table.table_name == table_name {
                return Ok(Some(foreign_key));
            }

            let extended_tables = foreign_table.ancestral_extension_tables(conn)?;

            if extended_tables.iter().any(|t| t.table_name == table_name) {
                return Ok(Some(foreign_key));
            }
        }
    }
    Ok(None)
}

/// Returns whether the provided column is a foreign key to the `assets` table
/// or any table descending from it.
///
/// # Arguments
///
/// * `column` - The column to check.
/// * `conn` - The database connection to use.
///
/// # Errors
///
/// * Returns a `diesel::result::Error` if there is an error querying the
///  database.
pub(crate) fn is_asset_foreign_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, diesel::result::Error> {
    is_foreign_primary_key_to_table(column, conn, ASSETS_TABLE_NAME)
}

/// Returns whether the provided column is a foreign key to the `asset_models`
/// table or any table descending from it.
///
/// # Arguments
///
/// * `column` - The column to check.
/// * `conn` - The database connection to use
///
/// # Errors
///
/// * Returns a `diesel::result::Error` if there is an error querying the
///  database.
pub(crate) fn is_asset_model_foreign_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, diesel::result::Error> {
    is_foreign_primary_key_to_table(column, conn, ASSET_MODELS_TABLE_NAME)
}

/// Returns whether the provided column is a foreign key to the
/// `procedure_assets` table or any table descending from it.
///
/// # Arguments
///
/// * `column` - The column to check.
/// * `conn` - The database connection to use
///
///  # Errors
///
/// * Returns a `diesel::result::Error` if there is an error querying the
///  database.
pub(crate) fn is_procedure_asset_foreign_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, diesel::result::Error> {
    is_foreign_primary_key_to_table(column, conn, PROCEDURE_ASSETS_TABLE_NAME)
}

/// Returns whether the provided column is a foreign key to the
/// `procedure_template_asset_models` table or any table descending from it.
///
///  # Arguments
///
/// * `column` - The column to check.
/// * `conn` - The database connection to use
///
/// # Errors
///
/// * Returns a `diesel::result::Error` if there is an error querying the
/// database.
pub(crate) fn is_procedure_template_asset_model_foreign_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, diesel::result::Error> {
    is_foreign_primary_key_to_table(column, conn, PROCEDURE_TEMPLATE_ASSETS_TABLE_NAME)
}

/// Returns whether the provided column appears in a same-as constraint in its
/// own table with the primary key of the table.
///
/// # Arguments
///
/// * `column` - The column to check.
/// * `conn` - The database connection to use.
///
/// # Errors
///
/// * Returns a `diesel::result::Error` if there is an error querying the
///   database.
pub(crate) fn is_in_same_as_with_primary_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<bool, crate::errors::Error> {
    let table = column.table(conn)?;
    let primary_key_columns = table.primary_key_columns(conn)?;

    for same_as_foreign_key in column.same_as_constraints(conn)? {
        let same_as_columns = same_as_foreign_key.columns(conn)?;
        if primary_key_columns.iter().all(|pk_col| same_as_columns.contains(pk_col)) {
            return Ok(true);
        }
    }

    Ok(false)
}
