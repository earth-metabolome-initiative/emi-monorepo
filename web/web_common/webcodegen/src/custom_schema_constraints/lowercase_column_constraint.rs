
use crate::custom_schema_constraints::CustomColumnConstraint;
use crate::custom_schema_constraints::ConstraintError;
use crate::errors::WebCodeGenError;
use diesel::pg::PgConnection;
use crate::Column;


#[derive(Default)]
/// Constraint to enforce that all column names are lower case.
pub struct LowercaseColumnConstraint;

impl CustomColumnConstraint for LowercaseColumnConstraint {
    fn check_constraint(&self, _conn: &mut PgConnection, column: &Column) -> Result<(), WebCodeGenError> {
        if column.column_name.chars().any(|c| c.is_uppercase()) {
            return Err(ConstraintError::UnexpectedUppercaseColumn(column.column_name.clone()).into());
        }
        Ok(())
    }
}