use async_trait::async_trait;
use diesel_async::AsyncPgConnection;

use super::ConstraintError;
use crate::{custom_schema_constraints::CustomTableConstraint, errors::WebCodeGenError};

/// Check that a column has a sibling column
pub struct CompulsorySiblingColumnConstraint {
    /// The name of the column
    column_name: String,
    /// The name of the mandatory sibling column
    sibling_column_name: String,
}

#[async_trait]
impl CustomTableConstraint for CompulsorySiblingColumnConstraint {
    async fn check_constraint(
        &self,
        conn: &mut AsyncPgConnection,
        table: &crate::Table,
    ) -> Result<(), WebCodeGenError> {
        if table.columns(conn).await?.into_iter().fold(false, |acc, column| {
            if column.column_name == self.column_name
                || column.column_name == self.sibling_column_name
            {
                !acc
            } else {
                acc
            }
        }) {
            return Err(ConstraintError::DoesNotHaveSiblingColumn {
                table_name: table.table_name.clone(),
                column_name: self.column_name.clone(),
                sibling_column_name: self.sibling_column_name.clone(),
            }
            .into());
        }

        Ok(())
    }
}
