//! Submodule providing the `HasPrimaryKey` constraint, which enforces that
//! tables have a primary key.

use common_traits::builder::Builder;
use sql_traits::traits::{DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that tables have a primary key.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `HasPrimaryKey` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = HasPrimaryKey::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (id INT, name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema =
///     ParserDB::try_from("CREATE TABLE mytable (id INT PRIMARY KEY, name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct HasPrimaryKey<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for HasPrimaryKey<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<HasPrimaryKey<DB>> for GenericConstrainer<DB> {
    fn from(constraint: HasPrimaryKey<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for HasPrimaryKey<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        _database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("HasPrimaryKey")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!("Table '{}' does not have a primary key", context.table_name()))
            .unwrap()
            .resolution("Add a primary key to the table".to_string())
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
        if table.has_primary_key(database) {
            Ok(())
        } else {
            Err(crate::error::Error::Table(self.table_error_information(database, table)))
        }
    }
}
