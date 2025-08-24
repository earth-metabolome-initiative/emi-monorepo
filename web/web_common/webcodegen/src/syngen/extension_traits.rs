//! Submodule implementing the `ExtensionTraits` trait for `ProcedureModel`.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use crate::traits::TableLike;

use crate::errors::WebCodeGenError;

impl crate::Table {
    /// Creates the extension trait implementations for the table, if it is
    /// extending tables.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    pub fn extension_traits_impls(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<TokenStream>, WebCodeGenError> {
        let table_ident = self.struct_ident()?;
        let primary_key_type = self.primary_key_type(conn)?;

        let mut extension_traits = self.ancestral_extension_tables(conn)?
            .iter()
            .map(|extended_table| {
                let extended_table_path = extended_table.import_struct_path()?;

                Ok(quote! {
                    impl web_common_traits::prelude::ExtensionTable<#extended_table_path> for #table_ident where for<'a> &'a Self: diesel::Identifiable<Id=&'a #primary_key_type> {}
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        if self.is_extended(conn)? {
            extension_traits.push(quote! {
                impl web_common_traits::prelude::ExtensionTable<Self> for #table_ident where for<'a> &'a Self: diesel::Identifiable<Id=&'a #primary_key_type> {}
            });
        }

        Ok(extension_traits)
    }
}
