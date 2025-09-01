//! Column constraint that checks whether the type of a column is compatible
//! with its foreign column type, if the column is indeed a foreign key.

use diesel::PgConnection;

use super::CustomTableConstraint;

/// A column constraint that checks whether the type of a column is compatible
/// with its foreign column type, if the column is indeed a foreign key.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompatibleForeignTypeConstraint;

impl CustomTableConstraint for CompatibleForeignTypeConstraint {
    type Error = crate::errors::WebCodeGenError;

    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &crate::Table,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        for foreign_key in table.foreign_keys(conn)? {
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
