
use crate::custom_schema_constraints::CustomTableConstraint;
use crate::custom_schema_constraints::ConstraintError;
use crate::errors::WebCodeGenError;
use diesel::pg::PgConnection;
use crate::Table;


#[derive(Default)]
/// Constraint to enforce that all column names are lower case.
pub struct LowercaseTableConstraint;

impl CustomTableConstraint for LowercaseTableConstraint {
    fn check_constraint(&self, _conn: &mut PgConnection, table: &Table) -> Result<(), WebCodeGenError> {
        if table.table_name.chars().any(|c| c.is_uppercase()) {
            return Err(ConstraintError::UnexpectedUppercaseTable(table.table_name.clone()).into());
        }
        Ok(())
    }
}