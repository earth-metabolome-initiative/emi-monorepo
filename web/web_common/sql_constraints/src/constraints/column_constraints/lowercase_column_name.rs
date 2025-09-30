//! Submodule providing the `LowercaseColumnName` constraint, which enforces
//! that column names are lowercase.

use common_traits::builder::Builder;

use crate::{
    error::ConstraintErrorInfo,
    traits::{ColumnConstraint, ConstrainableColumn, Constrainer, Constraint, GenericConstrainer},
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
/// let constrainer: GenericConstrainer = LowercaseColumnName.into();
///
/// let invalid_schema = SqlParserSchema::from_sql("CREATE TABLE mytable (Id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = SqlParserSchema::from_sql("CREATE TABLE mytable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct LowercaseColumnName;

impl From<LowercaseColumnName> for GenericConstrainer {
    fn from(constraint: LowercaseColumnName) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl Constraint for LowercaseColumnName {
    fn error_information(
        &self,
        context: &dyn crate::traits::Constrainable,
    ) -> Box<dyn crate::traits::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("LowercaseColumnName")
            .unwrap()
            .object(context.context_name().to_owned())
            .unwrap()
            .message(format!("Column name '{}' is not lowercase", context.context_name()))
            .unwrap()
            .resolution("Rename the column to be all lowercase".to_string())
            .unwrap()
            .build()
            .unwrap()
            .into()
    }
}
impl ColumnConstraint for LowercaseColumnName {
    fn validate_column(&self, column: &dyn ConstrainableColumn) -> Result<(), crate::error::Error> {
        if column.column_name().chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) {
            Ok(())
        } else {
            Err(crate::error::Error::Column(self.error_information(column)))
        }
    }
}
