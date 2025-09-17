//! Submodule generating the enum implementation for the most concrete variant
//! insert errors of a table DAG.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Table, traits::TableLike};

impl Codegen<'_> {
    /// Generate implementations of the insert errors associated with the
    /// [`MostConcreteVariant`] and [`MostConcreteVariant`]-adjacent traits for
    /// the provided tables.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which to generate the code.
    /// * `most_concrete_table_column` - The column representing the most
    ///   concrete table.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    pub(super) fn generate_most_concrete_variant_insert_error_enum_implementation(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, crate::errors::WebCodeGenError> {
        let table_name = table.snake_case_name()?;
        let extension_network = self.table_extension_network().unwrap();
        let mut descendants = extension_network.descendants(table);
        descendants.push(table);
        descendants.sort_unstable();

        let mut from_variant_impls = Vec::new();
        let mut variants = Vec::new();
        let mut variants_with_documentation = Vec::new();
        let insert_error_dag_ident: proc_macro2::Ident =
            table.insert_error_dag_ident(conn)?.unwrap();
        for descendant in descendants {
            let variant_ident = descendant.struct_ident()?;
            let attribute = descendant.insertable_enum_ty()?;
            let table_name = descendant.snake_case_name()?;
            let documentation = format!("Insert error associated with the `{table_name}` table.");
            variants_with_documentation.push(quote! {
                #[doc = #documentation]
                #variant_ident(web_common_traits::database::InsertError<#attribute>)
            });
            from_variant_impls.push(quote! {
                impl From<web_common_traits::database::InsertError<#attribute>> for #insert_error_dag_ident {
                    fn from(error: web_common_traits::database::InsertError<#attribute>) -> Self {
                        #insert_error_dag_ident::#variant_ident(error)
                    }
                }
            });
            variants.push(variant_ident);
        }

        let documentation = format!(
            "Enum representing insert errors which may occur when inserting an entry from the `{table_name}` table DAG."
        );

        Ok(quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[doc = #documentation]
            pub enum #insert_error_dag_ident {
                #(#variants_with_documentation),*
            }

            impl std::fmt::Display for #insert_error_dag_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(Self::#variants(e) => write!(f, "{e}")),*
                    }
                }
            }

            impl std::error::Error for #insert_error_dag_ident {
                fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                    match self {
                        #(Self::#variants(e) => Some(e)),*,
                    }
                }
            }

            #(#from_variant_impls)*
        })
    }
}
