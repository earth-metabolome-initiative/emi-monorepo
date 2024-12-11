use crate::custom_schema_constraints::CustomTableConstraint;
use crate::errors::WebCodeGenError;
use diesel::pg::PgConnection;

use super::ConstraintError;

pub struct CompulsorySiblingColumnConstraint {
    column_name: String,
    sibling_column_name: String,
}

impl CustomTableConstraint for CompulsorySiblingColumnConstraint {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &crate::Table,
    ) -> Result<(), WebCodeGenError> {
        if table.columns(conn)?.into_iter().fold(false, |acc, column| {
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
