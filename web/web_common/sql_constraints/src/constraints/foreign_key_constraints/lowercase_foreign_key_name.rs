//! Submodule providing the `LowercaseForeignKeyName` constraint, which enforces
//! that foreign key names, when defined, are lowercase.

use common_traits::builder::Builder;
use sql_traits::traits::{DatabaseLike, ForeignKeyLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, ForeignKeyConstraint, GenericConstrainer},
};

/// Struct defining a constraint that enforces that foreign key names are
/// lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `LowercaseForeignKeyName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = LowercaseForeignKeyName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (id INT, CONSTRAINT Fk FOREIGN KEY (id) REFERENCES other_table (id));").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema1 = ParserDB::try_from("CREATE TABLE mytable (id INT, CONSTRAINT fk FOREIGN KEY (id) REFERENCES other_table (id));").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema1).is_ok());
///
/// let valid_schema2 = ParserDB::try_from("CREATE TABLE mytable (id INT, FOREIGN KEY (id) REFERENCES other_table (id));").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
/// ```
pub struct LowercaseForeignKeyName<C>(std::marker::PhantomData<C>);

impl<C> Default for LowercaseForeignKeyName<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<LowercaseForeignKeyName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: LowercaseForeignKeyName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_foreign_key_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ForeignKeyConstraint for LowercaseForeignKeyName<DB> {
    type Database = DB;

    fn validate_foreign_key(
        &self,
        _database: &Self::Database,
        foreign_key: &<Self::Database as DatabaseLike>::ForeignKey,
    ) -> Result<(), crate::prelude::Error> {
        if let Some(name) = foreign_key.foreign_key_name()
            && name.chars().any(char::is_uppercase)
        {
            return Err(crate::error::Error::ForeignKey(
                ConstraintErrorInfo::new()
                    .constraint("LowercaseForeignKeyName")
                    .unwrap()
                    .object(foreign_key.foreign_key_name().unwrap().to_owned())
                    .unwrap()
                    .message(format!(
                        "Foreign key name '{}' is not lowercase",
                        foreign_key.foreign_key_name().unwrap()
                    ))
                    .unwrap()
                    .resolution("Rename the foreign key to be all lowercase".to_string())
                    .unwrap()
                    .build()
                    .unwrap()
                    .into(),
            ));
        }
        Ok(())
    }
}
