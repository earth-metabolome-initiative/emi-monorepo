//! Submodule providing the `HorizontalAssetModelForeignKey` constraint, which
//! enforces that for columns `procedure_template_{am}` referencing
//! `procedure_template_asset_models`, there must exist a horizontal foreign key
//! from (procedure_template_{am}, {am}) to
//! procedure_template_asset_models(id, asset_model_id).

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_relations::prelude::*;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    column_constraints::{
        asset_model_column_naming::ASSET_MODELS_TABLE_NAME,
        procedure_template_asset_model_column_naming::PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
    },
    table_constraints::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// Struct defining a constraint that enforces horizontal foreign keys for
/// asset model columns in procedure templates.
///
/// # Example
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     HorizontalAssetModelForeignKey::default().into();
///
/// // Invalid: has procedure_template_container_model and container_model but no horizontal FK
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE asset_models (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (
///     id INT PRIMARY KEY,
///     asset_model_id INT REFERENCES asset_models(id),
///     UNIQUE(id, asset_model_id)
/// );
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id),
///     container_model INT REFERENCES asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: has horizontal FK from (procedure_template_container_model, container_model)
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE asset_models (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (
///     id INT PRIMARY KEY,
///     asset_model_id INT REFERENCES asset_models(id),
///     UNIQUE(id, asset_model_id)
/// );
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id),
///     container_model INT REFERENCES asset_models(id),
///     FOREIGN KEY (procedure_template_container_model, container_model)
///         REFERENCES procedure_template_asset_models(id, asset_model_id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct HorizontalAssetModelForeignKey<C>(std::marker::PhantomData<C>);

impl<C> Default for HorizontalAssetModelForeignKey<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<HorizontalAssetModelForeignKey<DB::Column>>
    for GenericConstrainer<DB>
{
    fn from(constraint: HorizontalAssetModelForeignKey<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for HorizontalAssetModelForeignKey<C> {
    type Column = C;

    fn column_error_information(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Box<dyn ConstraintFailureInformation> {
        let table = column.table(database);
        let table_name = table.table_name();
        let column_name = column.column_name();

        let suffix = column_name.strip_prefix("procedure_template_").unwrap_or(column_name);

        ConstraintErrorInfo::new()
            .constraint("HorizontalAssetModelForeignKey")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!(
                "Column '{}' in procedure template table '{}' references \
                 procedure_template_asset_models but there is no horizontal foreign key from \
                 ({}, {}) to procedure_template_asset_models(id, asset_model_id)",
                column_name, table_name, column_name, suffix
            ))
            .unwrap()
            .resolution(format!(
                "Add a foreign key: FOREIGN KEY ({}, {}) REFERENCES \
                 procedure_template_asset_models(id, asset_model_id)",
                column_name, suffix
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

        let Some(procedure_templates_table) = database.table(None, PROCEDURE_TEMPLATES_TABLE_NAME)
        else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURE_TEMPLATES_TABLE_NAME
            )));
        };
        let Some(procedure_template_asset_models_table) =
            database.table(None, PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME)
        else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME
            )));
        };
        let Some(asset_models_table) = database.table(None, ASSET_MODELS_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                ASSET_MODELS_TABLE_NAME
            )));
        };

        // Check if this table is a descendant of procedure_templates
        if !table.is_descendant_of(database, procedure_templates_table)
            && table != procedure_templates_table
        {
            return Ok(());
        }

        // Get the tables referenced by this column via foreign keys
        let referenced_tables = table.referenced_tables_via_column(database, column.borrow());

        // Check if this column references procedure_template_asset_models
        let references_procedure_template_asset_models =
            referenced_tables.iter().any(|&t| t == procedure_template_asset_models_table);

        if !references_procedure_template_asset_models {
            return Ok(());
        }

        // Column must start with "procedure_template_" (enforced by another constraint)
        let Some(suffix) = column.column_name().strip_prefix("procedure_template_") else {
            return Ok(());
        };

        // Find the matching asset model column
        let Some(asset_model_column) = table.columns(database).find(|col| {
            col.column_name() == suffix
                && table.referenced_tables_via_column(database, col).contains(&asset_models_table)
        }) else {
            // Matching column doesn't exist - handled by another constraint
            return Ok(());
        };

        // Check if there exists a horizontal foreign key from
        // (procedure_template_{am}, {am}) to procedure_template_asset_models(id,
        // asset_model_id)
        let has_horizontal_fk = table.foreign_keys(database).any(|fk| {
            if !fk.is_horizontal_same_as(database) {
                return false;
            }

            // Check if this FK references procedure_template_asset_models
            if fk.referenced_table(database) != procedure_template_asset_models_table {
                return false;
            }

            // Check if the FK involves both our columns
            let host_columns: Vec<_> = fk.host_columns(database).collect();
            let has_procedure_template_column =
                host_columns.iter().any(|&col| col == column.borrow());
            let has_asset_model_column = host_columns.iter().any(|&col| col == asset_model_column);

            has_procedure_template_column && has_asset_model_column && host_columns.len() == 2
        });

        if !has_horizontal_fk {
            return Err(Error::Column(self.column_error_information(database, column)));
        }

        Ok(())
    }
}
