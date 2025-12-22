//! Submodule providing the `PrimaryKeyReferenceEndsWithId` constraint, which
//! enforces that foreign key host columns that reference primary key columns
//! (directly or indirectly) must have names ending with "id".

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, ForeignKeyConstraint, GenericConstrainer},
};

/// Struct defining a constraint that enforces that foreign key host columns
/// that reference primary key columns (directly or indirectly) must have names
/// ending with "id".
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `PrimaryKeyReferenceEndsWithId` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = PrimaryKeyReferenceEndsWithId::default().into();
///
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent (id INT PRIMARY KEY);
/// CREATE TABLE child (parent_key INT, FOREIGN KEY (parent_key) REFERENCES parent (id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema_direct = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent (id INT PRIMARY KEY);
/// CREATE TABLE child (parent_id INT, FOREIGN KEY (parent_id) REFERENCES parent (id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema_direct).is_ok());
///
/// let valid_schema_indirect = ParserDB::try_from(
///     r#"
/// CREATE TABLE grandparent (id INT PRIMARY KEY);
/// CREATE TABLE parent (gp_id INT, FOREIGN KEY (gp_id) REFERENCES grandparent (id));
/// CREATE TABLE child (parent_id INT, FOREIGN KEY (parent_id) REFERENCES parent (gp_id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema_indirect).is_ok());
///
/// let valid_schema_non_pk = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent (id INT PRIMARY KEY, code TEXT UNIQUE);
/// CREATE TABLE child (parent_code TEXT, FOREIGN KEY (parent_code) REFERENCES parent (code));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema_non_pk).is_ok());
/// ```
pub struct PrimaryKeyReferenceEndsWithId<C>(std::marker::PhantomData<C>);

impl<C> Default for PrimaryKeyReferenceEndsWithId<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<PrimaryKeyReferenceEndsWithId<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: PrimaryKeyReferenceEndsWithId<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_foreign_key_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ForeignKeyConstraint for PrimaryKeyReferenceEndsWithId<DB> {
    type Database = DB;

    fn validate_foreign_key(
        &self,
        database: &Self::Database,
        foreign_key: &<Self::Database as DatabaseLike>::ForeignKey,
    ) -> Result<(), crate::prelude::Error> {
        let host_table = foreign_key.host_table(database);
        let referenced_table = foreign_key.referenced_table(database);

        for (host_column, referenced_column) in
            foreign_key.host_columns(database).zip(foreign_key.referenced_columns(database))
        {
            // Check if the referenced column is a primary key or references a primary key
            let references_primary_key =
                if referenced_table.is_primary_key_column(database, referenced_column) {
                    true
                } else {
                    // Check if the referenced column itself references a primary key through a
                    // foreign key
                    referenced_table.foreign_keys(database).any(|fk| {
                        fk.host_columns(database).any(|fk_host_col| {
                            fk_host_col.column_name() == referenced_column.column_name()
                                && fk.referenced_columns(database).any(|fk_ref_col| {
                                    let fk_ref_table = fk.referenced_table(database);
                                    fk_ref_table.is_primary_key_column(database, fk_ref_col)
                                })
                        })
                    })
                };

            if references_primary_key && !host_column.column_name().ends_with("id") {
                return Err(crate::error::Error::ForeignKey(
                    ConstraintErrorInfo::new()
                        .constraint("PrimaryKeyReferenceEndsWithId")
                        .unwrap()
                        .object(
                            foreign_key
                                .foreign_key_name()
                                .unwrap_or("Unnamed foreign key")
                                .to_owned(),
                        )
                        .unwrap()
                        .message(format!(
                            "Foreign key host column '{}.{}' references a primary key column '{}.{}' but does not end with 'id'",
                            host_table.table_name(),
                            host_column.column_name(),
                            referenced_table.table_name(),
                            referenced_column.column_name(),
                        ))
                        .unwrap()
                        .resolution(format!(
                            "Rename column '{}.{}' to end with 'id' (e.g., '{}_id')",
                            host_table.table_name(),
                            host_column.column_name(),
                            host_column.column_name(),
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
