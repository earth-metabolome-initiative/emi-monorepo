//! Submodule providing the `ProcedureTemplateDescendantNaming` constraint,
//! which enforces that tables that are descendants of the `procedure_templates`
//! table must have names ending with `procedure_templates`.

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_traits::traits::{DatabaseLike, TableLike};

/// The name of the root procedure templates table.
pub const PROCEDURE_TEMPLATES_TABLE_NAME: &str = "procedure_templates";

/// Struct defining a constraint that enforces that descendants of the
/// `procedure_templates` table must have names ending with
/// `procedure_templates`.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `ProcedureTemplateDescendantNaming` constraint.
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     ProcedureTemplateDescendantNaming::default().into();
///
/// // Invalid: table extends procedure_templates but doesn't end with "procedure_templates"
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE freezing (id INT PRIMARY KEY REFERENCES procedure_templates(id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: table extends procedure_templates and ends with "procedure_templates"
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedure_templates (id INT PRIMARY KEY REFERENCES procedure_templates(id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct ProcedureTemplateDescendantNaming<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for ProcedureTemplateDescendantNaming<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB> From<ProcedureTemplateDescendantNaming<DB>> for Box<dyn TableConstraint<Database = DB>>
where
    DB: DatabaseLike + 'static,
{
    fn from(constraint: ProcedureTemplateDescendantNaming<DB>) -> Self {
        Box::new(constraint)
    }
}

impl<DB: DatabaseLike + 'static> From<ProcedureTemplateDescendantNaming<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: ProcedureTemplateDescendantNaming<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for ProcedureTemplateDescendantNaming<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        _database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("ProcedureTemplateDescendantNaming")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' is a descendant of '{}' but does not end with 'procedure_templates'",
                context.table_name(),
                PROCEDURE_TEMPLATES_TABLE_NAME
            ))
            .unwrap()
            .resolution(format!(
                "Rename the table to end with 'procedure_templates' (e.g., \
                 '{}_procedure_templates')",
                context.table_name().trim_end_matches('s')
            ))
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
        // Skip validation if the procedure_templates table doesn't exist
        let Some(procedure_templates_table) = database.table(None, PROCEDURE_TEMPLATES_TABLE_NAME)
        else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURE_TEMPLATES_TABLE_NAME
            )));
        };

        // Skip if this is the procedure_templates table itself
        if table == procedure_templates_table {
            return Ok(());
        }

        // Check if this table is a descendant of procedure_templates
        if table.is_descendant_of(database, procedure_templates_table) {
            // Must end with "procedure_templates"
            if !table.table_name().ends_with("procedure_templates") {
                return Err(Error::Table(self.table_error_information(database, table)));
            }
        }

        Ok(())
    }
}
