//! Submodule implementing the `ExtendableBuilder` trait for the insertable
//! builder struct.

use std::collections::HashSet;

use diesel::PgConnection;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use crate::traits::TableLike;

use crate::{Codegen, Column, Table, errors::WebCodeGenError};

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
                Ok(quote! {
                    #struct_ident: web_common_traits::database::ExtendableBuilder<
                        Attributes = #extended_attributes
                    >
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let maybe_generics = extension_network.generics_for_table_builder_implementation(table)?;
        let attributes = table.insertable_enum_ident()?;
        let extension_attributes = table.insertable_extension_enum_ident()?;
        let mut multi_column_check_constraints = table
            .check_constraints(conn)?
            .into_iter()
            .filter(|check_constraint| {
                // We only keep the check constraints which are
                // relative to multiple columns and which do not
                // involve any automatically generated columns.
                let Ok(columns) = check_constraint.columns(conn) else {
                    return false;
                };
                columns.len() > 1
                    && !columns.iter().any(|column| column.is_always_automatically_generated())
            })
            .collect::<Vec<_>>();

        // We sort the multi-column check constraints by the number of columns
        // they involve, so that the most complex ones are handled first.
        multi_column_check_constraints.sort_by(|a, b| {
            b.columns(conn)
                .map_or(0, |columns| columns.len())
                .cmp(&a.columns(conn).map_or(0, |columns| columns.len()))
        });

        let columns_involved_in_check_constraints = multi_column_check_constraints
            .iter()
            .flat_map(|check_constraint| check_constraint.columns(conn).unwrap_or_default())
            .collect::<HashSet<_>>();

        // At this point, we proceed to call the setter methods for each term which
        // appears set in the other builder.
        let single_column_setters = table
            .insertable_columns(conn, true)?
            .into_iter()
            .filter(|insertable_column| {
                // We only keep the columns which are not part of the
                // multi-column check constraints, which are handled
                // separately afterwards.
                !columns_involved_in_check_constraints.contains(insertable_column)
            })
            .map(|insertable_column| {
                Ok(
                    if let Some(_partial_builder_foreign_key) =
                        insertable_column.requires_partial_builder(conn)?
                    {
                        let column_ident = insertable_column.snake_case_ident()?;
                        let column_setter = insertable_column.getter_ident()?;

						if insertable_column.is_nullable() {
							quote! {
								if let Some(#column_ident) = other.#column_ident {
									self = self.#column_setter(Some(#column_ident))?;
								}
							}
						} else {
							quote! {
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

                        quote! {
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
                            quote! {
                                if let Some(#column_ident) = other.#column_ident {
                                    self = self.#setter_ident(Some(#column_ident))?;
                                }
                            }
                        } else {
                            quote! {
                                if let Some(#column_ident) = other.#column_ident {
                                    self = self.#setter_ident(#column_ident)?;
                                }
                            }
                        }
                    },
                )
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        // TODO! Handle combinations of overlapping multi-column check constraints.
        for multi_column_check_constraint in &multi_column_check_constraints {
            let columns = multi_column_check_constraint.columns(conn)?;
            for inner in &multi_column_check_constraints {
                if inner == multi_column_check_constraint {
                    continue;
                }

                let inner_columns = inner.columns(conn)?;
                if columns.iter().all(|c| inner_columns.contains(c)) {
                    unimplemented!(
                        "Handling combinations of overlapping multi-column check constraints is not implemented yet."
                    );
                }
            }
        }

        let multi_column_setters = multi_column_check_constraints
            .into_iter()
            .map(|check_constraint| {
                let columns = check_constraint.columns(conn)?;
                let column_idents = columns
                    .iter()
                    .map(|column| column.snake_case_ident())
                    .collect::<Result<Vec<_>, WebCodeGenError>>()?;
                let multi_setter_ident = Column::multi_column_getter_ident(&columns)?;
                let mut combinations = Vec::new();
                // First we handle the case where all columns are set.
                combinations.push(quote! {
                    (#(Some(#column_idents)),*) => {
                        self = self.#multi_setter_ident(#(#column_idents),*)?;
                    }
                });
                for number_of_null_values in 1..=columns.len() {
                    for null_columns in columns.iter().combinations(number_of_null_values) {
                        let mut combination = Vec::new();
                        let mut setter_calls = Vec::new();
                        for column in &columns {
                            if null_columns.contains(&column) {
                                combination.push(quote! { None });
                            } else {
                                let setter_ident = column.getter_ident()?;
                                let column_ident = column.snake_case_ident()?;
                                combination.push(quote! { Some(#column_ident) });
                                setter_calls.push(if column.is_nullable() {
                                    quote! {
                                        self = self.#setter_ident(Some(#column_ident))?;
                                    }
                                } else {
                                    quote! {
                                        self = self.#setter_ident(#column_ident)?;
                                    }
                                });
                            }
                        }
                        combinations.push(quote! {
                            (#(#combination),*) => {
                                #(#setter_calls)*
                            }
                        });
                    }
                }

                Ok(quote! {
                    match (#(other.#column_idents),*) {
                        #(#combinations)*
                    }
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let maybe_where_requirements = if where_requirements.is_empty() {
            None
        } else {
            Some(quote! {
                where #(#where_requirements),*
            })
        };

        Ok(quote! {
            impl #maybe_generics web_common_traits::database::ExtendableBuilder for #builder_ident #maybe_generics #maybe_where_requirements {
                type Attributes = #attributes;

                fn extend_builder(mut self, other: Self) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
                    #(#multi_column_setters)*
                    #(#single_column_setters)*
                    Ok(self)
                }
            }
        })
    }
}
