//! Submodule providing the `UniqueUniqueIndex` constraint, which enforces
//! that check constraints are unique per table, meaning that no two check
//! constraints have the same clause in a single table.

use common_traits::builder::Builder;
use sql_traits::traits::{DatabaseLike, TableLike, UniqueIndexLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that table names are lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `UniqueUniqueIndex` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = UniqueUniqueIndex::default().into();
///
/// let invalid_schema =
///     ParserDB::try_from("CREATE TABLE MyTable (id INT, UNIQUE (id), UNIQUE (id));").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (id INT, UNIQUE (id));").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct UniqueUniqueIndex<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for UniqueUniqueIndex<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<UniqueUniqueIndex<DB>> for GenericConstrainer<DB> {
    fn from(constraint: UniqueUniqueIndex<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for UniqueUniqueIndex<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("UniqueUniqueIndex")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!("Table '{}' has non-unique unique index", context.table_name()))
            .unwrap()
            .resolution("Ensure all unique index in the table are unique".to_string())
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
        let mut constraints = table.unique_indices(database).collect::<Vec<_>>();
        constraints.sort_unstable_by_key(|c| c.expression(database));
        for window in constraints.windows(2) {
            if window[0].expression(database) == window[1].expression(database) {
                return Err(crate::error::Error::Table(self.table_error_information(table)));
            }
        }
        Ok(())
    }
}
