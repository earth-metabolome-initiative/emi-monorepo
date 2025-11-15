//! Submodule providing the `TableTransitiveExtensionLike` trait for SynQL table
//! models.

use syn::Ident;
use synql_core::structs::{TraitVariantRef, Workspace};
use synql_models::traits::TableModelLike;

use crate::structs::TableTransitiveExtension;

/// Name of the module containing the transitive extension traits for a table.
pub const TRAIT_MODULE_NAME: &str = "transitive_extension";

/// Trait representing a SynQL table model with transitive extension support.
pub trait TableTransitiveExtensionLike: TableModelLike {
    /// Returns the name of the crate which will contain the transitive
    /// extension traits for tables that extend this table.
    fn table_transitive_extension_crate_name(&self) -> String {
        format!("{}_transitive_extension", self.table_singular_snake_name())
    }

    /// Returns the trait name for the transitive extension trait.
    /// This is the trait that other tables will implement if they extend this
    /// table.
    fn table_transitive_extension_trait_name(&self) -> String {
        format!("ExtensionOf{}", self.table_singular_camel_name())
    }

    /// Returns the trait ident for the transitive extension trait.
    fn table_transitive_extension_trait_ident(&self) -> Ident {
        Ident::new(&self.table_transitive_extension_trait_name(), proc_macro2::Span::call_site())
    }

    /// Returns the
    /// [`TableTransitiveExtension`](crate::structs::TableTransitiveExtension)
    /// struct for generating the transitive extension traits.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    fn transitive_extension<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableTransitiveExtension<'table, Self> {
        TableTransitiveExtension::new(self, workspace, database)
    }

    /// Returns a reference to the transitive extension trait for the table.
    fn transitive_extension_trait_ref(&self, workspace: &Workspace) -> Option<TraitVariantRef> {
        let crate_ref = workspace.internal_crate(&self.table_transitive_extension_crate_name())?;
        Some(TraitVariantRef::Internal(
            crate_ref.internal_trait(&self.table_transitive_extension_trait_name())?.clone(),
            Some(crate_ref.clone()),
        ))
    }
}

impl<T: TableModelLike> TableTransitiveExtensionLike for T {}
