use async_trait::async_trait;
use diesel_async::AsyncPgConnection;
use crate::{
    Table,
    custom_schema_constraints::{ConstraintError, CustomTableConstraint},
    errors::WebCodeGenError,
};

#[derive(Default)]
/// Constraint to enforce that all column names are lower case.
pub struct LowercaseTableConstraint;

#[async_trait]
impl CustomTableConstraint for LowercaseTableConstraint {
    async fn check_constraint(
        &self,
        _conn: &mut AsyncPgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError> {
        if table.table_name.chars().any(char::is_uppercase) {
            return Err(ConstraintError::UnexpectedUppercaseTable(table.table_name.clone()).into());
        }
        Ok(())
    }
}
