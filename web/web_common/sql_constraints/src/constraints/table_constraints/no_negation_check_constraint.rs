//! Submodule providing the `NoNegationCheckConstraint` constraint, which
//! enforces that tables do not have negation (always false) check
//! constraints.

use common_traits::builder::Builder;
use sql_traits::traits::{CheckConstraintLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that tables do not have
/// negation check constraints.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `NoNegationCheckConstraint` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NoNegationCheckConstraint::default().into();
///
/// // Invalid: has negation check constraint CHECK (false)
/// let invalid_schema = ParserDB::try_from(
///     r#"CREATE TABLE my_table (
///         id INT PRIMARY KEY,
///         age INT CHECK (false)
///     );"#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Invalid: has negation check constraint CHECK (1 = 0)
/// let invalid_schema2 = ParserDB::try_from(
///     r#"CREATE TABLE my_table (
///         id INT PRIMARY KEY,
///         age INT CHECK (1 = 0)
///     );"#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err());
///
/// // Valid: has meaningful check constraint
/// let valid_schema = ParserDB::try_from(
///     r#"CREATE TABLE my_table (
///         id INT PRIMARY KEY,
///         age INT CHECK (age > 0)
///     );"#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct NoNegationCheckConstraint<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for NoNegationCheckConstraint<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NoNegationCheckConstraint<DB>> for GenericConstrainer<DB> {
    fn from(constraint: NoNegationCheckConstraint<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for NoNegationCheckConstraint<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let table_name = context.table_name();

        // Find the first negation check constraint
        let negation_constraint = context
            .check_constraints(database)
            .find(|cc| cc.is_negation(database))
            .map(|cc| cc.expression(database).to_string())
            .unwrap_or_else(|| "unknown".to_string());

        ConstraintErrorInfo::new()
            .constraint("NoNegationCheckConstraint")
            .unwrap()
            .object(table_name.to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' has a negation check constraint: CHECK ({})",
                table_name, negation_constraint
            ))
            .unwrap()
            .resolution("Remove the negation check constraint.".to_string())
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), crate::error::Error> {
        if table.check_constraints(database).any(|cc| cc.is_negation(database)) {
            return Err(crate::error::Error::Table(self.table_error_information(database, table)));
        }
        Ok(())
    }
}
