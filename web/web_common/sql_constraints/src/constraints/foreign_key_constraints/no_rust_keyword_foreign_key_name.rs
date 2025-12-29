//! Submodule providing the `NoRustKeywordForeignKeyName` constraint, which
//! enforces that foreign key names are not Rust keywords.

use common_traits::builder::Builder;
use sql_traits::traits::{DatabaseLike, ForeignKeyLike};

use crate::{
    constraints::rust_keywords::is_rust_keyword,
    error::ConstraintErrorInfo,
    traits::{Constrainer, ForeignKeyConstraint, GenericConstrainer},
};

/// Struct defining a constraint that enforces that foreign key names are not
/// Rust keywords.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `NoRustKeywordForeignKeyName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NoRustKeywordForeignKeyName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE other_table (id INT); CREATE TABLE mytable (id INT, CONSTRAINT struct FOREIGN KEY (id) REFERENCES other_table (id));").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE other_table (id INT); CREATE TABLE mytable (id INT, CONSTRAINT my_struct FOREIGN KEY (id) REFERENCES other_table (id));").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct NoRustKeywordForeignKeyName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for NoRustKeywordForeignKeyName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NoRustKeywordForeignKeyName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: NoRustKeywordForeignKeyName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_foreign_key_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ForeignKeyConstraint for NoRustKeywordForeignKeyName<DB> {
    type Database = DB;

    fn validate_foreign_key(
        &self,
        _database: &Self::Database,
        foreign_key: &<Self::Database as DatabaseLike>::ForeignKey,
    ) -> Result<(), crate::error::Error> {
        if let Some(name) = foreign_key.foreign_key_name() {
            if is_rust_keyword(name) {
                return Err(crate::error::Error::ForeignKey(
                    ConstraintErrorInfo::new()
                        .constraint("NoRustKeywordForeignKeyName")
                        .unwrap()
                        .object(name.to_owned())
                        .unwrap()
                        .message(format!("Foreign key name '{}' is a Rust keyword.", name))
                        .unwrap()
                        .resolution(format!(
                            "Rename the foreign key '{}' to something that is not a Rust keyword.",
                            name
                        ))
                        .unwrap()
                        .build()
                        .unwrap()
                        .into(),
                ));
            }
        }
        Ok(())
    }
}
