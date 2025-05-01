use async_trait::async_trait;
use diesel_async::AsyncPgConnection;
use crate::{
    Column,
    custom_schema_constraints::{ConstraintError, CustomColumnConstraint},
    errors::WebCodeGenError,
};

#[derive(Default)]
/// Constraint to enforce that all column names are lower case.
pub struct LowercaseColumnConstraint;

#[async_trait]
impl CustomColumnConstraint for LowercaseColumnConstraint {
    async fn check_constraint(
        &self,
        _conn: &mut AsyncPgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        if column.column_name.chars().any(char::is_uppercase) {
            return Err(
                ConstraintError::UnexpectedUppercaseColumn(column.column_name.clone()).into()
            );
        }
        Ok(())
    }
}
