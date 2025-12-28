//! Submodule providing the `ProcedureDescendantNaming` constraint, which
//! enforces that tables that are descendants of the `procedures` table must
//! have names ending with `procedures`.

use common_traits::builder::Builder;
use sql_constraints::{error::ConstraintErrorInfo, prelude::*};
use sql_traits::traits::{DatabaseLike, TableLike};

/// The name of the root procedures table.
pub const PROCEDURES_TABLE_NAME: &str = "procedures";

/// Struct defining a constraint that enforces that descendants of the
/// `procedures` table must have names ending with `procedures`.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `ProcedureDescendantNaming` constraint.
///
/// ```rust
/// use sql_procedure_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = ProcedureDescendantNaming::default().into();
///
/// // Invalid: table extends procedures but doesn't end with "procedures"
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE freezing (id INT PRIMARY KEY REFERENCES procedures(id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: table extends procedures and ends with "procedures"
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (id INT PRIMARY KEY REFERENCES procedures(id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct ProcedureDescendantNaming<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for ProcedureDescendantNaming<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB> From<ProcedureDescendantNaming<DB>> for Box<dyn TableConstraint<Database = DB>>
where
    DB: DatabaseLike + 'static,
{
    fn from(constraint: ProcedureDescendantNaming<DB>) -> Self {
        Box::new(constraint)
    }
}

impl<DB: DatabaseLike + 'static> From<ProcedureDescendantNaming<DB>> for GenericConstrainer<DB> {
    fn from(constraint: ProcedureDescendantNaming<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for ProcedureDescendantNaming<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        _database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("ProcedureDescendantNaming")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' is a descendant of '{}' but does not end with 'procedures'",
                context.table_name(),
                PROCEDURES_TABLE_NAME
            ))
            .unwrap()
            .resolution(format!(
                "Rename the table to end with 'procedures' (e.g., '{}_procedures')",
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
        let Some(procedures_table) = database.table(None, PROCEDURES_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURES_TABLE_NAME
            )));
        };

        // Skip if this is the procedures table itself
        if table == procedures_table {
            return Ok(());
        }

        // Check if this table is a descendant of procedures
        if table.is_descendant_of(database, procedures_table) {
            // Must end with "procedures"
            if !table.table_name().ends_with("procedures") {
                return Err(Error::Table(self.table_error_information(database, table)));
            }
        }

        Ok(())
    }
}
