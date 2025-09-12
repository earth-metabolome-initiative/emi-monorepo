//! Submodule taking care of the definition of the insertable builder struct for
//! a table and the most basic traits.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

mod from_builder_to_id_or_builder_impl;
mod generate_builder_documentation;
mod is_complete_builder;
use crate::{Codegen, Column, Table, errors::WebCodeGenError, traits::TableLike};

impl Codegen<'_> {
    pub(super) fn generate_insertable_builder_definition(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let maybe_generics = self
            .table_extension_network()
            .unwrap()
            .generics_for_table_builder_definition(table, conn)?;
        let insertable_columns = table.insertable_columns(conn, true)?;
        let builder_ident = table.insertable_builder_ident()?;
        let maybe_impl_generics = self
            .table_extension_network()
            .unwrap()
            .generics_for_table_builder_implementation(table)?;
        let extension_network = self.table_extension_network().expect("Extension network exists");

        let has_sql_default_types = insertable_columns.iter().any(Column::has_default);
        let mut derives = vec![
            quote::quote!(Clone),
            quote::quote!(Debug),
            quote::quote!(PartialEq),
            quote::quote!(PartialOrd),
        ];

        if insertable_columns.iter().all(|c| c.supports_eq(conn).unwrap_or(false)) {
            derives.push(quote::quote!(Eq));
            derives.push(quote::quote!(Hash));
        }
        if insertable_columns.iter().all(|c| c.supports_ord(conn).unwrap_or(false)) {
            derives.push(quote::quote!(Ord));
        }

        if !has_sql_default_types {
            derives.push(quote::quote!(Default));
        }

        let insertable_builder_default_impl = if has_sql_default_types {
            let mut default_impl_attributes = Vec::new();
            let mut defalt_where_requirements = Vec::new();

            for insertable_column in &insertable_columns {
                let attribute_ident = if let Some(extension_foreign_key) =
                    insertable_column.is_part_of_extension_primary_key(conn)?
                {
                    let foreign_table = extension_foreign_key.foreign_table(conn)?;
                    let struct_ident = foreign_table.struct_ident()?;
                    defalt_where_requirements.push(quote::quote! {
                        #struct_ident: Default
                    });

                    extension_foreign_key.constraint_ident(conn)?
                } else {
                    insertable_column.snake_case_ident()?
                };

                if insertable_column.has_default() {
                    let default_value = insertable_column.rust_default_value(conn)?;
                    default_impl_attributes.push(quote::quote! {
                        #attribute_ident: Some(#default_value)
                    });
                } else {
                    default_impl_attributes.push(quote::quote! {
                        #attribute_ident: Default::default()
                    });
                }
            }

            let maybe_default_where_requirements = if defalt_where_requirements.is_empty() {
                None
            } else {
                Some(quote::quote! {
                    where #(#defalt_where_requirements),*
                })
            };

            quote::quote! {
                impl #maybe_impl_generics Default for #builder_ident #maybe_impl_generics #maybe_default_where_requirements {
                    fn default() -> Self {
                        Self {
                            #(#default_impl_attributes),*
                        }
                    }
                }
            }
        } else {
            TokenStream::new()
        };

        let mut insertable_builder_attributes = insertable_columns
            .iter()
            // We handle separately the extension primary key columns.
            .filter_map(|column| {
                if column.is_part_of_extension_primary_key(conn).ok().flatten().is_some() {
                    return None;
                }
                let nullable_column = column.to_nullable();
                let column_name = column.snake_case_ident().ok()?;
                let column_type = if let Some((partial_builder_kind, _, foreign_key)) =
                    column.requires_partial_builder(conn).ok()?
                {
                    let foreign_table = foreign_key.foreign_table(conn).ok()?;
                    partial_builder_kind.formatted_type(&foreign_table, conn).ok()?
                } else {
                    let nullable_column_type = nullable_column.rust_data_type(conn).ok()?;
                    quote! { #nullable_column_type }
                };
                Some(Ok(quote::quote! {
                    pub(crate) #column_name: #column_type
                }))
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        insertable_builder_attributes.extend(
            table.extension_foreign_keys(conn)?.into_iter().filter_map(|foreign_key| {
                let foreign_table = foreign_key.foreign_table(conn).ok()?;
                let constraint_ident = foreign_key.constraint_ident(conn).ok()?;
                let builder_generic_parameter = foreign_table.struct_ident().ok()?;
                Some(quote::quote! {
                    pub(crate) #constraint_ident: #builder_generic_parameter
                })
            }),
        );

        let is_complete_builder_impl =
            self.generate_is_complete_builder_implementation(table, conn)?;
        let from_builder_to_id_or_builder_impl =
            self.from_builder_to_id_or_builder_impl(table, conn)?;
        let builder_trait_definition = table.generate_builder_trait(conn)?;
        let builder_trait_impls = table.generate_builder_trait_impl_for_ancestral_tables(
            extension_network,
            conn,
            self.check_constraints_extensions.as_slice(),
        )?;
        let documentation: Vec<String> = self.generate_builder_documentation(table, conn)?;

        Ok(quote! {
            #[derive(#(#derives),*)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #(#[doc = #documentation])*
            pub struct #builder_ident #maybe_generics {
                #(#insertable_builder_attributes),*
            }

            #from_builder_to_id_or_builder_impl
            #insertable_builder_default_impl
            #is_complete_builder_impl

            #builder_trait_definition
            #(#builder_trait_impls)*
        })
    }
}
