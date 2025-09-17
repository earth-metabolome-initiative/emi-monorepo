use diesel::PgConnection;

use crate::{
    Column, CustomTableConstraint,
    custom_schema_constraints::{ConstraintError, CustomColumnConstraint},
    errors::WebCodeGenError,
};

#[derive(Default)]
/// Constraint to enforce that certain words are not used in column names or
/// table names.
pub struct WordDeprecationConstraint {
    // List of deprecated words to check against.
    deprecated_words: Vec<String>,
}

impl<S> From<Vec<S>> for WordDeprecationConstraint
where
    S: ToString,
{
    fn from(deprecated_words: Vec<S>) -> Self {
        Self {
            deprecated_words: deprecated_words
                .into_iter()
                .map(|w| w.to_string().to_lowercase())
                .collect(),
        }
    }
}

impl CustomColumnConstraint for WordDeprecationConstraint {
    type Error = crate::errors::WebCodeGenError;

    fn check_constraint(
        &self,
        _conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError> {
        for deprecated_word in &self.deprecated_words {
            if column.column_name.to_lowercase().contains(deprecated_word) {
                return Err(ConstraintError::ColumnWordDeprecationError {
                    column: Box::new(column.clone()),
                    deprecated_word: deprecated_word.clone(),
                }
                .into());
            }
        }
        Ok(())
    }
}

impl CustomTableConstraint for WordDeprecationConstraint {
    type Error = crate::errors::WebCodeGenError;

    fn check_constraint(
        &self,
        _conn: &mut PgConnection,
        table: &crate::Table,
    ) -> Result<(), WebCodeGenError> {
        for deprecated_word in &self.deprecated_words {
            if table.table_name.to_lowercase().contains(deprecated_word) {
                return Err(ConstraintError::TableWordDeprecationError {
                    table: Box::new(table.clone()),
                    deprecated_word: deprecated_word.clone(),
                }
                .into());
            }
        }
        Ok(())
    }
}
