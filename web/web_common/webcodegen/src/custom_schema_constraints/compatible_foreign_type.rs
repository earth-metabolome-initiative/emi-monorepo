//! Column constraint that checks whether the type of a column is compatible
//! with its foreign column type, if the column is indeed a foreign key.

use diesel::PgConnection;

use super::CustomColumnConstraint;

/// A column constraint that checks whether the type of a column is compatible
/// with its foreign column type, if the column is indeed a foreign key.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompatibleForeignTypeConstraint;

impl CustomColumnConstraint for CompatibleForeignTypeConstraint {
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &crate::Column,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        for foreign_key in column.foreign_keys(conn)? {
            for (local_column, foreign_column) in foreign_key
                .columns(conn)?
                .into_iter()
                .zip(foreign_key.foreign_columns(conn)?.into_iter())
            {
                if !local_column.has_compatible_data_type(&foreign_column, conn)? {
                    return Err(super::ConstraintError::IncompatibleForeignType {
                        column: Box::new(local_column),
                        foreign_column: Box::new(foreign_column),
                    }
                    .into());
                }
            }
        }
        Ok(())
    }
}
