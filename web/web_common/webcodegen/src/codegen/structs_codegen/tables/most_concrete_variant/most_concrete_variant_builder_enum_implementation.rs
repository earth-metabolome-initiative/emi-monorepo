//! Submodule generating the enum implementation for the most concrete variant
//! builder of a table DAG.

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

        let setter_trait = table.setter_trait_ty().unwrap();
        let mut from_variant_impls = Vec::new();
        let mut variants_with_documentation = Vec::new();
        let mut variants = Vec::new();
        let mut builder_types = Vec::new();
        let mut involved_primary_keys = Vec::new();
        let dag_ty = table.dag_ty(conn)?.unwrap();
        let builder_dag_ident: proc_macro2::Ident = table.builder_dag_ident(conn)?.unwrap();
        let insert_error_dag_ty = table.insert_error_dag_ty(conn)?.unwrap();
        for descendant in descendants {
            let variant_ident = descendant.struct_ident()?;
            let builder_variant_path = descendant.insertable_builder_ty()?;
            builder_types.push(builder_variant_path.clone());
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

                impl From<#builder_dag_ident> for Option<#builder_variant_path> {
                    fn from(value: #builder_dag_ident) -> Self {
                        match value {
                            #builder_dag_ident::#variant_ident(v) => Some(v),
                            _ => None,
                        }
                    }
                }
            });
            variants.push(variant_ident);
        }

        let (_, methods) = table.generate_builder_method_impls_for_ancestral_table(
            conn,
            table,
            self.check_constraints_extensions.as_slice(),
            |_conn, insertable_column, _check_constraints_extensions, _extension_table_traits| {
                let setter_ident = insertable_column.getter_ident()?;
                let snake_case_ident = insertable_column.snake_case_ident()?;
                let body = quote! {
                    Ok(match self {
                        #(Self::#variants(builder) => <#builder_types as #setter_trait>::#setter_ident(builder, #snake_case_ident)?.into(),)*
                    })
                };
                Ok((
                    true,
                    false,
                    false,
                    vec![insertable_column.clone()],
                    body
                ))
            },
        )?;

        let documentation = format!("Enum representing the `{table_name}` table builder DAG.");

        Ok(quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[doc = #documentation]
            pub enum #builder_dag_ident {
                #(#variants_with_documentation),*
            }

            impl #builder_dag_ident {
                /// Returns the type name of the variant contained within the enum.
                pub fn type_name(&self) -> &'static str {
                    match self {
                        #(Self::#variants(_) => std::any::type_name::<#builder_types>()),*
                    }
                }
            }

            impl common_traits::builder::IsCompleteBuilder for #builder_dag_ident
            {
                fn is_complete(&self) -> bool {
                    match self {
                        #(Self::#variants(builder) => builder.is_complete()),*
                    }
                }
            }

            impl web_common_traits::database::DispatchableInsertVariantMetadata for #builder_dag_ident {
                type Row = #dag_ty;
                type Error = #insert_error_dag_ty;
            }

            impl<C> web_common_traits::database::DispatchableInsertableVariant<C> for #builder_dag_ident
                where
                    #(#builder_types: web_common_traits::database::DispatchableInsertableVariant<C>),*
            {
                fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
                    Ok(match self {
                        #( Self::#variants(variant) => variant.insert(user_id, conn)?.into(), )*
                    })
                }
            }

            impl #setter_trait for #builder_dag_ident
                where
                    #(#builder_types: #setter_trait),*
            {
                type Error = #insert_error_dag_ty;

                #(#methods)*
            }

            #(#from_variant_impls)*
        })
    }
}
