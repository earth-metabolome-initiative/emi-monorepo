//! Column constraint that checks whether the type of a column is compatible with
//! its foreign column type, if the column is indeed a foreign key.

use super::CustomColumnConstraint;

/// A column constraint that checks whether the type of a column is compatible with
/// its foreign column type, if the column is indeed a foreign key.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompatibleForeignTypeConstraint;

impl CustomColumnConstraint for CompatibleForeignTypeConstraint {
    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        column: &crate::Column,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        if let Some((_foreign_table, foreign_column)) = column.foreign_table(conn)? {
            if column.has_compatible_data_type(&foreign_column, conn)? {
                Ok(())
            } else {
                Err(super::ConstraintError::IncompatibleForeignType {
                    column: Box::new(column.clone()),
                    foreign_column: Box::new(foreign_column.clone()),
                }
                .into())
            }
        } else {
            Ok(())
        }
    }
}
