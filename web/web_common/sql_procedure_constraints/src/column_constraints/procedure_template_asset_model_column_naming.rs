//! Submodule providing the `ProcedureTemplateAssetModelColumnNaming`
//! constraint, which enforces that columns in procedure_template tables that
//! reference `procedure_template_asset_models` must start with
//! `procedure_template_`.

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::table_constraints::PROCEDURE_TEMPLATES_TABLE_NAME;

/// The name of the procedure_template_asset_models table.
pub const PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME: &str = "procedure_template_asset_models";

/// Struct defining a constraint that enforces that columns in
/// procedure_template tables referencing procedure_template_asset_models must
/// start with `procedure_template_`.
///
/// # Example
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     ProcedureTemplateAssetModelColumnNaming::default().into();
///
/// // Invalid: column references procedure_template_asset_models but doesn't start with
/// // "procedure_template_"
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     asset_model_id INT REFERENCES procedure_template_asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: column references procedure_template_asset_models and starts with
/// // "procedure_template_"
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_asset_model_id INT REFERENCES procedure_template_asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct ProcedureTemplateAssetModelColumnNaming<C>(std::marker::PhantomData<C>);

impl<C> Default for ProcedureTemplateAssetModelColumnNaming<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<ProcedureTemplateAssetModelColumnNaming<DB::Column>>
    for GenericConstrainer<DB>
{
    fn from(constraint: ProcedureTemplateAssetModelColumnNaming<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for ProcedureTemplateAssetModelColumnNaming<C> {
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
            .constraint("ProcedureTemplateAssetModelColumnNaming")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!(
                "Column '{}' in procedure template table '{}' references \
                 procedure_template_asset_models but does not start with 'procedure_template_'",
                column_name, table_name
            ))
            .unwrap()
            .resolution(format!(
                "Rename column '{}' to start with 'procedure_template_' (e.g., \
                 'procedure_template_{}')",
                column_name, column_name
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

        // Skip validation if the procedure_templates table doesn't exist
        let Some(procedure_templates_table) = database.table(None, PROCEDURE_TEMPLATES_TABLE_NAME)
        else {
            return Ok(());
        };

        // Skip validation if the procedure_template_asset_models table doesn't exist
        let Some(procedure_template_asset_models_table) =
            database.table(None, PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME)
        else {
            return Ok(());
        };

        // Check if this table is a descendant of procedure_templates
        let is_procedure_template_descendant = table
            .is_descendant_of(database, procedure_templates_table)
            || table == procedure_templates_table;

        if !is_procedure_template_descendant {
            return Ok(());
        }

        // Get the tables referenced by this column via foreign keys
        let referenced_tables = table.referenced_tables_via_column(database, column.borrow());

        // Check if any of the referenced tables are procedure_template_asset_models
        for referenced_table in &referenced_tables {
            if *referenced_table == procedure_template_asset_models_table {
                // Column references procedure_template_asset_models - must start with
                // procedure_template_
                if !column.column_name().starts_with("procedure_template_") {
                    return Err(Error::Column(self.column_error_information(database, column)));
                }
            }
        }

        Ok(())
    }
}
