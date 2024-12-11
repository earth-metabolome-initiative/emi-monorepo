use crate::custom_schema_constraints::CustomColumnConstraint;
use crate::errors::WebCodeGenError;
use crate::{Column, Table};
use diesel::pg::PgConnection;

use super::ConstraintError;

#[derive(Debug)]
pub struct IsForeignKeyConstraint<'a> {
    table: &'a Table,
    column_name: String,
}

impl<'a> IsForeignKeyConstraint<'a> {
    /// Creates a new instance of the `IsForeignKeyConstraint` constraint.
    pub fn new<'b>(table: &'b Table, column_name: String) -> IsForeignKeyConstraint<'b> {
        IsForeignKeyConstraint {
            table,
            column_name,
        }
    }
}

impl<'a> CustomColumnConstraint for IsForeignKeyConstraint<'a> {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        if column.column_name == self.column_name
            && column
                .foreign_table(conn)?
                .map_or(true, |(table, _)| &table != self.table)
        {
            return Err(WebCodeGenError::ConstraintError(
                ConstraintError::NotForeignKeyColumn {
                    table: self.table.clone(),
                    column: column.clone()
                },
            ));
        }
        Ok(())
    }
}
