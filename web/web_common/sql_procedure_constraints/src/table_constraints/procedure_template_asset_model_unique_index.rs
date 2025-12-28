//! Submodule providing the `ProcedureTemplateAssetModelUniqueIndex` constraint,
//! which enforces that for each column `procedure_template_{am}` in a procedure
//! template table, there must exist a UNIQUE index on (id,
//! procedure_template_{am}).

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_traits::traits::{DatabaseLike, TableLike, UniqueIndexLike};

use crate::{
    column_constraints::PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
    table_constraints::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// Struct defining a constraint that enforces unique indexes for procedure
/// template asset model columns.
///
/// # Example
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     ProcedureTemplateAssetModelUniqueIndex::default().into();
///
/// // Invalid: has procedure_template_container_model but no UNIQUE (id, procedure_template_container_model)
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: has UNIQUE (id, procedure_template_container_model)
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id),
///     UNIQUE (id, procedure_template_container_model)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct ProcedureTemplateAssetModelUniqueIndex<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for ProcedureTemplateAssetModelUniqueIndex<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<ProcedureTemplateAssetModelUniqueIndex<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: ProcedureTemplateAssetModelUniqueIndex<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for ProcedureTemplateAssetModelUniqueIndex<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn ConstraintFailureInformation> {
        let table_name = context.table_name();

        // Find the procedure_template_{am} columns that are missing unique indexes
        let Some(procedure_template_asset_models_table) =
            database.table(None, PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME)
        else {
            return ConstraintErrorInfo::new()
                .constraint("ProcedureTemplateAssetModelUniqueIndex")
                .unwrap()
                .object(table_name.to_owned())
                .unwrap()
                .message("procedure_template_asset_models table not found".to_string())
                .unwrap()
                .build()
                .unwrap()
                .into();
        };

        let missing_indexes: Vec<String> = context
            .columns(database)
            .filter(|col| {
                col.column_name().starts_with("procedure_template_")
                    && context
                        .referenced_tables_via_column(database, col)
                        .contains(&procedure_template_asset_models_table)
            })
            .filter(|col| {
                // Check if there's a unique index on (id, procedure_template_{am})
                !context.unique_indices(database).any(|idx| {
                    let idx_columns: Vec<_> = idx.columns(database).collect();
                    idx_columns.len() == 2
                        && idx_columns.iter().any(|&c| c.column_name() == "id")
                        && idx_columns.iter().any(|&c| c.column_name() == col.column_name())
                })
            })
            .map(|col| col.column_name().to_string())
            .collect();

        let missing_column = missing_indexes.first().cloned().unwrap_or_default();

        ConstraintErrorInfo::new()
            .constraint("ProcedureTemplateAssetModelUniqueIndex")
            .unwrap()
            .object(table_name.to_owned())
            .unwrap()
            .message(format!(
                "Procedure template table '{}' has column(s) referencing \
                 procedure_template_asset_models but missing required UNIQUE index on (id, {})",
                table_name, missing_column
            ))
            .unwrap()
            .resolution(format!("Add unique index: UNIQUE (id, {})", missing_column))
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), Error> {
        // Skip validation if required tables don't exist
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

        // Check if this table is a descendant of procedure_templates
        if !table.is_descendant_of(database, procedure_templates_table)
            && table != procedure_templates_table
        {
            return Ok(());
        }

        // Find all columns that reference procedure_template_asset_models
        let procedure_template_asset_model_columns: Vec<_> = table
            .columns(database)
            .filter(|col| {
                col.column_name().starts_with("procedure_template_")
                    && table
                        .referenced_tables_via_column(database, col)
                        .contains(&procedure_template_asset_models_table)
            })
            .collect();

        // For each such column, verify there's a UNIQUE (id, procedure_template_{am})
        // index
        for column in procedure_template_asset_model_columns {
            let has_unique_index = table.unique_indices(database).any(|idx| {
                let idx_columns: Vec<_> = idx.columns(database).collect();
                idx_columns.len() == 2
                    && idx_columns.iter().any(|&c| c.column_name() == "id")
                    && idx_columns.iter().any(|&c| c.column_name() == column.column_name())
            });

            if !has_unique_index {
                return Err(Error::Table(self.table_error_information(database, table)));
            }
        }

        Ok(())
    }
}
