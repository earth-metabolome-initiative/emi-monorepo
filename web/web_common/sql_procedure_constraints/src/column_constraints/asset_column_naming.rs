//! Submodule providing the `AssetColumnNaming` constraint, which enforces that
//! columns in procedure/procedure_template tables that reference `assets`
//! descendants must NOT end with `_model_id` (to avoid confusion with asset
//! models).

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::table_constraints::{PROCEDURE_TEMPLATES_TABLE_NAME, PROCEDURES_TABLE_NAME};

/// The name of the root assets table.
pub const ASSETS_TABLE_NAME: &str = "assets";

/// Struct defining a constraint that enforces that columns in
/// procedure/procedure_template tables referencing assets descendants must NOT
/// end with `_model_id`.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `AssetColumnNaming` constraint.
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = AssetColumnNaming::default().into();
///
/// // Invalid: column references assets but ends with "_model_id" (likely a typo)
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE assets (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (
///     id INT PRIMARY KEY REFERENCES procedures(id),
///     container_model_id INT REFERENCES assets(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: column references assets and does not end with "_model_id"
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE assets (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (
///     id INT PRIMARY KEY REFERENCES procedures(id),
///     container_id INT REFERENCES assets(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct AssetColumnNaming<C>(std::marker::PhantomData<C>);

impl<C> Default for AssetColumnNaming<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<AssetColumnNaming<DB::Column>> for GenericConstrainer<DB> {
    fn from(constraint: AssetColumnNaming<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for AssetColumnNaming<C> {
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
            .constraint("AssetColumnNaming")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!(
                "Column '{}' in procedure table '{}' references an assets descendant but ends \
                 with '_model_id' (likely a typo - should reference asset_models instead)",
                column_name, table_name
            ))
            .unwrap()
            .resolution(format!(
                "Rename column '{}' to remove '_model' suffix (e.g., '{}') OR change the foreign \
                 key to reference an asset_models descendant instead",
                column_name,
                column_name.trim_end_matches("_model_id").to_string() + "_id"
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

        // Check if this table is a descendant of procedures or procedure_templates,
        // otherwise this check constraint does not apply.
        if !(table.is_descendant_of(database, procedures_table)
            || table == procedures_table
            || table.is_descendant_of(database, procedure_templates_table)
            || table == procedure_templates_table)
        {
            return Ok(());
        }

        let Some(assets_table) = database.table(None, ASSETS_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                ASSETS_TABLE_NAME
            )));
        };

        // We determine if the column references an assets descendant.
        if !column.references_table_pk_or_descendant(database, &assets_table) {
            return Ok(());
        }

        // The column must NOT end with "_model_id"
        if column.column_name().ends_with("_model_id") {
            return Err(Error::Column(self.column_error_information(database, column)));
        }

        Ok(())
    }
}
