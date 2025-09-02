//! Submodule providing a constraint to check that if a column is a foreign key
//! to `procedure template asset` and the current table is a procedure or
//! procedure model table, then the foreign key must be cascading if it is
//! always associated to the current procedure or procedure template, and MUST
//! NOT be cascading if it may be associated to other procedures or procedure
//! templates.

use sqlparser::ast::CascadeOption;
use webcodegen::{ConstraintError, CustomColumnConstraint};

use crate::{
    Procedure, ProcedureTemplate, is_in_same_as_with_primary_key,
    is_procedure_template_asset_model_foreign_key,
};

/// Constraint to check that if a column is a foreign key to `procedure template
/// asset` and the current table is a procedure or procedure template table,
/// then the foreign key must be cascading if it is always associated to the
/// current procedure or procedure template, and MUST NOT be cascading if it may
/// be associated to other procedures or procedure templates.
pub struct ProcedureTemplateAssetModelsForeignKeysConstraint;

impl CustomColumnConstraint for ProcedureTemplateAssetModelsForeignKeysConstraint {
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
            if let Some(foreign_key) = is_procedure_template_asset_model_foreign_key(column, conn)?
            {
                if !column.column_name.starts_with("procedure_template_") {
                    return Err(ConstraintError::DoesNotHaveExpectedPrefix {
                        column: Box::new(column.clone()),
                        expected_prefix: "procedure_template_".into(),
                    }
                    .into());
                }
                if !column.column_name.ends_with("_model") {
                    return Err(ConstraintError::DoesNotHaveExpectedSuffix {
                        column: Box::new(column.clone()),
                        expected_suffix: "_model".into(),
                    }
                    .into());
                }

                if column.is_nullable() {
                    return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                        column.clone(),
                    ))
                    .into());
                }

                if is_in_same_as_with_primary_key(column, conn)? {
                    // The column is always associated to the current procedure or
                    // procedure template, so the foreign key MUST be cascading.
                    if !foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            column: Box::new(column.clone()),
                            expected_behavior: CascadeOption::Cascade,
                            found_behavior: CascadeOption::Restrict,
                        }
                        .into());
                    }
                } else {
                    // The column may be associated to other procedures or
                    // procedure templates, so the foreign key MUST NOT be cascading.
                    if foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            column: Box::new(column.clone()),
                            expected_behavior: CascadeOption::Restrict,
                            found_behavior: CascadeOption::Cascade,
                        }
                        .into());
                    }
                }
            }
        }

        Ok(())
    }
}
