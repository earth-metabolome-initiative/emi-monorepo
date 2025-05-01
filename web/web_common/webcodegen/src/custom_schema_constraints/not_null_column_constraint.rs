use async_trait::async_trait;
use diesel_async::AsyncPgConnection;
use crate::{
    Column,
    custom_schema_constraints::{ConstraintError, CustomColumnConstraint},
    errors::WebCodeGenError,
};

/// Constraint to enforce that columns with a given name must not be nullable.
pub struct NotNullColumnConstraint {
    /// The column name to be checked for none nullabilty
    column_name: String,
}

impl NotNullColumnConstraint {
    #[must_use]
    /// Create a new instance of the constraint
    pub fn new(column_name: &str) -> Self {
        Self { column_name: column_name.to_string() }
    }
}

#[async_trait]
impl CustomColumnConstraint for NotNullColumnConstraint {
    async fn check_constraint(
        &self,
        _conn: &mut AsyncPgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        if self.column_name == column.column_name && column.is_nullable() {
            return Err(ConstraintError::UnexpectedNullableColumn(
                column.table_name.clone(),
                self.column_name.clone(),
            )
            .into());
        }
        Ok(())
    }
}
