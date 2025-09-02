//! Submodule providing a constraint to check that if a column is an asset
//! foreign key, i.e. a foreign key to either the `assets` table or any table
//! descending from it, and the current table is a procedure or procedure
//! template table, then the foreign key must not be cascading.

use webcodegen::CustomColumnConstraint;

use crate::{Procedure, ProcedureTemplate, is_asset_foreign_key};

/// Constraint to check that if a column which is a decendant of the `assets`
/// table is present in a procedure or procedure template table, then its
/// foreign key must not be cascading.
pub struct AssetForeignKeysConstraint;

impl CustomColumnConstraint for AssetForeignKeysConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        column: &webcodegen::Column,
    ) -> Result<(), Self::Error> {
        let table = column.table(conn)?;

        if Procedure::must_be_procedure_table(&table, conn).is_ok()
            || ProcedureTemplate::must_be_procedure_template_table(&table, conn).is_ok()
        {
            if let Some(foreign_key) = is_asset_foreign_key(column, conn)? {
                if foreign_key.has_on_delete_cascade(conn)? {
                    return Err(crate::errors::Error::CascadingAssetForeignKey(Box::new(
                        column.clone(),
                    )));
                }
            }
        }

        Ok(())
    }
}
