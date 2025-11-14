//! Submodule providing the `LowercaseColumnName` constraint, which enforces
//! that column names are lowercase.

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{ColumnConstraint, Constrainer, GenericConstrainer},
};

/// Struct defining a constraint that enforces that column names are lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `LowercaseColumnName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = LowercaseColumnName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (Id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct LowercaseColumnName<C>(std::marker::PhantomData<C>);

impl<C> Default for LowercaseColumnName<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<LowercaseColumnName<DB::Column>> for GenericConstrainer<DB> {
    fn from(constraint: LowercaseColumnName<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for LowercaseColumnName<C> {
    type Column = C;
    fn column_error_information(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let table = column.table(database);
        let table_name = table.table_name();
        let column_name = column.column_name();

        ConstraintErrorInfo::new()
            .constraint("LowercaseColumnName")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!("Column '{}' in table '{}' is not lowercase", column_name, table_name))
            .unwrap()
            .resolution(format!(
                "Rename column '{}' in table '{}' to be all lowercase",
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
        if column.column_name().chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) {
            Ok(())
        } else {
            Err(crate::error::Error::Column(self.column_error_information(database, column)))
        }
    }
}
