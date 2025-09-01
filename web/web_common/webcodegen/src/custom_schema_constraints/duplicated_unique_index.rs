use diesel::PgConnection;

use crate::{
    Table,
    custom_schema_constraints::{ConstraintError, CustomTableConstraint},
    errors::WebCodeGenError,
};

#[derive(Default)]
/// Constraint to enforce that all column names are lower case.
pub struct DuplicatedUniqueIndexConstraint;

impl CustomTableConstraint for DuplicatedUniqueIndexConstraint {
    type Error = crate::errors::WebCodeGenError;

    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError> {
        let unique_indices = table.unique_indices(conn)?;
        let mut clauses = std::collections::HashSet::new();
        for unique_index in &unique_indices {
            let columns = unique_index.columns(conn)?;
            if !clauses.insert(columns.clone()) {
                return Err(ConstraintError::RedundantUniqueIndices {
                    table: Box::new(table.clone()),
                    columns,
                }
                .into());
            }
        }
        Ok(())
    }
}
