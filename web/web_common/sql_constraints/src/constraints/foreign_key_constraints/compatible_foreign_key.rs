//! Submodule providing the `CompatibleForeignKey` constraint, which enforces
//! that foreign key names, when defined, must have columns which are compatible
//! with the referenced columns, i.e. have the same data type and they are from
//! which are part the same extension hierarchy.

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, ForeignKeyConstraint, GenericConstrainer},
};

/// Struct defining a constraint that enforces that foreign key names are
/// compatible with the referenced columns, i.e. have the same data type and
/// they are from the same extension hierarchy.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `CompatibleForeignKey` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = CompatibleForeignKey::default().into();
///
/// let invalid_data_type = ParserDB::from_sql(
///     r#"
/// CREATE TABLE mytable (id INT PRIMARY KEY);
/// CREATE TABLE othertable (id SMALLINT, CONSTRAINT fk FOREIGN KEY (id) REFERENCES mytable (id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_data_type).is_err());
///
/// let invalid_extension_dag = ParserDB::from_sql(
///     r#"
/// CREATE TABLE left_root (id INT PRIMARY KEY);
/// CREATE TABLE right_root (id INT PRIMARY KEY);
/// CREATE TABLE left_child (id INT PRIMARY KEY REFERENCES left_root (id));
/// CREATE TABLE right_child (id INT PRIMARY KEY REFERENCES right_root (id));
/// CREATE TABLE mytable (
///     id INT PRIMARY KEY,
///     other_id INT,
///     FOREIGN KEY (other_id) REFERENCES left_child (id),
///     FOREIGN KEY (other_id) REFERENCES right_child (id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_extension_dag).is_ok());
///
/// let valid_schema2 = ParserDB::from_sql(r#"
/// CREATE TABLE root (id INT PRIMARY KEY);
/// CREATE TABLE child1 (id INT PRIMARY KEY REFERENCES root (id));
/// CREATE TABLE mytable (id INT PRIMARY KEY, other_id INT, CONSTRAINT fk FOREIGN KEY (other_id) REFERENCES child1 (id));
/// "#).unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
/// ```
pub struct CompatibleForeignKey<C>(std::marker::PhantomData<C>);

impl<C> Default for CompatibleForeignKey<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<CompatibleForeignKey<DB>> for GenericConstrainer<DB> {
    fn from(constraint: CompatibleForeignKey<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_foreign_key_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ForeignKeyConstraint for CompatibleForeignKey<DB> {
    type ForeignKey = DB::ForeignKey;
    type Database = DB;
    type Table = DB::Table;

    fn validate_foreign_key(
        &self,
        database: &Self::Database,
        foreign_key: &Self::ForeignKey,
    ) -> Result<(), crate::prelude::Error> {
        let host_table = foreign_key.host_table(database);
        let referenced_table = foreign_key.referenced_table(database);
        for (host_column, referenced_column) in foreign_key
            .host_columns(database)
            .zip(foreign_key.referenced_columns(database))
        {
            if !host_table.is_compatible_with(
                database,
                &host_column,
                referenced_table,
                &referenced_column,
            ) {
                return Err(crate::error::Error::ForeignKey(
                    ConstraintErrorInfo::new()
                        .constraint("CompatibleForeignKey")
                        .unwrap()
                        .object(foreign_key.foreign_key_name().unwrap_or("Unnamed foreign key").to_owned())
                        .unwrap()
                        .message(format!(
                            "Foreign key column `{}.{}` is not compatible with referenced column `{}.{}`",
                            host_table.table_name(),
                            host_column.column_name(),
                            referenced_table.table_name(),
                            referenced_column.column_name(),
                        ))
                        .unwrap()
                        .resolution("Ensure that the foreign key column is compatible with the referenced column".to_string())
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
