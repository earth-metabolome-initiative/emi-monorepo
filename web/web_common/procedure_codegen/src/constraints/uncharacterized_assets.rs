//! Submodule providing a constraint to check that if a column which is a
//! decendant of the `assets` table is present in a procedure table, then there
//! exist a foreign key from the procedure table to the `procedure_assets` table
//! including that column.

use webcodegen::CustomColumnConstraint;

use crate::Procedure;

/// Submodule providing the uncharacterized assets constraint,
/// which checks that if a column which is a decendant of the `assets` table
/// is present in a procedure table, then there exist a foreign key from the
/// procedure table to the `procedure_assets` table including that column.
pub struct UncharacterizedAssetsConstraint;

impl CustomColumnConstraint for UncharacterizedAssetsConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        column: &webcodegen::Column,
    ) -> Result<(), Self::Error> {
        let table = column.table(conn)?;
        let extended_tables = table.ancestral_extension_tables(conn)?;

        // If this column is not a descendant of the `assets` table,
        // then this constraint is not relevant.
        if !extended_tables.iter().any(|t| t.table_name == "assets") {
            return Ok(());
        }

        if Procedure::must_be_procedure_table(&table, conn).is_ok() {
            for (same_as_foreign_key, _) in table.same_as_foreign_keys(conn)? {
                let foreign_table = same_as_foreign_key.foreign_table(conn)?.expect(
                    "Failed to get foreign table for same-as foreign key on a procedure table",
                );

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
        }

        Ok(())
    }
}
