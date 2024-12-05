use crate::custom_schema_constraints::CustomColumnConstraint;
use crate::errors::WebCodeGenError;
use crate::Column;
use diesel::pg::PgConnection;

use super::ConstraintError;

#[derive(Debug)]
pub struct IsForeignKey;

impl CustomColumnConstraint for IsForeignKey {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        // Constraint to check that the column is a foreign key of of the users table
        let table_name = column.table_name.clone();
        if table_name != "users" && !column.is_foreign_key(conn) {
            return Err(WebCodeGenError::ConstraintError(
                ConstraintError::NotForeignKeyColumn(column.column_name.clone()),
            ));
        }
        Ok(())
    }
}
