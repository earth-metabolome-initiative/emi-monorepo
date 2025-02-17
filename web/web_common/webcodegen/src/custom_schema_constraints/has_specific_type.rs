use diesel::pg::PgConnection;

use super::ConstraintError;
use crate::{custom_schema_constraints::CustomColumnConstraint, errors::WebCodeGenError, Column};

/// Check that a column has a specific type
pub struct HasSpecificTypeConstraint {
    /// The name of the column
    column_name: String,
    /// The expected type of the column
    column_type: String,
}

impl CustomColumnConstraint for HasSpecificTypeConstraint {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        if self.column_name == column.column_name
            && self.column_type != column.data_type_str(conn)?
        {
            return Err(ConstraintError::NotOfCorrectType {
                column_name: self.column_name.clone(),
                column_type: column.data_type_str(conn)?.to_owned(),
                expected_column_type: self.column_type.clone(),
            }
            .into());
        }
        Ok(())
    }
}
