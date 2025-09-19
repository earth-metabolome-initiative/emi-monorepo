//! Submodule providing the procedure primary key constraint, which checks that
//! the primary key of a procedure table is called `procedure`.  This type of
//! check constraint is important to avoid cognitive overhead.

use sqlparser::ast::CascadeOption;
use webcodegen::{ConstraintError, CustomTableConstraint};

use crate::Procedure;

/// Submodule providing the procedure primary key constraint, which checks that
/// the primary key of a procedure table is called `procedure`.  This type of
/// check constraint is important to avoid cognitive overhead.
pub struct ProcedurePrimaryKeyConstraint;

impl CustomTableConstraint for ProcedurePrimaryKeyConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        table: &webcodegen::Table,
    ) -> Result<(), Self::Error> {
        if Procedure::must_be_procedure_table(table, conn).is_ok() {
            let primary_keys = table.primary_key_columns(conn)?;
            let first_primary_key = &primary_keys[0];
            if first_primary_key.column_name != "procedure" {
                return Err(webcodegen::ConstraintError::DoesNotHaveExpectedName {
                    column: Box::new(first_primary_key.clone()),
                    expected_name: "procedure".to_owned(),
                }
                .into());
            }
            // We check that if there exist an extension foreign key associated to the
            // primary key, then this foreign key must be cascading.
            if let Some(foreign_key) = first_primary_key.is_part_of_extension_primary_key(conn)?
                && !foreign_key.has_on_delete_cascade(conn)?
            {
                return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                    columns: vec![first_primary_key.clone()],
                    expected_behavior: CascadeOption::Cascade,
                    found_behavior: CascadeOption::Restrict,
                }
                .into());
            }
        }
        Ok(())
    }
}
