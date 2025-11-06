//! Submodule providing the `UniqueCheckConstraint` constraint, which enforces
//! that check constraints are unique per table, meaning that no two check
//! constraints have the same clause in a single table.

use common_traits::builder::Builder;
use sql_traits::traits::{CheckConstraintLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that table names are lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `UniqueCheckConstraint` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = UniqueCheckConstraint::default().into();
///
/// let invalid_schema =
///     ParserDB::try_from("CREATE TABLE MyTable (id INT, CHECK (id > 0), CHECK (id > 0));")
///         .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema =
///     ParserDB::try_from("CREATE TABLE mytable (id INT, CHECK (id > 0));").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct UniqueCheckConstraint<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for UniqueCheckConstraint<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<UniqueCheckConstraint<DB>> for GenericConstrainer<DB> {
    fn from(constraint: UniqueCheckConstraint<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for UniqueCheckConstraint<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("UniqueCheckConstraint")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!("Table '{}' has non-unique check constraints", context.table_name()))
            .unwrap()
            .resolution("Ensure all check constraints in the table are unique".to_string())
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
        let mut constraints = table.check_constraints(database).collect::<Vec<_>>();
        constraints.sort_unstable_by_key(|c| c.expression(database));
        for window in constraints.windows(2) {
            if window[0].expression(database) == window[1].expression(database) {
                return Err(crate::error::Error::Table(self.table_error_information(table)));
            }
        }
        Ok(())
    }
}
