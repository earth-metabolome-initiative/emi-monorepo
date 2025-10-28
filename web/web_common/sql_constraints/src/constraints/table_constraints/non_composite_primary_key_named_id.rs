//! Submodule providing the `NonCompositePrimaryKeyNamedId` constraint, which
//! enforces that if a table has a non-composite primary key, the column must be
//! named "id".

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that if a table has a
/// non-composite primary key, the column must be named "id".
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `NonCompositePrimaryKeyNamedId` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NonCompositePrimaryKeyNamedId::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (pk INT PRIMARY KEY, name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (id INT PRIMARY KEY, name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// // Composite primary keys are allowed to have any name
/// let valid_composite_schema = ParserDB::try_from("CREATE TABLE mytable (pk1 INT, pk2 INT, name TEXT, PRIMARY KEY (pk1, pk2));").unwrap();
/// assert!(constrainer.validate_schema(&valid_composite_schema).is_ok());
/// ```
pub struct NonCompositePrimaryKeyNamedId<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for NonCompositePrimaryKeyNamedId<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NonCompositePrimaryKeyNamedId<DB>> for GenericConstrainer<DB> {
    fn from(constraint: NonCompositePrimaryKeyNamedId<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for NonCompositePrimaryKeyNamedId<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("NonCompositePrimaryKeyNamedId")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' has a non-composite primary key, but the column is not named 'id'",
                context.table_name()
            ))
            .unwrap()
            .resolution("Rename the primary key column to 'id'".to_string())
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
        let pk_columns: Vec<_> = table.primary_key_columns(database).collect();
        
        // If there's no primary key or it's composite, the constraint doesn't apply
        if pk_columns.is_empty() || pk_columns.len() > 1 {
            return Ok(());
        }
        
        // Single primary key column must be named "id"
        let pk_column = pk_columns[0];
        if pk_column.column_name() == "id" {
            Ok(())
        } else {
            Err(crate::error::Error::Table(self.table_error_information(table)))
        }
    }
}
