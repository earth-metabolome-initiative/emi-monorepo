//! Submodule providing the `LowercaseTableName` constraint, which enforces that
//! table names are lowercase.

use common_traits::builder::Builder;
use sql_traits::traits::{DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that table names are lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `LowercaseTableName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = LowercaseTableName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE MyTable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct LowercaseTableName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for LowercaseTableName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<LowercaseTableName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: LowercaseTableName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for LowercaseTableName<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("LowercaseTableName")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!("Table name '{}' is not lowercase", context.table_name()))
            .unwrap()
            .resolution("Rename the table to be all lowercase".to_string())
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_table(
        &self,
        _database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), crate::error::Error> {
        if table.table_name().chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) {
            Ok(())
        } else {
            Err(crate::error::Error::Table(self.table_error_information(table)))
        }
    }
}
