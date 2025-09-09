//! Submodule generating the enum implementation for the most concrete variant
//! of a table DAG.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Table, traits::TableLike};

impl Codegen<'_> {
    /// Generate implementations of the builder structs associated with the
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
    pub(super) fn generate_most_concrete_variant_builder_enum_implementation(
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
        let mut variants_with_documentation = Vec::new();
        let mut most_concrete_variants = Vec::new();
        let mut where_requirements = Vec::new();
        let mut involved_primary_keys = Vec::new();
        let builder_dag_ident: proc_macro2::Ident = table.builder_dag_ident(conn)?.unwrap();
        for descendant in descendants {
            let variant_ident = descendant.struct_ident()?;
            let builder_variant_path = descendant.insertable_builder_ty()?;
            let table_name = descendant.snake_case_name()?;
            let documentation = format!("Builder for the `{table_name}` table.");
            involved_primary_keys.extend(descendant.primary_key_columns(conn)?.iter().cloned());
            variants_with_documentation.push(quote! {
                #[doc = #documentation]
                #variant_ident(#builder_variant_path)
            });
            from_variant_impls.push(quote! {
                impl From<#builder_variant_path> for #builder_dag_ident {
                    fn from(value: #builder_variant_path) -> Self {
                        #builder_dag_ident::#variant_ident(value)
                    }
                }
            });
            most_concrete_variants.push(if descendant == table {
                quote! {
                    #table_name => self.clone().into(),
                }
            } else {
                where_requirements.push(quote! {
                    #builder_variant_path: web_common_traits::database::Read<C>
                });
                quote! {
                    #table_name => <#builder_variant_path as web_common_traits::database::Read<C>>::read(*self.id(), conn)?.into(),
                }
            });
        }

        let documentation = format!("Enum representing the `{table_name}` table builder DAG.");

        Ok(quote! {
            #[derive(Debug, Clone, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[doc = #documentation]
            pub enum #builder_dag_ident {
                #(#variants_with_documentation),*
            }

            #(#from_variant_impls)*
        })
    }
}
