//! Submodule providing the `NonCompositePrimaryKeyNamedId` constraint, which
//! enforces that if a column is a non-composite primary key, it must be named
//! "id".

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{ColumnConstraint, Constrainer, GenericConstrainer},
};

/// Struct defining a constraint that enforces that if a column is a
/// non-composite primary key, it must be named "id".
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
/// let invalid_schema =
///     ParserDB::try_from("CREATE TABLE mytable (pk INT PRIMARY KEY, name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema =
///     ParserDB::try_from("CREATE TABLE mytable (id INT PRIMARY KEY, name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// // Composite primary keys are allowed to have any name
/// let valid_composite_schema = ParserDB::try_from(
///     "CREATE TABLE mytable (pk1 INT, pk2 INT, name TEXT, PRIMARY KEY (pk1, pk2));",
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_composite_schema).is_ok());
/// ```
pub struct NonCompositePrimaryKeyNamedId<C>(std::marker::PhantomData<C>);

impl<C> Default for NonCompositePrimaryKeyNamedId<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NonCompositePrimaryKeyNamedId<DB::Column>>
    for GenericConstrainer<DB>
{
    fn from(constraint: NonCompositePrimaryKeyNamedId<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for NonCompositePrimaryKeyNamedId<C> {
    type Column = C;

    fn column_error_information(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let column_name = column.column_name();
        let table = column.table(database);
        let table_name = table.table_name();

        ConstraintErrorInfo::new()
            .constraint("NonCompositePrimaryKeyNamedId")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!(
                "Column '{}' in table '{}' is a non-composite primary key but is not named 'id'",
                column_name, table_name
            ))
            .unwrap()
            .resolution(format!(
                "Rename the primary key column '{}' to 'id' in table '{}'",
                column_name, table_name
            ))
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_column(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Result<(), crate::error::Error> {
        // Check if this column is a primary key
        if !column.is_primary_key(database) {
            return Ok(());
        }

        // Get the table to check if it has a composite primary key
        let table = column.table(database);
        let pk_columns: Vec<_> = table.primary_key_columns(database).collect();

        // If the primary key is composite, the constraint doesn't apply
        if pk_columns.len() > 1 {
            return Ok(());
        }

        // Single primary key column must be named "id"
        if column.column_name() == "id" {
            Ok(())
        } else {
            Err(crate::error::Error::Column(self.column_error_information(database, column)))
        }
    }
}
