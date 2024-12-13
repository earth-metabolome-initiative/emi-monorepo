use crate::custom_schema_constraints::CustomColumnConstraint;
use crate::errors::WebCodeGenError;
use crate::Column;
use diesel::pg::PgConnection;

use super::ConstraintError;

#[derive(Debug)]
pub struct IsForeignKeyConstraint {
    table_name: String,
    column_name: String,
}

impl IsForeignKeyConstraint {
    #[must_use]
    /// Creates a new instance of the `IsForeignKeyConstraint` constraint.
    pub fn new(table_name: String, column_name: String) -> Self {
        Self {
            table_name,
            column_name,
        }
    }
}

impl CustomColumnConstraint for IsForeignKeyConstraint {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        if column.column_name == self.column_name
            && column
                .foreign_table(conn)?
                .map_or(true, |(table, _)| table.table_name != self.table_name)
        {
            return Err(WebCodeGenError::ConstraintError(
                ConstraintError::NotForeignKeyColumn {
                    table_name: self.table_name.clone(),
                    column_name: column.column_name.clone(),
                },
            ));
        }
        Ok(())
    }
}
