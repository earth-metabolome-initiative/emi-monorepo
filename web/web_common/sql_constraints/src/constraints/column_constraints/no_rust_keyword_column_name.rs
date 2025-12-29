//! Submodule providing the `NoRustKeywordColumnName` constraint, which enforces
//! that column names are not Rust keywords.

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    constraints::rust_keywords::is_rust_keyword,
    error::ConstraintErrorInfo,
    traits::{ColumnConstraint, Constrainer, GenericConstrainer},
};

/// Struct defining a constraint that enforces that column names are not Rust
/// keywords.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `NoRustKeywordColumnName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NoRustKeywordColumnName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (struct INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (my_struct INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct NoRustKeywordColumnName<C>(std::marker::PhantomData<C>);

impl<C> Default for NoRustKeywordColumnName<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NoRustKeywordColumnName<DB::Column>>
    for GenericConstrainer<DB>
{
    fn from(constraint: NoRustKeywordColumnName<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for NoRustKeywordColumnName<C> {
    type Column = C;

    fn column_error_information(
        &self,
        _database: &<Self::Column as ColumnLike>::DB,
        context: &Self::Column,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let column_name = context.column_name();
        let table_name = context.table(_database).table_name();
        ConstraintErrorInfo::new()
            .constraint("NoRustKeywordColumnName")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!(
                "Column name '{}' in table '{}' is a Rust keyword.",
                column_name, table_name
            ))
            .unwrap()
            .resolution(format!(
                "Rename the column '{}' to something that is not a Rust keyword.",
                column_name
            ))
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_column(
        &self,
        _database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Result<(), crate::error::Error> {
        let column_name = column.column_name();
        if is_rust_keyword(column_name) {
            return Err(crate::error::Error::Column(
                self.column_error_information(_database, column),
            ));
        }
        Ok(())
    }
}
