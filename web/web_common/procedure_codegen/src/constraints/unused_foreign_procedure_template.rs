//! Submodule defining a constraint to check that if there exist one or more
//! same-as indices in a procedure template table linking to a foreign procedure
//! template, then these indices must be used in the associated procedure table.

use webcodegen::CustomTableConstraint;

use crate::{ProcedureTemplate, errors::Error};

/// Constraint to check that if there exist one or more same-as indices in a
/// procedure template table linking to a foreign procedure template, then these
/// indices must be used in the associated procedure table.
pub struct UnusedForeignProcedureTemplateConstraint;

impl CustomTableConstraint for UnusedForeignProcedureTemplateConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        table: &webcodegen::Table,
    ) -> Result<(), Self::Error> {
        if let Ok(procedure_template) = ProcedureTemplate::from_table(table.clone(), conn) {
            let mut same_as_indices =
                procedure_template.foreign_procedure_template_same_as_indices(conn)?;

            // We load the associated procedure table.
            let procedure = procedure_template.procedure(conn)?;
            // We iterate through the same-as foreign keys of the procedure table and remove
            // the associated same-as indices from the list of same-as indices of the
            // procedure template table.
            for (_, foreign_same_as_index) in procedure.as_ref().same_as_foreign_keys(conn)? {
                if let Some(position) =
                    same_as_indices.iter().position(|index| index == &foreign_same_as_index)
                {
                    same_as_indices.remove(position);
                }
            }
            if !same_as_indices.is_empty() {
                let same_as_index = same_as_indices.remove(0);
                let columns = same_as_index.columns(conn)?;

                return Err(Error::UnusedForeignProcedureTemplateConstraint {
                    table: Box::new(table.clone()),
                    columns,
                });
            }
        }
        Ok(())
    }
}
