//! Submodule providing the `TableExtensionAttributesLike` trait for SynQL table
//! extension attributes.

use syn::Ident;
use synql_core::{
    structs::{InternalDataRef, Workspace},
    traits::TableSynLike,
};
use synql_diesel_schema::traits::TableSchema;

use crate::structs::TableExtensionAttributes;

/// Name of the module containing the extension attributes for a table.
pub const EXTENSION_ATTRIBUTES_MODULE_NAME: &str = "extension_attributes";
/// Name of the extension attributes enum for a table.
pub const EXTENSION_ATTRIBUTES_ENUM_NAME: &str = "Extension";

/// Trait representing a SynQL table extension attributes.
pub trait TableExtensionAttributesLike: TableSchema {
    /// Returns the name of the crate which will contain the extension
    /// attributes enum for the table.
    fn table_extension_attributes_crate_name(&self) -> String {
        format!("{}_extension_attributes", self.table_singular_snake_name())
    }

    /// Returns the name of the extension attributes enum for the table.
    fn table_extension_attributes_name(&self) -> String {
        format!("{}ExtensionAttribute", self.table_singular_camel_name())
    }

    /// Returns the ident of the extension attributes enum for the table.
    fn table_extension_attributes_ident(&self) -> Ident {
        Ident::new(&self.table_extension_attributes_name(), proc_macro2::Span::call_site())
    }

    /// Returns the
    /// [`TableExtensionAttributes<'data, 'table,
    /// Self>`](crate::structs::TableExtensionAttributes) representing the
    /// extension attributes for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   extension attributes.
    fn extension_attributes<'table, 'data>(
        &'table self,
        workspace: &'table Workspace<'data>,
        database: &'table Self::DB,
    ) -> Option<TableExtensionAttributes<'data, 'table, Self>>
    where
        Self: 'data,
    {
        if self.is_extension(database) {
            Some(TableExtensionAttributes::new(self, workspace, database))
        } else {
            None
        }
    }

    /// Returns a reference to the extension attributes enum for the table.
    fn extension_attributes_enum_ref<'data>(
        &self,
        workspace: &Workspace<'data>,
    ) -> Option<InternalDataRef<'data>> {
        let crate_ref = workspace.internal_crate(&self.table_extension_attributes_crate_name())?;
        Some(InternalDataRef::new(
            crate_ref,
            crate_ref.internal_data(&self.table_extension_attributes_name())?,
        ))
    }
}

impl<T: TableSynLike> TableExtensionAttributesLike for T {}
