//! Submodule providing the `AssetModelColumnNaming` constraint, which enforces
//! that columns in procedure/procedure_template tables that reference
//! `asset_models` descendants must end with `_model_id`.

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::table_constraints::{PROCEDURE_TEMPLATES_TABLE_NAME, PROCEDURES_TABLE_NAME};

/// The name of the root asset_models table.
pub const ASSET_MODELS_TABLE_NAME: &str = "asset_models";

/// Struct defining a constraint that enforces that columns in
/// procedure/procedure_template tables referencing asset_models descendants
/// must end with `_model_id`.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `AssetModelColumnNaming` constraint.
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = AssetModelColumnNaming::default().into();
///
/// // Invalid: column references asset_models but doesn't end with "_model_id"
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE asset_models (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (
///     id INT PRIMARY KEY REFERENCES procedures(id),
///     asset_id INT REFERENCES asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: column references asset_models and ends with "_model_id"
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE asset_models (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (
///     id INT PRIMARY KEY REFERENCES procedures(id),
///     asset_model_id INT REFERENCES asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct AssetModelColumnNaming<C>(std::marker::PhantomData<C>);

impl<C> Default for AssetModelColumnNaming<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<AssetModelColumnNaming<DB::Column>>
    for GenericConstrainer<DB>
{
    fn from(constraint: AssetModelColumnNaming<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for AssetModelColumnNaming<C> {
    type Column = C;

    fn column_error_information(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Box<dyn ConstraintFailureInformation> {
        let table = column.table(database);
        let table_name = table.table_name();
        let column_name = column.column_name();

        ConstraintErrorInfo::new()
            .constraint("AssetModelColumnNaming")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!(
                "Column '{}' in procedure table '{}' references an asset_models descendant but \
                 does not end with '_model_id'",
                column_name, table_name
            ))
            .unwrap()
            .resolution(format!(
                "Rename column '{}' to end with '_model_id' (e.g., '{}_model_id')",
                column_name,
                column_name.trim_end_matches("_id")
            ))
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_column(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Result<(), Error> {
        let table = column.table(database);

        // Skip validation if the procedures or procedure_templates tables don't exist
        let Some(procedures_table) = database.table(None, PROCEDURES_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURES_TABLE_NAME
            )));
        };
        let Some(procedure_templates_table) = database.table(None, PROCEDURE_TEMPLATES_TABLE_NAME)
        else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURE_TEMPLATES_TABLE_NAME
            )));
        };

        // Skip validation if the asset_models table doesn't exist
        let Some(asset_models_table) = database.table(None, ASSET_MODELS_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                ASSET_MODELS_TABLE_NAME
            )));
        };

        // Check if this table is a descendant of procedures or procedure_templates
        let is_procedure_descendant = table.is_descendant_of(database, procedures_table)
            || table == procedures_table
            || table.is_descendant_of(database, procedure_templates_table)
            || table == procedure_templates_table;

        if !is_procedure_descendant {
            return Ok(());
        }

        // Get the tables referenced by this column via foreign keys
        let referenced_tables = table.referenced_tables_via_column(database, column.borrow());

        // Check if any of the referenced tables are descendants of asset_models
        for referenced_table in &referenced_tables {
            if *referenced_table == asset_models_table
                || referenced_table.is_descendant_of(database, asset_models_table)
            {
                // Column references asset_models or its descendant - must end with _model_id
                if !column.column_name().ends_with("_model_id") {
                    return Err(Error::Column(self.column_error_information(database, column)));
                }
            }
        }

        Ok(())
    }
}
