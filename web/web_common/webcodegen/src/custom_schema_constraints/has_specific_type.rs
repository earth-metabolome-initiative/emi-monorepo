use diesel::PgConnection;

use super::ConstraintError;
use crate::{Column, custom_schema_constraints::CustomColumnConstraint, errors::WebCodeGenError};

/// Check that a column has a specific type
pub struct HasSpecificTypeConstraint<'column> {
    /// The name of the column
    column_name: &'column str,
    /// The expected type of the column
    column_type: &'column str,
}

impl<'column> HasSpecificTypeConstraint<'column> {
    #[must_use]
    /// Create a new `HasSpecificTypeConstraint`
    pub fn new(column_name: &'column str, column_type: &'column str) -> Self {
        Self { column_name, column_type }
    }
}

impl CustomColumnConstraint for HasSpecificTypeConstraint<'_> {
    type Error = crate::errors::WebCodeGenError;

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
