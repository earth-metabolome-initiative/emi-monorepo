//! Submodule providing the `MatchingProcedureAndTemplateTables` constraint,
//! which enforces that for every `{t}_procedures` table, there must exist a
//! corresponding `{t}_procedure_templates` table.

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_traits::traits::{DatabaseLike, TableLike};

use super::{
    procedure_descendant_naming::PROCEDURES_TABLE_NAME,
    procedure_template_descendant_naming::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// Struct defining a constraint that enforces that for every `{t}_procedures`
/// table, there must exist a corresponding `{t}_procedure_templates` table.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `MatchingProcedureAndTemplateTables` constraint.
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     MatchingProcedureAndTemplateTables::default().into();
///
/// // Invalid: freezing_procedures exists but freezing_procedure_templates doesn't
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (id INT PRIMARY KEY REFERENCES procedures(id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: both freezing_procedures and freezing_procedure_templates exist
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (id INT PRIMARY KEY REFERENCES procedures(id));
/// CREATE TABLE freezing_procedure_templates (id INT PRIMARY KEY REFERENCES procedure_templates(id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// // Valid: the base procedures and procedure_templates tables don't need matches
/// let valid_base_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_base_schema).is_ok());
/// ```
pub struct MatchingProcedureAndTemplateTables<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for MatchingProcedureAndTemplateTables<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB> From<MatchingProcedureAndTemplateTables<DB>> for Box<dyn TableConstraint<Database = DB>>
where
    DB: DatabaseLike + 'static,
{
    fn from(constraint: MatchingProcedureAndTemplateTables<DB>) -> Self {
        Box::new(constraint)
    }
}

impl<DB: DatabaseLike + 'static> From<MatchingProcedureAndTemplateTables<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: MatchingProcedureAndTemplateTables<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for MatchingProcedureAndTemplateTables<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        _database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn ConstraintFailureInformation> {
        let table_name = context.table_name();

        // Determine the expected matching table name
        let (missing_table, description) =
            if let Some(prefix) = table_name.strip_suffix("_procedures") {
                (
                    format!("{}_procedure_templates", prefix),
                    "procedures table without corresponding procedure_templates table",
                )
            } else if let Some(prefix) = table_name.strip_suffix("_procedure_templates") {
                (
                    format!("{}_procedures", prefix),
                    "procedure_templates table without corresponding procedures table",
                )
            } else {
                // This shouldn't happen in practice since we filter for these suffixes
                (String::new(), "table with unexpected naming")
            };

        ConstraintErrorInfo::new()
            .constraint("MatchingProcedureAndTemplateTables")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' is a {} - expected table '{}' to exist",
                table_name, description, missing_table
            ))
            .unwrap()
            .resolution(format!(
                "Create the matching table '{}' that corresponds to '{}'",
                missing_table, table_name
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
        let Some(procedure_templates) = database.table(None, PROCEDURE_TEMPLATES_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURE_TEMPLATES_TABLE_NAME
            )));
        };
        let Some(procedures) = database.table(None, PROCEDURES_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURES_TABLE_NAME
            )));
        };

        if table == procedures || table == procedure_templates {
            return Ok(());
        }

        if !table.is_descendant_of(database, procedure_templates)
            && !table.is_descendant_of(database, procedures)
        {
            return Ok(());
        }

        let table_name = table.table_name();

        if let Some(prefix) = table_name.strip_suffix("_procedures") {
            let expected_template_table_name = format!("{}_procedure_templates", prefix);
            if database.table(None, &expected_template_table_name).is_none() {
                return Err(Error::Table(self.table_error_information(database, table)));
            }
        } else if let Some(prefix) = table_name.strip_suffix("_procedure_templates") {
            let expected_procedure_table_name = format!("{}_procedures", prefix);
            if database.table(None, &expected_procedure_table_name).is_none() {
                return Err(Error::Table(self.table_error_information(database, table)));
            }
        }

        Ok(())
    }
}
