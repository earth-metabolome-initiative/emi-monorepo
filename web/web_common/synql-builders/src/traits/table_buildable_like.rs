//! Submodule providing the `TableBuildable` trait for SynQL table buildables.

use synql_core::{
    prelude::Builder,
    structs::{DataVariantRef, Documentation, InternalAttribute, Workspace},
};
use synql_insertable::traits::TableInsertableLike;

use crate::structs::TableBuildable;

/// Name of the module containing the buildable for a table.
pub const BUILDABLE_MODULE_NAME: &str = "buildable";

/// Trait representing a SynQL table buildable.
pub trait TableBuildableLike: TableInsertableLike {
    /// Returns the crate name which contains the buildable struct, and
    /// associated traits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE users (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "users").unwrap();
    /// assert_eq!(table.table_buildable_crate_name(), "user_builders");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn table_buildable_crate_name(&self) -> String {
        format!("{}_builders", self.table_singular_snake_name())
    }

    /// Returns the name of the buildable struct for the table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE users (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "users").unwrap();
    /// assert_eq!(table.table_buildable_name(), "UserBuilder");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn table_buildable_name(&self) -> String {
        format!("{}Builder", self.table_singular_camel_name())
    }

    /// Returns the attribute for an extension field of a builder
    /// when the table is extended by another table.
    fn builder_extension_attribute<'data>(&self, workspace: &Workspace) -> InternalAttribute {
        let schema_crate_ref = self
            .table_schema_ref(workspace)
            .expect("Failed to get the table schema ref for the buildable module");

        InternalAttribute::new()
            .private()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Extension field to the ancestral {} table.",
                        self.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .ty(DataVariantRef::generic_str(&self.table_singular_camel_name()))
            .name(self.table_singular_snake_name())
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns the attribute for an insertable field of a builder
    /// for the underlying insertable object.
    fn insertable_attribute<'data>(&self, workspace: &Workspace) -> InternalAttribute {
        let insertable_ref = self
            .insertable_data_ref(workspace)
            .expect("Failed to get the table schema ref for the buildable module");

        InternalAttribute::new()
            .private()
            .documentation(
                Documentation::new()
                    .documentation("Underlying insertable field.")
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .ty(insertable_ref)
            .name(self.table_singular_snake_name())
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns the [`TableBuildable`](crate::structs::TableBuildable)
    /// representing the buildable for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   buildable.
    #[inline]
    fn buildable<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableBuildable<'table, Self> {
        TableBuildable::new(self, workspace, database)
    }
}

impl<T: TableInsertableLike> TableBuildableLike for T {}
