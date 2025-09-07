//! Submodule generating the enum implementation for the most concrete variant
//! of a table DAG.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    Codegen, Column, Table,
    codegen::structs_codegen::tables::insertables::columns_to_mermaid_illustration,
    traits::TableLike,
};

impl Codegen<'_> {
    /// Generate implementations of the structs able to implement the
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
    pub(super) fn generate_most_concrete_variant_enum_implementation(
        &self,
        table: &Table,
        most_concrete_table_column: &Column,
        dag_ident: &proc_macro2::Ident,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, crate::errors::WebCodeGenError> {
        let struct_path = table.import_struct_path()?;
        let table_name = table.snake_case_name()?;
        let most_concrete_table_ident = most_concrete_table_column.snake_case_ident()?;
        let extension_network = self.table_extension_network().unwrap();
        let mut descendants = extension_network.descendants(table);
        descendants.push(table);
        descendants.sort_unstable();

        let mut from_variant_impls = Vec::new();
        let mut variants_with_documentation = Vec::new();
        let mut most_concrete_variants = Vec::new();
        let mut where_requirements = Vec::new();
        let mut involved_primary_keys = Vec::new();
        let primary_key_columns = table.primary_key_columns(conn)?;
        let primary_key_column = &primary_key_columns[0];
        for descendant in descendants {
            let variant_ident = descendant.struct_ident()?;
            let variant_path = descendant.import_struct_path()?;
            let table_name = descendant.snake_case_name()?;
            let documentation = format!("Variant representing the `{table_name}` table.");
            involved_primary_keys.extend(descendant.primary_key_columns(conn)?.iter().cloned());
            variants_with_documentation.push(quote! {
                #[doc = #documentation]
                #variant_ident(#variant_path)
            });
            from_variant_impls.push(quote! {
                impl From<#variant_path> for #dag_ident {
                    fn from(value: #variant_path) -> Self {
                        #dag_ident::#variant_ident(value)
                    }
                }
            });
            most_concrete_variants.push(if descendant == table {
                quote! {
                    #table_name => self.clone().into(),
                }
            } else {
                where_requirements.push(quote! {
                    #variant_path: web_common_traits::database::Read<C>
                });
                quote! {
                    #table_name => <#variant_path as web_common_traits::database::Read<C>>::read(*self.id(), conn)?.into(),
                }
            });
        }
        let mermaid = columns_to_mermaid_illustration(
            false,
            &involved_primary_keys,
            primary_key_column,
            conn,
        )?
        .to_string();

        let documentation = vec![
            format!("Enum representing the most concrete variant of the `{table_name}` table DAG."),
            "".to_owned(),
            "# Mermaid illustration of the DAG:".to_owned(),
            "```mermaid".to_owned(),
            mermaid,
            "```".to_owned(),
        ];

        Ok(quote! {
            #[derive(Debug, Clone, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #(#[doc = #documentation])*
            pub enum #dag_ident {
                #(#variants_with_documentation),*
            }

            #(#from_variant_impls)*

            impl<C> web_common_traits::database::MostConcreteVariant<C> for #struct_path
                where #(#where_requirements),*
            {
                type Variant = #dag_ident;

                fn most_concrete_variant(&self, conn: &mut C) -> Result<Self::Variant, diesel::result::Error> {
                    use diesel::Identifiable;
                    Ok(match self.#most_concrete_table_ident.as_str() {
                        #(#most_concrete_variants)*
                        _ => unreachable!("Database and codegen are out of sync.")
                    })
                }
            }
        })
    }
}
