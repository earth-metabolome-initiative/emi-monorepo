//! Submodule providing the `SnakeCaseColumnName` constraint, which enforces
//! that column names follow `snake_case` style.

use common_traits::builder::Builder;
use snake_case_sanitizer::Sanitizer;
use sql_traits::traits::{ColumnLike, DatabaseLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{ColumnConstraint, Constrainer, GenericConstrainer},
};

/// Struct defining a constraint that enforces that column names follow
/// `snake_case` style.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `SnakeCaseColumnName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = SnakeCaseColumnName::default().into();
///
/// // Invalid: PascalCase
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (MyColumn INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Invalid: double underscore
/// let invalid_schema2 = ParserDB::try_from("CREATE TABLE mytable (my__column INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err());
///
/// // Invalid: camelCase
/// let invalid_schema3 = ParserDB::try_from("CREATE TABLE mytable (myColumn INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema3).is_err());
///
/// // Valid: proper snake_case
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (my_column INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// let valid_schema2 = ParserDB::try_from("CREATE TABLE mytable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
///
/// let valid_schema3 = ParserDB::try_from("CREATE TABLE mytable (first_name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema3).is_ok());
/// ```
pub struct SnakeCaseColumnName<C>(std::marker::PhantomData<C>);

impl<C> Default for SnakeCaseColumnName<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<SnakeCaseColumnName<DB::Column>> for GenericConstrainer<DB> {
    fn from(constraint: SnakeCaseColumnName<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for SnakeCaseColumnName<C> {
    type Column = C;

    fn column_error_information(
        &self,
        column: &Self::Column,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let column_name = column.column_name();
        let sanitizer =
            Sanitizer::default().remove_leading_underscores().remove_trailing_underscores();

        let expected_name =
            sanitizer.to_snake_case(column_name).unwrap_or_else(|_| column_name.to_string());

        let issue = if column_name.contains("__") {
            "contains double underscores"
        } else if column_name.chars().any(|c| c.is_ascii_uppercase()) {
            "contains uppercase letters"
        } else if column_name != expected_name {
            "does not follow snake_case convention"
        } else {
            "is not valid snake_case"
        };

        ConstraintErrorInfo::new()
            .constraint("SnakeCaseColumnName")
            .unwrap()
            .object(column_name.to_owned())
            .unwrap()
            .message(format!(
                "Column '{column_name}' violates snake_case naming convention: {issue}"
            ))
            .unwrap()
            .resolution(format!(
                "Change '{column_name}' to '{expected_name}' (use lowercase letters and single underscores only)"
            ))
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_column(&self, column: &Self::Column) -> Result<(), crate::error::Error> {
        let column_name = column.column_name();
        let sanitizer =
            Sanitizer::default().remove_leading_underscores().remove_trailing_underscores();

        // Check if the name matches its snake_case conversion
        match sanitizer.to_snake_case(column_name) {
            Ok(snake_case_name) if snake_case_name == column_name => Ok(()),
            _ => Err(crate::error::Error::Column(self.column_error_information(column))),
        }
    }
}
