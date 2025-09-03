//! Submodule providing a constraint to check that if a column which is a
//! decendant of the `asset_models` table is present in a procedure table, then
//! there exist a foreign key from the procedure table to the `procedure_assets`
//! table including that column. Analogously, if the column appears in a
//! procedure template table, then there must be a foreign key from the
//! procedure template table to the `procedure_template_asset_models` table
//! including that column.

use webcodegen::CustomColumnConstraint;

use crate::{Procedure, ProcedureTemplate};

/// Submodule providing the uncharacterized asset models constraint,
/// which checks that if a column which is a decendant of the `asset_models`
/// table is present in a procedure table, then there exist a foreign key from
/// the procedure table to the `procedure_assets` table including that column.
/// Analogously, if the column appears in a procedure template table, then there
/// must be a foreign key from the procedure template table to the
/// `procedure_template_asset_models` table including that column.
pub struct UncharacterizedAssetModelsConstraint;

impl CustomColumnConstraint for UncharacterizedAssetModelsConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        column: &webcodegen::Column,
    ) -> Result<(), Self::Error> {
        let table = column.table(conn)?;
        let extended_tables = table.ancestral_extension_tables(conn)?;

        // If this column is not a descendant of the `asset_models` table,
        // then this constraint is not relevant.
        if !extended_tables.iter().any(|t| t.table_name == "asset_models") {
            return Ok(());
        }

        if Procedure::must_be_procedure_table(&table, conn).is_ok() {
            for (same_as_foreign_key, _) in table.same_as_foreign_keys(conn)? {
                let foreign_table = same_as_foreign_key.foreign_table(conn)?;

                if foreign_table.table_name != "procedure_assets" {
                    continue;
                }

                let columns = same_as_foreign_key.columns(conn)?;

                if columns.contains(column) {
                    return Ok(());
                }
            }

            return Err(crate::errors::Error::UncharacterizedAssetModelColumn(Box::new(
                column.clone(),
            )));
        } else if ProcedureTemplate::must_be_procedure_template_table(&table, conn).is_ok() {
            for (same_as_foreign_key, _) in table.same_as_foreign_keys(conn)? {
                let foreign_table = same_as_foreign_key.foreign_table(conn)?;

                if foreign_table.table_name != "procedure_template_asset_models" {
                    continue;
                }

                let columns = same_as_foreign_key.columns(conn)?;

                if columns.contains(column) {
                    return Ok(());
                }
            }

            return Err(crate::errors::Error::UncharacterizedAssetModelColumn(Box::new(
                column.clone(),
            )));
        }

        Ok(())
    }
}
