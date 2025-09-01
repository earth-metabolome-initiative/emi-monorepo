//! Submodule providing the procedure to procedure template foreign key
//! constraint, which checks that the foreign key from a procedure table to a
//! procedure model table is called `procedure_template`.  This type of check
//! constraint is important to avoid cognitive overhead.

use webcodegen::{ConstraintError, CustomTableConstraint};

use crate::Procedure;

/// Submodule providing the procedure to procedure template foreign key
/// constraint, which checks that the foreign key from a procedure table to a
/// procedure model table is called `procedure_template`.  This type of check
/// constraint is important to avoid cognitive overhead.
pub struct ProcedureToProcedureTemplateForeignKeyConstraint;

impl CustomTableConstraint for ProcedureToProcedureTemplateForeignKeyConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        table: &webcodegen::Table,
    ) -> Result<(), Self::Error> {
        if Procedure::must_be_procedure_table(table, conn).is_ok() {
            let procedure = Procedure::load_by_name(
                table.table_catalog.as_str(),
                table.table_name.as_str(),
                conn,
            )
            .expect(&format!(
                "Failed to load procedure by name for table `{}.{}` that must be a procedure table",
                table.table_catalog, table.table_name
            ));
            let Some(procedure_template_foreign_key) = procedure.procedure_template_foreign_key(conn).expect(
				"Failed to get procedure template foreign key for a procedure table that must be valid",
			) else {
                return Ok(());
            };

            let mut columns = procedure_template_foreign_key.columns(conn)?;
            assert_eq!(columns.len(), 1, "Procedure tables must have exactly one column");
            let column = columns.remove(0);
            if column.column_name != "procedure_template" {
                return Err(ConstraintError::DoesNotHaveExpectedName {
                    column: Box::new(column),
                    expected_name: "procedure_template".to_owned(),
                }
                .into());
            }

            if column.is_nullable() {
                return Err(ConstraintError::UnexpectedNullableColumn(Box::new(column)).into());
            }
        }
        Ok(())
    }
}
