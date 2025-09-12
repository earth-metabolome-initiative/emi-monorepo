//! Submodule defining the trait builder implementation for a table and all
//! its extensions.

use std::fmt::{Debug, Display};

use diesel::PgConnection;
use proc_macro2::TokenStream;

use crate::{
    Column, PgExtension, Table,
    errors::{CheckConstraintError, CodeGenerationError, WebCodeGenError},
};

impl Table {
    pub(super) fn generate_method_check_constraints<C: AsRef<Column> + Debug + Display>(
        &self,
        columns: &[C],
        check_constraints_extensions: &[&PgExtension],
        conn: &mut PgConnection,
    ) -> Result<Vec<TokenStream>, WebCodeGenError> {
        let all_columns = self.columns(conn)?;
        let insertable_columns = self.insertable_columns(conn, false)?;
        let nullable_insertable_columns: Vec<Column> = insertable_columns
            .iter()
            .filter(|column| !columns.iter().any(|c| c.as_ref() == *column))
            .map(Column::to_nullable)
            .collect();

        let mut check_constraints = columns
            .iter()
            .map(|column| Ok(column.as_ref().check_constraints(conn)?.as_ref().clone()))
            .collect::<Result<Vec<_>, WebCodeGenError>>()?
            .into_iter()
            .flatten()
            .filter(|constraint| !constraint.is_postgis_constraint())
            .collect::<Vec<_>>();

        check_constraints.sort_unstable();

        check_constraints
            .into_iter()
            .filter_map(|constraint| {
                let outcome = constraint.to_syn(
                    columns,
                    &nullable_insertable_columns,
                    check_constraints_extensions,
                    conn,
                );
                if let Err(WebCodeGenError::CodeGenerationError(
                    CodeGenerationError::CheckConstraintError(
                        CheckConstraintError::NoFunctionCalls(_check_constraint),
                    ),
                )) = &outcome
                {
                    return None;
                }
                if let Err(WebCodeGenError::CodeGenerationError(
                    CodeGenerationError::CheckConstraintError(
                        CheckConstraintError::NoInvolvedColumns(unknown_column, _),
                    ),
                )) = &outcome
                {
                    if all_columns.contains(unknown_column)
                        && !insertable_columns.contains(unknown_column)
                    {
                        return None;
                    }
                }
                Some(outcome)
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()
    }
}
