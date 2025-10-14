//! Submodule providing the `UniqueForeignKey` constraint, which enforces
//! that foreign keys are unique per table, meaning that no two foreign keys
//! have the same columns in a single table.

use std::hash::{DefaultHasher, Hash, Hasher};

use common_traits::builder::Builder;
use sql_traits::traits::{DatabaseLike, ForeignKeyLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that table names are lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `UniqueForeignKey` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = UniqueForeignKey::default().into();
///
/// let invalid_schema =
///     ParserDB::from_sql("CREATE TABLE MyTable (id INT PRIMARY KEY REFERENCES MyTable (id), FOREIGN KEY (id) REFERENCES MyTable (id));")
///         .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err(), "1) Foreign keys must be unique per table");
///
/// let invalid_schema2 =
///     ParserDB::from_sql("CREATE TABLE MyTable (id INT PRIMARY KEY, FOREIGN KEY (id) REFERENCES MyTable (id), FOREIGN KEY (id) REFERENCES MyTable (id));").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err(), "2) Foreign keys must be unique per table");
///
/// let valid_schema =
///     ParserDB::from_sql("CREATE TABLE mytable (id INT PRIMARY KEY, FOREIGN KEY (id) REFERENCES mytable (id));").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct UniqueForeignKey<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for UniqueForeignKey<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<UniqueForeignKey<DB>> for GenericConstrainer<DB> {
    fn from(constraint: UniqueForeignKey<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for UniqueForeignKey<DB> {
    type Table = DB::Table;
    type Database = DB;
    fn table_error_information(
        &self,
        context: &Self::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("UniqueForeignKey")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!("Table '{}' has non-unique foreign key", context.table_name()))
            .unwrap()
            .resolution("Ensure all foreign keys in the table are unique".to_string())
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &Self::Table,
    ) -> Result<(), crate::error::Error> {
        let constraints = table.foreign_keys(database).collect::<Vec<_>>();
        let mut signatures = constraints
            .iter()
            .map(|c| {
                let mut hasher = DefaultHasher::new();
                for host_col in c.host_columns(database) {
                    host_col.hash(&mut hasher);
                }
                let referenced_table = c.referenced_table(database);
                referenced_table.hash(&mut hasher);
                for referenced_col in c.referenced_columns(database) {
                    referenced_col.hash(&mut hasher);
                }
                hasher.finish()
            })
            .collect::<Vec<_>>();

        signatures.sort_unstable();

        for window in signatures.windows(2) {
            if let [sig1, sig2] = window {
                if sig1 == sig2 {
                    return Err(crate::error::Error::Table(self.table_error_information(table)));
                }
            }
        }

        Ok(())
    }
}
