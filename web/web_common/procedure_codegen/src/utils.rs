//! Submodule providing utils employed by the procedure codegen.

use diesel::PgConnection;
use webcodegen::{KeyColumnUsage, Table, errors::WebCodeGenError};

pub(crate) const ASSETS_TABLE_NAME: &str = "assets";
pub(crate) const ASSET_MODELS_TABLE_NAME: &str = "asset_models";
pub(crate) const PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME: &str =
    "procedure_template_asset_models";
pub(crate) const PROCEDURE_ASSETS_TABLE_NAME: &str = "procedure_assets";

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
/// * Returns a `WebCodeGenError` if there is an error querying the
///  database.
pub(crate) fn is_asset_foreign_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
    let table = Table::load(conn, ASSETS_TABLE_NAME, &column.table_schema, &column.table_catalog)?;
    column.is_foreign_primary_key_of_table(&table, conn)
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
/// * Returns a `WebCodeGenError` if there is an error querying the
///  database.
pub(crate) fn is_asset_model_foreign_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
    let table =
        Table::load(conn, ASSET_MODELS_TABLE_NAME, &column.table_schema, &column.table_catalog)?;
    column.is_foreign_primary_key_of_table(&table, conn)
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
/// * Returns a `WebCodeGenError` if there is an error querying the
/// database.
pub(crate) fn is_procedure_template_asset_model_foreign_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
    let table = Table::load(
        conn,
        PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
        &column.table_schema,
        &column.table_catalog,
    )?;
    column.is_foreign_primary_key_of_table(&table, conn)
}

/// Returns whether the provided column is a foreign key to the
/// `procedure_assets` table or any table descending from it.
///
///  # Arguments
///
/// * `column` - The column to check.
/// * `conn` - The database connection to use
///
/// # Errors
///
/// * Returns a `WebCodeGenError` if there is an error querying the
/// database.
pub(crate) fn is_procedure_assets_foreign_key(
    column: &webcodegen::Column,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
    let table = Table::load(
        conn,
        PROCEDURE_ASSETS_TABLE_NAME,
        &column.table_schema,
        &column.table_catalog,
    )?;
    column.is_foreign_primary_key_of_table(&table, conn)
}
