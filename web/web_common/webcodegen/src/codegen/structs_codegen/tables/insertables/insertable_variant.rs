//! Submodule handling the generation of insertable variants and relative traits
//! implementation.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;
use crate::traits::TableLike;

use crate::{
    Table,
    codegen::{
        CODEGEN_DIRECTORY, CODEGEN_INSERTABLES_PATH, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
    },
    errors::WebCodeGenError,
};

impl Table {
    /// Returns the name for the insertable variant.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant cannot be retrieved.
    pub fn insertable_variant_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("Insertable{}", self.struct_name()?))
    }

    /// Returns the [`Type`](syn::Type) for the insertable variant.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant cannot be retrieved.
    pub fn insertable_variant_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.insertable_variant_name()?
        ))?)
    }

    /// Returns the [`Ident`](syn::Ident) for the insertable variant.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant cannot be retrieved.
    pub fn insertable_variant_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.insertable_variant_name()?, proc_macro2::Span::call_site()))
    }

    pub(super) fn insertable_variant_definition(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let insertable_variant_ident = self.insertable_variant_ident()?;
        let insertable_columns = self.insertable_columns(conn, true)?;
        let table_diesel_ident = self.import_diesel_path()?;

        let insertable_attributes = insertable_columns
            .iter()
            .map(|column| {
                let column_name = column.snake_case_ident()?;
                let column_type = column.rust_data_type(conn)?;
                Ok(quote::quote! {
                    pub(crate) #column_name: #column_type
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let insertable_variant_methods = self.foreign_key_methods(conn)?;

        Ok(quote::quote! {
            #[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
            #[cfg_attr(any(feature = "postgres", feature = "sqlite"), diesel(table_name = #table_diesel_ident))]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #insertable_variant_ident {
                #(#insertable_attributes),*
            }

            impl #insertable_variant_ident {
                #insertable_variant_methods
            }
        })
    }
}
