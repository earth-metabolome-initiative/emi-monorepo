//! Submodule providing the procedure template primary key constraint, which
//! checks that the primary key of a procedure template table is called
//! `procedure_template`. This type of check constraint is important to avoid
//! cognitive overhead.

use sqlparser::ast::CascadeOption;
use webcodegen::{ConstraintError, CustomTableConstraint};

use crate::ProcedureTemplate;

/// Submodule providing the procedure template primary key constraint, which
/// checks that the primary key of a procedure template table is called
/// `procedure_template`. This type of check constraint is important to avoid
/// cognitive overhead.
pub struct ProcedureTemplatePrimaryKeyConstraint;

impl CustomTableConstraint for ProcedureTemplatePrimaryKeyConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        table: &webcodegen::Table,
    ) -> Result<(), Self::Error> {
        if ProcedureTemplate::must_be_procedure_template_table(table, conn).is_ok() {
            let mut primary_keys = table.primary_key_columns(conn)?;
            let first_primary_key = primary_keys.remove(0);
            if first_primary_key.column_name != "procedure_template" {
                return Err(webcodegen::ConstraintError::DoesNotHaveExpectedName {
                    column: Box::new(first_primary_key),
                    expected_name: "procedure_template".to_owned(),
                }
                .into());
            }
            // We check that if there exist an extension foreign key associated to the
            // primary key, then this foreign key must be cascading.
            if let Some(foreign_key) = first_primary_key.is_part_of_extension_primary_key(conn)? {
                if !foreign_key.has_on_delete_cascade(conn)? {
                    return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                        columns: vec![first_primary_key],
                        expected_behavior: CascadeOption::Cascade,
                        found_behavior: CascadeOption::Restrict,
                    }
                    .into());
                }
            }
        }
        Ok(())
    }
}
