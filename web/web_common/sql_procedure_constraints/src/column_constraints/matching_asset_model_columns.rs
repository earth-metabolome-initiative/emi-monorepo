//! Submodule providing the `MatchingAssetModelColumns` constraint, which
//! enforces that if a procedure template table has a column
//! `procedure_template_{am}` referencing `procedure_template_asset_models`,
//! there must exist a column `{am}` referencing `asset_models`.

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    column_constraints::{
        asset_model_column_naming::ASSET_MODELS_TABLE_NAME,
        procedure_template_asset_model_column_naming::PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
    },
    table_constraints::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// Struct defining a constraint that enforces matching asset model columns in
/// procedure templates.
///
/// # Example
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = MatchingAssetModelColumns::default().into();
///
/// // Invalid: has procedure_template_container_model but no container_model
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE asset_models (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (
///     id INT PRIMARY KEY,
///     asset_model_id INT REFERENCES asset_models(id)
/// );
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: has both procedure_template_container_model and container_model
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE asset_models (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (
///     id INT PRIMARY KEY,
///     asset_model_id INT REFERENCES asset_models(id)
/// );
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id),
///     container_model INT REFERENCES asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct MatchingAssetModelColumns<C>(std::marker::PhantomData<C>);

impl<C> Default for MatchingAssetModelColumns<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<MatchingAssetModelColumns<DB::Column>>
    for GenericConstrainer<DB>
{
    fn from(constraint: MatchingAssetModelColumns<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for MatchingAssetModelColumns<C> {
    type Column = C;

    fn column_error_information(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Box<dyn ConstraintFailureInformation> {
        let table = column.table(database);
        let table_name = table.table_name();
        let column_name = column.column_name();

        let expected_column_name =
            column_name.strip_prefix("procedure_template_").unwrap_or(column_name);

        ConstraintErrorInfo::new()
            .constraint("MatchingAssetModelColumns")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!(
                "Column '{}' in procedure template table '{}' references \
                 procedure_template_asset_models but there is no corresponding column '{}' \
                 referencing asset_models",
                column_name, table_name, expected_column_name
            ))
            .unwrap()
            .resolution(format!(
                "Add a column '{}' to table '{}' that references asset_models",
                expected_column_name, table_name
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

        // If the table is not a procedure template descendant, skip validation.
        if !table.is_descendant_of(database, procedure_templates_table) {
            return Ok(());
        }

        let Some(procedure_template_asset_models_table) =
            database.table(None, PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME)
        else {
            return Ok(());
        };

        // If the column does not reference procedure_template_asset_models, skip
        // validation.
        if !column
            .references_table_pk_or_descendant(database, &procedure_template_asset_models_table)
        {
            return Ok(());
        }

        // The fact that this column should start with "procedure_template_" is
        // already enforced by ProcedureTemplateAssetModelColumnNaming, so we can
        // skip the check if the prefix is not present and avoid redundant errors.
        let Some(expected_column_name) = column.column_name().strip_prefix("procedure_template_")
        else {
            return Ok(());
        };

        // The table must have a column with the expected name that references
        // asset_models.
        let Some(expected_asset_model_column) = table.column(expected_column_name, database) else {
            return Err(Error::Column(self.column_error_information(database, column)));
        };

        let Some(asset_models_table) = database.table(None, ASSET_MODELS_TABLE_NAME) else {
            return Ok(());
        };

        // If the retrieved column does not reference asset_models, return an error.
        if !expected_asset_model_column
            .references_table_pk_or_descendant(database, &asset_models_table)
        {
            return Err(Error::Column(self.column_error_information(database, column)));
        }

        Ok(())
    }
}
