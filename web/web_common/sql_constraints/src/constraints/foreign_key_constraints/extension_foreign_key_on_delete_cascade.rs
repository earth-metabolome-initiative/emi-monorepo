//! Submodule providing the `ExtensionForeignKeyOnDeleteCascade` constraint,
//! which enforces that extension foreign keys must have `ON DELETE CASCADE`
//! specified.

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, ForeignKeyConstraint, GenericConstrainer},
};

/// Struct defining a constraint that enforces that extension foreign keys
/// must have `ON DELETE CASCADE` specified.
///
/// An extension foreign key is a foreign key where:
/// - The host columns are the primary key of the host table
/// - The referenced columns are the primary key of the referenced table
/// - The foreign key is not self-referential
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `ExtensionForeignKeyOnDeleteCascade` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     ExtensionForeignKeyOnDeleteCascade::default().into();
///
/// // Extension FK without ON DELETE CASCADE - should fail
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY);
/// CREATE TABLE extension_table (
///     id INT PRIMARY KEY,
///     FOREIGN KEY (id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Extension FK with ON DELETE CASCADE - should pass
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY);
/// CREATE TABLE extension_table (
///     id INT PRIMARY KEY,
///     FOREIGN KEY (id) REFERENCES parent_table(id) ON DELETE CASCADE
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// // Non-extension FK without ON DELETE CASCADE - should pass (not an extension FK)
/// let valid_schema2 = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY);
/// CREATE TABLE reference_table (
///     id INT PRIMARY KEY,
///     parent_id INT,
///     FOREIGN KEY (parent_id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
/// ```
pub struct ExtensionForeignKeyOnDeleteCascade<C>(std::marker::PhantomData<C>);

impl<C> Default for ExtensionForeignKeyOnDeleteCascade<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<ExtensionForeignKeyOnDeleteCascade<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: ExtensionForeignKeyOnDeleteCascade<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_foreign_key_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ForeignKeyConstraint for ExtensionForeignKeyOnDeleteCascade<DB> {
    type Database = DB;

    fn validate_foreign_key(
        &self,
        database: &Self::Database,
        foreign_key: &<Self::Database as DatabaseLike>::ForeignKey,
    ) -> Result<(), crate::prelude::Error> {
        // Only check extension foreign keys
        if foreign_key.is_extension_foreign_key(database) {
            if !foreign_key.on_delete_cascade(database) {
                let host_table = foreign_key.host_table(database);
                let referenced_table = foreign_key.referenced_table(database);

                let fk_name =
                    foreign_key.foreign_key_name().map(|s| s.to_string()).unwrap_or_else(|| {
                        format!(
                            "{}.({}) -> {}.({}) ",
                            host_table.table_name(),
                            foreign_key
                                .host_columns(database)
                                .map(|c| c.column_name())
                                .collect::<Vec<_>>()
                                .join(", "),
                            referenced_table.table_name(),
                            foreign_key
                                .referenced_columns(database)
                                .map(|c| c.column_name())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    });

                return Err(crate::error::Error::ForeignKey(
                    ConstraintErrorInfo::new()
                        .constraint("ExtensionForeignKeyOnDeleteCascade")
                        .unwrap()
                        .object(fk_name.clone())
                        .unwrap()
                        .message(format!(
                            "Extension foreign key '{}' in table '{}' must have ON DELETE CASCADE",
                            fk_name,
                            host_table.table_name()
                        ))
                        .unwrap()
                        .resolution(
                            "Add ON DELETE CASCADE to the extension foreign key definition"
                                .to_string(),
                        )
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
