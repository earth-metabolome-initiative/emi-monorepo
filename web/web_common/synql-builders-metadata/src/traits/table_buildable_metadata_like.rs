//! Submodule providing the `TableBuildableMetadataLike` trait for SynQL table
//! buildables metadata.

use synql_core::{
    prelude::Builder,
    structs::{DataVariantRef, Documentation, InternalAttribute, InternalDataRef, Workspace},
};
use synql_insertable::traits::TableInsertableLike;

/// Trait representing metadata for a SynQL table buildable.
pub trait TableBuildableMetadataLike: TableInsertableLike {
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

    /// Returns the reference to the [`InternalDataRef`] for the table
    /// buildable, if it was already registered in the provided workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    fn builder_ref(&self, workspace: &Workspace) -> Option<InternalDataRef> {
        let crate_ref = workspace.internal_crate(&self.table_buildable_crate_name())?;
        Some(InternalDataRef::new(
            crate_ref,
            crate_ref.internal_data(&self.table_buildable_name())?,
        ))
    }
}

impl<T: TableInsertableLike> TableBuildableMetadataLike for T {}
