//! Submodule providing the `NoTautologicalCheckConstraint` constraint, which
//! enforces that tables do not have tautological (always true) check
//! constraints.

use common_traits::builder::Builder;
use sql_traits::traits::{CheckConstraintLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that tables do not have
/// tautological check constraints.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `NoTautologicalCheckConstraint` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NoTautologicalCheckConstraint::default().into();
///
/// // Invalid: has tautological check constraint CHECK (true)
/// let invalid_schema = ParserDB::try_from(
///     r#"CREATE TABLE my_table (
///         id INT PRIMARY KEY,
///         age INT CHECK (true)
///     );"#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Invalid: has tautological check constraint CHECK (1 = 1)
/// let invalid_schema2 = ParserDB::try_from(
///     r#"CREATE TABLE my_table (
///         id INT PRIMARY KEY,
///         age INT CHECK (1 = 1)
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
pub struct NoTautologicalCheckConstraint<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for NoTautologicalCheckConstraint<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NoTautologicalCheckConstraint<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: NoTautologicalCheckConstraint<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for NoTautologicalCheckConstraint<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let table_name = context.table_name();

        // Find the first tautological check constraint
        let tautological_constraint = context
            .check_constraints(database)
            .find(|cc| cc.is_tautology(database))
            .map(|cc| cc.expression(database).to_string())
            .unwrap_or_else(|| "unknown".to_string());

        ConstraintErrorInfo::new()
            .constraint("NoTautologicalCheckConstraint")
            .unwrap()
            .object(table_name.to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' has a tautological check constraint: CHECK ({})",
                table_name, tautological_constraint
            ))
            .unwrap()
            .resolution(format!(
                "Remove the tautological check constraint 'CHECK ({})' from table '{}'",
                tautological_constraint, table_name
            ))
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
        // Check if any check constraint is tautological
        for check_constraint in table.check_constraints(database) {
            if check_constraint.is_tautology(database) {
                return Err(crate::error::Error::Table(
                    self.table_error_information(database, table),
                ));
            }
        }

        Ok(())
    }
}
