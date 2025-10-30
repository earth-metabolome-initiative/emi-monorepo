//! Submodule providing the `NonRedundantExtensionDag` constraint, which
//! enforces that if a table extends multiple tables (via extension foreign
//! keys), the extended tables must form a non-redundant DAG structure.
//!
//! This means:
//! 1. No duplicate direct extensions to the same table
//! 2. No redundant paths (e.g., if A extends B and B extends C, then A
//!    shouldn't also directly extend C)

use common_traits::builder::Builder;
use sql_traits::traits::{DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces non-redundant extension DAG
/// structure for tables with multiple extensions.
///
/// # Example
///
/// Here follows an example of validating invalid SQL statements with the
/// `NonRedundantExtensionDag` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NonRedundantExtensionDag::default().into();
///
/// // Invalid: Duplicate direct extension to the same table B
/// // The same primary key column (id) appears in two foreign keys to the same table
/// let invalid_duplicate = ParserDB::try_from(
///     r#"
/// CREATE TABLE table_b (id INT PRIMARY KEY);
/// CREATE TABLE table_a (
///     id INT PRIMARY KEY,
///     FOREIGN KEY (id) REFERENCES table_b(id),
///     FOREIGN KEY (id) REFERENCES table_b(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_duplicate).is_err());
///
/// // Invalid: Redundant path (A -> B -> C and A -> C)
/// // table_a's id references both table_b and table_c, but table_b already extends table_c
/// let invalid_redundant = ParserDB::try_from(
///     r#"
/// CREATE TABLE table_c (id INT PRIMARY KEY);
/// CREATE TABLE table_b (id INT PRIMARY KEY REFERENCES table_c(id));
/// CREATE TABLE table_a (
///     id INT PRIMARY KEY,
///     FOREIGN KEY (id) REFERENCES table_b(id),
///     FOREIGN KEY (id) REFERENCES table_c(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_redundant).is_err());
///
/// // Valid: Non-redundant DAG (A -> B -> C and A -> D -> C)
/// // table_a's id references both table_b and table_d, which independently extend table_c
/// let valid_dag = ParserDB::try_from(
///     r#"
/// CREATE TABLE table_c (id INT PRIMARY KEY);
/// CREATE TABLE table_b (id INT PRIMARY KEY REFERENCES table_c(id));
/// CREATE TABLE table_d (id INT PRIMARY KEY REFERENCES table_c(id));
/// CREATE TABLE table_a (
///     id INT PRIMARY KEY,
///     FOREIGN KEY (id) REFERENCES table_b(id),
///     FOREIGN KEY (id) REFERENCES table_d(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_dag).is_ok());
/// ```
pub struct NonRedundantExtensionDag<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for NonRedundantExtensionDag<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NonRedundantExtensionDag<DB>> for GenericConstrainer<DB> {
    fn from(constraint: NonRedundantExtensionDag<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for NonRedundantExtensionDag<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        ConstraintErrorInfo::new()
            .constraint("NonRedundantExtensionDag")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' has redundant extension structure (duplicate or transitive extensions)",
                context.table_name()
            ))
            .unwrap()
            .resolution(
                "Remove redundant foreign key extensions. Ensure each extended table is distinct \
                 and not reachable through another extension path."
                    .to_string(),
            )
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
        // Collect all directly extended tables
        let extended_tables = table.extended_tables(database);

        for (i, extended_table) in extended_tables.iter().enumerate() {
            for (j, other_extended_table) in extended_tables.iter().enumerate() {
                if i == j {
                    continue;
                }
                if other_extended_table.is_descendant_of(database, extended_table)
                    || extended_table == other_extended_table
                {
                    return Err(crate::error::Error::Table(self.table_error_information(table)));
                }
            }
        }

        Ok(())
    }
}
