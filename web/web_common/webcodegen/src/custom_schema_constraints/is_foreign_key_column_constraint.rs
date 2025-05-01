use async_trait::async_trait;
use diesel_async::AsyncPgConnection;
use super::ConstraintError;
use crate::{Column, custom_schema_constraints::CustomColumnConstraint, errors::WebCodeGenError};

#[derive(Debug)]
/// A constraint that checks if a column is a foreign key column
pub struct IsForeignKeyConstraint {
    /// The name of the table
    table_name: String,
    /// The name of the column
    column_name: String,
}

impl IsForeignKeyConstraint {
    #[must_use]
    /// Creates a new instance of the `IsForeignKeyConstraint` constraint.
    pub fn new(table_name: String, column_name: String) -> Self {
        Self { table_name, column_name }
    }
}

#[async_trait]
impl CustomColumnConstraint for IsForeignKeyConstraint {
    async fn check_constraint(
        &self,
        conn: &mut AsyncPgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        if column.column_name == self.column_name
            && column
                .foreign_table(conn).await?
                .is_none_or(|(table, _)| table.table_name != self.table_name)
        {
            return Err(WebCodeGenError::ConstraintError(ConstraintError::NotForeignKeyColumn {
                table_name: self.table_name.clone(),
                column_name: column.column_name.clone(),
            }));
        }
        Ok(())
    }
}
