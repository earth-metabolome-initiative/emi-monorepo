use diesel::PgConnection;

use crate::{
    Table,
    custom_schema_constraints::{ConstraintError, CustomTableConstraint},
    errors::WebCodeGenError,
};

#[derive(Default)]
/// Constraint to enforce that all column names are lower case.
pub struct SameAsConstraintMustNotCascade;

impl CustomTableConstraint for SameAsConstraintMustNotCascade {
    type Error = crate::errors::WebCodeGenError;

    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError> {
        // We load all of the same_as_constraints for the table
        for (same_as_constraint, _) in &table.same_as_foreign_keys(conn)? {
            if same_as_constraint.has_on_delete_cascade(conn)? {
                return Err(WebCodeGenError::ConstraintError(
                    ConstraintError::SameAsConstraintMustNotCascade(Box::new(
                        same_as_constraint.clone(),
                    )),
                ));
            }
        }

        Ok(())
    }
}
