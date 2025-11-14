//! Submodule providing the `SingularColumnName` constraint, which enforces
//! that the last segment of column names is singular.

use common_traits::builder::Builder;
use inflector::Inflector;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{ColumnConstraint, Constrainer, GenericConstrainer},
};

/// Struct defining a constraint that enforces that the last segment of column
/// names is singular.
///
/// For column names with underscores (e.g., `user_account`), only the last
/// segment after the final underscore is checked for singularity.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `SingularColumnName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = SingularColumnName::default().into();
///
/// // Invalid plural column names
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (users INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let invalid_schema2 = ParserDB::try_from("CREATE TABLE mytable (user_accounts INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err());
///
/// // Valid singular column names
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (user INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// let valid_schema2 = ParserDB::try_from("CREATE TABLE mytable (user_account INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
///
/// // Edge cases with Latin singulars
/// let valid_spectrum = ParserDB::try_from("CREATE TABLE mytable (spectrum INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_spectrum).is_ok());
///
/// let invalid_spectra = ParserDB::try_from("CREATE TABLE mytable (spectra INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_spectra).is_err());
///
/// let valid_matrix = ParserDB::try_from("CREATE TABLE mytable (matrix INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_matrix).is_ok());
///
/// let invalid_matrices = ParserDB::try_from("CREATE TABLE mytable (matrices INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_matrices).is_err());
///
/// let valid_taxon = ParserDB::try_from("CREATE TABLE mytable (taxon INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_taxon).is_ok());
///
/// let invalid_taxa = ParserDB::try_from("CREATE TABLE mytable (taxa INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_taxa).is_err());
/// ```
pub struct SingularColumnName<C>(std::marker::PhantomData<C>);

impl<C> Default for SingularColumnName<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<SingularColumnName<DB::Column>> for GenericConstrainer<DB> {
    fn from(constraint: SingularColumnName<DB::Column>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_constraint(Box::new(constraint));
        constrainer
    }
}

impl<C: ColumnLike> ColumnConstraint for SingularColumnName<C> {
    type Column = C;

    fn column_error_information(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let table = column.table(database);
        let table_name = table.table_name();
        let column_name = column.column_name();
        let last_segment = column_name.split('_').next_back().unwrap_or(column_name);
        let inflector = Inflector::default();
        let expected_singular = inflector.singularize(last_segment);

        let expected_name = if column_name.contains('_') {
            let prefix = &column_name[..column_name.rfind('_').unwrap()];
            format!("{}_{}", prefix, &expected_singular)
        } else {
            expected_singular.clone()
        };

        ConstraintErrorInfo::new()
            .constraint("SingularColumnName")
            .unwrap()
            .object(format!("{}.{}", table_name, column_name))
            .unwrap()
            .message(format!(
                "Column '{column_name}' in table '{table_name}' violates singular naming convention: the last segment '{last_segment}' is plural, not singular"
            ))
            .unwrap()
            .resolution(format!(
                "Change '{column_name}' to '{expected_name}' in table '{table_name}' (singularize the last segment from '{last_segment}' to '{expected_singular}')"
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
        let column_name = column.column_name();
        let last_segment = column_name.split('_').next_back().unwrap_or(column_name);
        let inflector = Inflector::default();

        // Check if the last segment is singular by verifying that singularizing it
        // doesn't change it
        let singularized = inflector.singularize(last_segment);

        if singularized == last_segment {
            Ok(())
        } else {
            Err(crate::error::Error::Column(self.column_error_information(database, column)))
        }
    }
}
