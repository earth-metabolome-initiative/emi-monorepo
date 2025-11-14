//! Submodule providing the `PluralTableName` constraint, which enforces that
//! the last segment of table names is plural.

use common_traits::builder::Builder;
use inflector::Inflector;
use sql_traits::traits::{DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that the last segment of table
/// names is plural.
///
/// For table names with underscores (e.g., `user_accounts`), only the last
/// segment after the final underscore is checked for plurality.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `PluralTableName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = PluralTableName::default().into();
///
/// // Invalid singular table names
/// let invalid_schema = ParserDB::try_from("CREATE TABLE user (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let invalid_schema2 = ParserDB::try_from("CREATE TABLE user_account (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err());
///
/// // Valid plural table names
/// let valid_schema = ParserDB::try_from("CREATE TABLE users (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// let valid_schema2 = ParserDB::try_from("CREATE TABLE user_accounts (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
///
/// // Edge cases with Latin plurals
/// let valid_spectra = ParserDB::try_from("CREATE TABLE spectra (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_spectra).is_ok());
///
/// let invalid_spectrum = ParserDB::try_from("CREATE TABLE spectrum (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_spectrum).is_err());
///
/// let valid_matrices = ParserDB::try_from("CREATE TABLE matrices (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_matrices).is_ok());
///
/// let invalid_matrix = ParserDB::try_from("CREATE TABLE matrix (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_matrix).is_err());
///
/// let valid_taxa = ParserDB::try_from("CREATE TABLE taxa (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_taxa).is_ok());
///
/// let invalid_taxon = ParserDB::try_from("CREATE TABLE taxon (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_taxon).is_err());
/// ```
pub struct PluralTableName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for PluralTableName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<PluralTableName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: PluralTableName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for PluralTableName<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let table_name = context.table_name();
        let last_segment = table_name.split('_').next_back().unwrap_or(table_name);
        let inflector = Inflector::default();
        let expected_plural = inflector.pluralize(last_segment);

        let expected_name = if table_name.contains('_') {
            let prefix = &table_name[..table_name.rfind('_').unwrap()];
            format!("{}_{}", prefix, &expected_plural)
        } else {
            expected_plural.clone()
        };

        ConstraintErrorInfo::new()
            .constraint("PluralTableName")
            .unwrap()
            .object(table_name.to_owned())
            .unwrap()
            .message(format!(
                "Table '{table_name}' violates plural naming convention: the last segment '{last_segment}' is singular, not plural"
            ))
            .unwrap()
            .resolution(format!(
                "Change '{table_name}' to '{expected_name}' (pluralize the last segment from '{last_segment}' to '{expected_plural}')"
            ))
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_table(
        &self,
        _database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), crate::error::Error> {
        let table_name = table.table_name();
        let last_segment = table_name.split('_').next_back().unwrap_or(table_name);
        let inflector = Inflector::default();

        // Check if the last segment is plural by verifying that pluralizing it doesn't
        // change it
        let pluralized = inflector.pluralize(last_segment);

        if pluralized == last_segment {
            Ok(())
        } else {
            Err(crate::error::Error::Table(self.table_error_information(table)))
        }
    }
}
