//! Submodule implementing the `ExtendableBuilder` trait for the insertable
//! builder struct.

use std::collections::HashSet;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Table, errors::WebCodeGenError};

impl Codegen<'_> {
    /// Generates the implementation of the `ExtendableBuilder` trait for the
    /// insertable builder struct.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which the implementation is generated.
    /// * `conn` - The database connection to use for querying the database.
    ///
    /// # Errors
    ///
    /// * If the provided connection to the database fails.
    pub(super) fn generate_extendable_builder_implementation(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let builder_ident = table.insertable_builder_ident()?;
        let extension_network = self.table_extension_network().unwrap();
        let extended_tables = extension_network.extension_tables(table);
        let mut extendable_builder_constraints: HashSet<Table> = HashSet::new();

        let where_requirements = extended_tables
            .iter()
            .map(|extended_table| {
                let struct_ident = extended_table.struct_ident()?;
                let extended_attributes = extended_table.insertable_enum_ty()?;
                extendable_builder_constraints.insert((*extended_table).clone());
                Ok(quote::quote! {
                    #struct_ident: web_common_traits::database::ExtendableBuilder<
                        Attributes = #extended_attributes
                    >
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let maybe_generics = extension_network.generics_for_table_builder_implementation(table)?;
        let attributes = table.insertable_enum_ident()?;
        let extension_attributes = table.insertable_extension_enum_ident()?;

        // At this point, we proceed to call the setter methods for each term which
        // appears set in the other builder.
        let extensions = table
            .insertable_columns(conn, true)?
            .into_iter()
            .map(|insertable_column| {
                Ok(
                    if let Some(_partial_builder_foreign_key) =
                        insertable_column.requires_partial_builder(conn)?
                    {
                        let column_ident = insertable_column.snake_case_ident()?;
                        let column_setter = insertable_column.getter_ident()?;

						if insertable_column.is_nullable() {
							quote::quote! {
								if let Some(#column_ident) = other.#column_ident {
									self = self.#column_setter(Some(#column_ident))?;
								}
							}
						} else {
							quote::quote! {
								self = self.#column_setter(other.#column_ident)?;
							}
						}
                    } else if let Some(extension_foreign_key) =
                        insertable_column.is_part_of_extension_primary_key(conn)?
                    {
                        // Analogously to the previous case, if the column is part of an
                        // extension primary key, we need to extend the builder with the foreign
                        // key. There is no need to extend the where
                        // requirements, as this is already handled earlier
                        // in this method. If the column represents a
                        // partial builder, we need to recursively
                        // call the `extend_builder` method for the foreign key and extend the where
                        // requirements appropriately.
						let extension_table = extension_foreign_key
							.foreign_table(conn)?
							.expect("Extension foreign key should have a foreign table");
						let extension_table_ident = extension_table.struct_ident()?;
                        let constraint_ident = extension_foreign_key.constraint_ident(conn)?;
                        assert!(
                            !insertable_column.is_nullable(),
                            "Extension primary key columns should not be nullable"
                        );

                        quote::quote! {
                            self.#constraint_ident = self.#constraint_ident.extend_builder(other.#constraint_ident)
								.map_err(|err| {
									err.into_field_name(|attribute| #attributes::Extension(
										#extension_attributes::#extension_table_ident(attribute)
									))
								})?;
                        }
                    } else {
                        // Finally, when the column is no form of foreign key, we can simply
                        // call the setter method for the column.
                        let column_ident = insertable_column.snake_case_ident()?;
                        let setter_ident = insertable_column.getter_ident()?;
                        if insertable_column.is_nullable() {
                            quote::quote! {
                                if let Some(#column_ident) = other.#column_ident {
                                    self = self.#setter_ident(Some(#column_ident))?;
                                }
                            }
                        } else {
                            quote::quote! {
                                if let Some(#column_ident) = other.#column_ident {
                                    self = self.#setter_ident(#column_ident)?;
                                }
                            }
                        }
                    },
                )
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let maybe_where_requirements = if where_requirements.is_empty() {
            None
        } else {
            Some(quote::quote! {
                where #(#where_requirements),*
            })
        };

        Ok(quote! {
            impl #maybe_generics web_common_traits::database::ExtendableBuilder for #builder_ident #maybe_generics #maybe_where_requirements {
                type Attributes = #attributes;

                fn extend_builder(mut self, other: Self) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
                    #(#extensions)*
                    Ok(self)
                }
            }
        })
    }
}
