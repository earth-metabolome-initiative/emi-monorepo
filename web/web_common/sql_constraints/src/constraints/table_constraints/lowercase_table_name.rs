//! Submodule providing the `LowercaseTableName` constraint, which enforces that table names
//! are lowercase.

use crate::traits::{ConstrainableTable, Constraint, TableConstraint};

/// Struct defining a constraint that enforces that table names are lowercase.
pub struct LowercaseTableName;

impl Constraint for LowercaseTableName {
    fn error_information(
        &self,
        context: &dyn crate::traits::Constrainable,
    ) -> Box<dyn crate::traits::ConstraintFailureInformation> {
        // Implementation of error information generation goes here.
        unimplemented!()
    }
}
impl TableConstraint for LowercaseTableName {
    fn validate_table(&self, table: &dyn ConstrainableTable) -> Result<(), crate::error::Error> {
        if table.name().chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) {
            Ok(())
        } else {
            Err(crate::error::Error::Table(self.error_information(table)))
        }
    }
}
