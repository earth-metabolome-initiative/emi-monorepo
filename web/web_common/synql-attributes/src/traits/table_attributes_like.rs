//! Submodule providing the `TableAttributes` trait for SynQL table attributes.

use syn::Ident;
use synql_core::{
    structs::{InternalDataRef, Workspace},
    traits::TableSynLike,
};
use synql_diesel_schema::traits::TableSchema;

use crate::structs::TableAttributes;

/// Name of the module containing the attributes for a table.
pub const ATTRIBUTES_MODULE_NAME: &str = "attributes";

/// Trait representing a SynQL table attributes.
pub trait TableAttributesLike: TableSchema {
    /// Returns the name of the crate which will contain the attributes enum
    /// for the table.
    #[inline]
    fn table_attributes_crate_name(&self) -> String {
        format!("{}_attributes", self.table_singular_snake_name())
    }

    /// Returns the name of the attributes enum for the table.
    #[inline]
    fn table_attributes_name(&self) -> String {
        format!("{}Attribute", self.table_singular_camel_name())
    }

    /// Returns the ident of the attributes enum for the table.
    #[inline]
    fn table_attributes_ident(&self) -> Ident {
        Ident::new(&self.table_attributes_name(), proc_macro2::Span::call_site())
    }

    /// Returns the
    /// [`TableAttributes<'data, 'table,
    /// Self>`](crate::structs::TableAttributes) representing the attributes
    /// for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   attributes.
    #[inline]
    fn attributes<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableAttributes<'table, Self> {
        TableAttributes::new(self, workspace, database)
    }

    /// Returns a reference to the attributes enum for the table.
    fn attributes_ref(&self, workspace: &Workspace) -> Option<InternalDataRef> {
        let crate_ref = workspace.internal_crate(&self.table_attributes_crate_name())?;
        Some(InternalDataRef::new(
            crate_ref,
            crate_ref.internal_data(&self.table_attributes_name())?,
        ))
    }
}

impl<T: TableSynLike> TableAttributesLike for T {}
