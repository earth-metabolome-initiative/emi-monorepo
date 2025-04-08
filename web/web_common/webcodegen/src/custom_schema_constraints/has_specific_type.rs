use diesel::pg::PgConnection;

use super::ConstraintError;
use crate::{custom_schema_constraints::CustomColumnConstraint, errors::WebCodeGenError, Column};

/// Check that a column has a specific type
pub struct HasSpecificTypeConstraint<'column> {
    /// The name of the column
    column_name: &'column str,
    /// The expected type of the column
    column_type: &'column str,
}

impl<'column> HasSpecificTypeConstraint<'column> {
    /// Create a new `HasSpecificTypeConstraint`
    pub fn new(column_name: &'column str, column_type: &'column str) -> Self {
        Self { column_name, column_type }
    }
}

impl<'column> CustomColumnConstraint for HasSpecificTypeConstraint<'column> {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        if self.column_name == column.column_name
            && self.column_type != column.data_type_str(conn)?
        {
            return Err(ConstraintError::NotOfCorrectType {
                column: Box::from(column.clone()),
                expected_column_type: self.column_type.to_owned(),
            }
            .into());
        }
        Ok(())
    }
}
