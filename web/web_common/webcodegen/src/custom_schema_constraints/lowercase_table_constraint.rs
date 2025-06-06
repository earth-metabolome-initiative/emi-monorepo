use diesel::PgConnection;

use crate::{
    Table,
    custom_schema_constraints::{ConstraintError, CustomTableConstraint},
    errors::WebCodeGenError,
};

#[derive(Default)]
/// Constraint to enforce that all column names are lower case.
pub struct LowercaseTableConstraint;

impl CustomTableConstraint for LowercaseTableConstraint {
    fn check_constraint(
        &self,
        _conn: &mut PgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError> {
        if table.table_name.chars().any(char::is_uppercase) {
            return Err(ConstraintError::UnexpectedUppercaseTable(table.table_name.clone()).into());
        }
        Ok(())
    }
}
