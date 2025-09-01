//! Submodule providing a constraint to check that if a column is an asset
//! foreign key, i.e. a foreign key to either the `asset_models` table or any
//! table descending from it, and the current table is a procedure or procedure
//! model table, then the foreign key must not be cascading.

use diesel::OptionalExtension;
use webcodegen::{ConstraintError, CustomColumnConstraint};

use crate::{
    Procedure, ProcedureTemplate, errors::ProcedureTemplateError, is_asset_model_foreign_key,
    is_procedure_template_asset_model_foreign_key,
};

/// Constraint to check that if a column which is a decendant of the
/// `asset_models` table is present in a procedure or procedure template table,
/// then its foreign key must not be cascading.
pub struct AssetModelsForeignKeysConstraint;

impl CustomColumnConstraint for AssetModelsForeignKeysConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        column: &webcodegen::Column,
    ) -> Result<(), Self::Error> {
        let table = column.table(conn)?;
        let is_procedure_template_table =
            ProcedureTemplate::must_be_procedure_template_table(&table).is_ok();

        if Procedure::must_be_procedure_table(&table, conn).is_ok() || is_procedure_template_table {
            if let Some(foreign_key) = is_asset_model_foreign_key(column, conn)? {
                if !column.column_name.ends_with("_model") {
                    return Err(ConstraintError::DoesNotHaveExpectedSuffix {
                        column: Box::new(column.clone()),
                        expected_suffix: "_model".into(),
                    }
                    .into());
                }

                // If we are currently in a procedure template table, we also check that there
                // exists in the table a column which is a procedure template
                // asset model with the name `procedure_<X>_model` where `<X>`
                // is the prefix of the current column name.
                if is_procedure_template_table {
                    let expected_column_name = format!("procedure_template_{}", column.column_name);
                    let Some(procedure_template_model_candidate) =
                        table.column_by_name(conn, &expected_column_name).optional()?
                    else {
                        // If we do not find such a column, we return an error.
                        return Err(ConstraintError::DoesNotHaveSiblingColumn {
                            column: Box::new(column.clone()),
                            sibling_column_name: expected_column_name,
                        }
                        .into());
                    };
                    // Next, we check that the found column is indeed a procedure
                    // template asset model.
                    if is_procedure_template_asset_model_foreign_key(
                        &procedure_template_model_candidate,
                        conn,
                    )?
                    .is_none()
                    {
                        return Err(
                            ProcedureTemplateError::ExpectedProcedureTemplateAssetModelForeignKey {
                                column: Box::new(procedure_template_model_candidate),
                            }
                            .into(),
                        );
                    }
                }

                if foreign_key.has_on_delete_cascade(conn)? {
                    return Err(crate::errors::Error::CascadingAssetModelForeignKey(Box::new(
                        column.clone(),
                    )));
                }
            }
        }

        Ok(())
    }
}
