use crate::custom_schema_constraints::CustomColumnConstraint;
use crate::errors::WebCodeGenError;
use crate::Column;
use diesel::pg::PgConnection;

use super::ConstraintError;

pub struct HasSpecificTypeConstraint {
    column_name: String,
    column_type: String,
}

impl CustomColumnConstraint for HasSpecificTypeConstraint {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        if self.column_name == column.column_name && self.column_type != column.data_type_str(conn)? {
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
