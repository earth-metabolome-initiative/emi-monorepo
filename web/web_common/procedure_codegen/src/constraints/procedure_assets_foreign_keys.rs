//! Submodule providing a constraint to check that if a column is a foreign key
//! to `procedure asset` and the current table is a procedure table, then the
//! foreign key must be cascading if it is always associated to
//! the current procedure, and MUST NOT be cascading if it may be
//! associated to other procedures.

use sqlparser::ast::CascadeOption;
use webcodegen::{ConstraintError, CustomColumnConstraint};

use crate::{Procedure, is_procedure_asset_foreign_key};

/// Constraint to check that if a column is a foreign key to `procedure
/// asset` and the current table is a procedure table, then
/// the foreign key must be cascading if it is always associated to the current
/// procedure, and MUST NOT be cascading if it may be
/// associated to other procedures.
pub struct ProcedureAssetsForeignKeysConstraint;

impl CustomColumnConstraint for ProcedureAssetsForeignKeysConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        column: &webcodegen::Column,
    ) -> Result<(), Self::Error> {
        let table = column.table(conn)?;

        if Procedure::must_be_procedure_table(&table, conn).is_ok() {
            if let Some(foreign_key) = is_procedure_asset_foreign_key(column, conn)? {
                if column.ancestral_same_as_constraints(conn)?.is_empty() {
                    // The column may be associated to other procedures, so the foreign key MUST NOT
                    // be cascading.
                    if foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            column: Box::new(column.clone()),
                            expected_behavior: CascadeOption::Restrict,
                            found_behavior: CascadeOption::Cascade,
                        }
                        .into());
                    }
                } else {
                    // The column is always associated to the current procedure so the foreign key
                    // MUST be cascading.
                    if !foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            column: Box::new(column.clone()),
                            expected_behavior: CascadeOption::Cascade,
                            found_behavior: CascadeOption::Restrict,
                        }
                        .into());
                    }
                }
            }
        }

        Ok(())
    }
}
