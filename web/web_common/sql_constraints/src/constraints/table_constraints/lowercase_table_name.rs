//! Submodule providing the `LowercaseTableName` constraint, which enforces that
//! table names are lowercase.

use common_traits::builder::Builder;

use crate::{
    error::ConstraintErrorInfo,
    traits::{ConstrainableTable, Constrainer, Constraint, GenericConstrainer, TableConstraint},
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
/// let constrainer: GenericConstrainer = LowercaseTableName.into();
///
/// let invalid_schema = SqlParserSchema::from_sql("CREATE TABLE MyTable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = SqlParserSchema::from_sql("CREATE TABLE mytable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct LowercaseTableName;

impl From<LowercaseTableName> for GenericConstrainer {
    fn from(constraint: LowercaseTableName) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl Constraint for LowercaseTableName {
    fn error_information(
        &self,
        context: &dyn crate::traits::Constrainable,
    ) -> Box<dyn crate::traits::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("LowercaseTableName")
            .unwrap()
            .object(context.context_name().to_owned())
            .unwrap()
            .message(format!("Table name '{}' is not lowercase", context.context_name()))
            .unwrap()
            .resolution("Rename the table to be all lowercase".to_string())
            .unwrap()
            .build()
            .unwrap()
            .into()
    }
}
impl TableConstraint for LowercaseTableName {
    fn validate_table(&self, table: &dyn ConstrainableTable) -> Result<(), crate::error::Error> {
        if table.table_name().chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) {
            Ok(())
        } else {
            Err(crate::error::Error::Table(self.error_information(table)))
        }
    }
}
