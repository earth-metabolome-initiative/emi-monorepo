use diesel::PgConnection;

use crate::{
    Table,
    custom_schema_constraints::{ConstraintError, CustomTableConstraint},
    errors::WebCodeGenError,
};

#[derive(Default)]
/// Constraint to detect duplicated check constraints in a table.
pub struct DuplicatedCheckConstraint;

impl CustomTableConstraint for DuplicatedCheckConstraint {
    type Error = crate::errors::WebCodeGenError;

    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError> {
        let check_constraints = table.check_constraints(conn)?;
        let mut clauses = std::collections::HashSet::new();
        for check in &check_constraints {
            if !clauses.insert(&check.check_clause) {
                return Err(ConstraintError::DuplicatedCheckConstraint {
                    table: Box::new(table.clone()),
                    check_constraint: Box::new(check.clone()),
                }
                .into());
            }
        }

        Ok(())
    }
}
