//! Submodule implementing the
//! [`ForeignKeys`](web_common_traits::prelude::ForeignKeys) trait for a struct.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{Codegen, Column, KeyColumnUsage, Table, errors::WebCodeGenError};

impl Codegen<'_> {
    /// Returns the identifier for the struct which implements the `ForeignKeys`
    /// trait.
    pub(crate) fn foreign_keys_struct_ident(table: &Table) -> Result<Ident, WebCodeGenError> {
        let table_name = table.struct_name()?;
        Ok(Ident::new(&format!("{table_name}ForeignKeys"), proc_macro2::Span::call_site()))
    }

    #[allow(clippy::needless_pass_by_value)]
    fn build_dispatch(
        foreign_keys: &[&KeyColumnUsage],
        foreign_table: &Table,
        assignment_right_term: TokenStream,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let foreign_table_snake_case_ident = foreign_table.snake_case_ident()?;
        foreign_keys.iter()
            .map(|foreign_key_constraint| {
                let columns = foreign_key_constraint.columns(conn)?;
                let foreign_columns = foreign_key_constraint.foreign_columns(conn)?;
                let constraint_ident = foreign_key_constraint.constraint_ident(conn)?;

                let if_statement = columns.iter().zip(foreign_columns.iter()).map(|(column, foreign_column)|{
                    let column_ident = column.snake_case_ident()?;
                    let foreign_ident = foreign_column.snake_case_ident()?;

                    Ok(match (column.is_nullable(), foreign_column.is_nullable()) {
                        (false, false) => quote::quote! {
                            self.#column_ident == #foreign_table_snake_case_ident.#foreign_ident
                        },
                        (true, false) => quote::quote! {
                            self.#column_ident.is_some_and(|#column_ident| #column_ident == #foreign_table_snake_case_ident.#foreign_ident)
                        },
                        (false, true) => quote::quote! {
                            #foreign_table_snake_case_ident.#foreign_ident.is_some_and(|#foreign_ident| self.#column_ident == #foreign_ident)
                        },
                        (true, true) => quote::quote! {
                            self.#column_ident.zip(#foreign_table_snake_case_ident.#foreign_ident).is_some_and(|(self_column, foreign_column)| self_column == foreign_column)
                        },
                    })
                }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

                Ok(quote::quote! {
                    if #(#if_statement)&&* {
                        foreign_keys.#constraint_ident = #assignment_right_term;
                        updated = true;
                    }
                })
            })
            .collect::<Result<TokenStream, WebCodeGenError>>()
    }

    #[allow(clippy::too_many_lines)]
    /// Generates the [`ForeignKeys`](web_common_traits::prelude::ForeignKeys)
    /// traits implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(super) fn generate_foreign_keys_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        let mut table_foreign_main_module = TokenStream::new();
        let table_primary_keys_enum_path = Self::table_primary_keys_enum_path();
        let row_enum_path = Self::row_enum_path();
        for table in tables {
            let table_path = table.import_struct_path()?;
            let foreign_keys_trait_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let snake_case_ident = table.snake_case_ident()?;
            let mut foreign_keys = table.foreign_keys(conn)?;

            // We only keep foreign key constraints that are associated with foreign primary
            // keys.
            foreign_keys.retain(|fk| fk.is_foreign_primary_key(conn).unwrap_or(false));

            for column in table.columns(conn)? {
                if let Some(foreign_key) = column.requires_partial_builder(conn)? {
                    // We remove this foreign key from the list of foreign keys
                    // to avoid including functionally useless foreign keys.
                    foreign_keys.retain(|fk| fk != &foreign_key);
                }
                for same_as_forein_keys in column.same_as_constraints(conn)? {
                    foreign_keys.retain(|fk| fk != &same_as_forein_keys);
                }
            }

            if foreign_keys.is_empty() {
                continue;
            }

            table_foreign_main_module.extend(quote::quote! {
                mod #snake_case_ident;
            });

            let mut foreign_tables = Vec::new();

            for foreign_key in &foreign_keys {
                let foreign_table = foreign_key.foreign_table(conn)?.unwrap();
                foreign_tables.push(foreign_table);
            }

            let foreign_keys_struct_ident = Self::foreign_keys_struct_ident(table)?;
            let attributes = foreign_keys
                .iter()
                .zip(foreign_tables.iter())
                .map(|(foreign_key, foreign_table)| {
                    let constraint_ident = foreign_key.constraint_ident(conn)?;
                    let foreign_table_struct_type = foreign_table.import_struct_path()?;

                    Ok(quote::quote! {
                        pub #constraint_ident: Option<#foreign_table_struct_type>
                    })
                })
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;

            let fully_loaded_method_impl = foreign_keys
                .iter()
                .map(|foreign_key| {
                    let columns = foreign_key.columns(conn)?;
                    let attributes = columns
                        .iter()
                        .filter(|c| c.is_nullable())
                        .map(|c| {
                            let attribute = c.snake_case_ident()?;
                            Ok(quote::quote! {
                                self.#attribute.is_some()
                            })
                        })
                        .try_fold(
                            TokenStream::new(),
                            |acc, attribute: Result<TokenStream, WebCodeGenError>| {
                                let attribute = attribute?;
                                Ok::<TokenStream, WebCodeGenError>(if acc.is_empty() {
                                    attribute
                                } else {
                                    quote::quote! {
                                        #acc && #attribute
                                    }
                                })
                            },
                        )?;

                    let constraint_ident = foreign_key.constraint_ident(conn)?;
                    if columns.iter().any(Column::is_nullable) {
                        Ok((
                            quote::quote! {
                                foreign_keys.#constraint_ident.is_some() || #attributes
                            },
                            true,
                        ))
                    } else {
                        Ok((
                            quote::quote! {
                                foreign_keys.#constraint_ident.is_some()
                            },
                            false,
                        ))
                    }
                })
                .try_fold(
                    TokenStream::new(),
                    |acc, foreign_key: Result<(TokenStream, bool), WebCodeGenError>| {
                        let (foreign_key, or_joined) = foreign_key?;
                        Ok::<TokenStream, WebCodeGenError>(if acc.is_empty() {
                            if or_joined && foreign_keys.len() > 1 {
                                quote::quote! {
                                    (#foreign_key)
                                }
                            } else {
                                quote::quote! {
                                    #foreign_key
                                }
                            }
                        } else if or_joined {
                            quote::quote! {
                                #acc && (#foreign_key)
                            }
                        } else {
                            quote::quote! {
                                #acc && #foreign_key
                            }
                        })
                    },
                )?;

            let load_foreign_keys_impl = foreign_keys
                .iter()
                .zip(foreign_tables.iter())
                .map(|(foreign_key, foreign_table)| {
                    let columns = foreign_key.columns(conn)?;
                    let foreign_table_struct_ident = foreign_table.struct_ident()?;

                    let attributes = columns
                        .iter()
                        .map(|c| {
                            let snake_case_ident = c.snake_case_ident()?;
                            Ok(if c.is_nullable() {
                                quote::quote! {
                                    #snake_case_ident
                                }
                            } else {
                                quote::quote! {
                                    self.#snake_case_ident
                                }
                            })
                        })
                        .collect::<Result<Vec<_>, WebCodeGenError>>()?;

                    let formatted_attributes = if attributes.len() == 1 {
                        quote::quote! {
                            #table_primary_keys_enum_path::#foreign_table_struct_ident(#(#attributes)*)
                        }
                    } else {
                        quote::quote! {
                            #table_primary_keys_enum_path::#foreign_table_struct_ident((#(#attributes),*))
                        }
                    };

                    let send = quote::quote! {
                        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                            #formatted_attributes
                        ));
                    };

                    Ok(if columns.iter().any(Column::is_nullable) {
                        let optional_idents = columns
                            .iter()
                            .filter(|c| c.is_nullable())
                            .map(|c| {
                                c.snake_case_ident()
                            })
                            .collect::<Result<Vec<_>, WebCodeGenError>>()?;
                        if optional_idents.len() == 1 {
                            quote::quote! {
                                if let #(Some(#optional_idents))* = #(self.#optional_idents)* {
                                    #send
                                }
                            }
                        } else {
                            quote::quote! {
                                if let (#(Some(#optional_idents),)*) = (#(self.#optional_idents),*) {
                                    #send
                                }
                            }
                        }
                    } else {
                        send
                    })
                })
                .collect::<Result<TokenStream, WebCodeGenError>>()?;

            let mut unique_table_types = foreign_tables.clone();
            unique_table_types.sort_unstable();
            unique_table_types.dedup();
            let mut update_impls: Vec<TokenStream> = Vec::new();

            for unique_foreign_table in unique_table_types {
                let mut unique_foreign_keys = Vec::new();

                for foreign_key in &foreign_keys {
                    let foreign_table = foreign_key.foreign_table(conn)?.unwrap();
                    if foreign_table == unique_foreign_table {
                        unique_foreign_keys.push(foreign_key);
                    }
                }

                let foreign_table_snake_case_ident = unique_foreign_table.snake_case_ident()?;
                let foreign_table_struct_ident = unique_foreign_table.struct_ident()?;

                let maybe_cloned = if unique_foreign_keys.len() > 1
                    && !unique_foreign_table.supports_copy(conn)?
                {
                    quote::quote! {
                        #foreign_table_snake_case_ident.clone()
                    }
                } else {
                    quote::quote! {
                        #foreign_table_snake_case_ident
                    }
                };

                let upsert_foreign_keys_dispatch = Self::build_dispatch(
                    unique_foreign_keys.as_slice(),
                    &unique_foreign_table,
                    quote::quote! {
                        Some(#maybe_cloned)
                    },
                    conn,
                )?;

                let delete_foreign_keys_dispatch = Self::build_dispatch(
                    unique_foreign_keys.as_slice(),
                    &unique_foreign_table,
                    quote::quote! {
                        None
                    },
                    conn,
                )?;

                update_impls.push(quote::quote! {
                    (#row_enum_path::#foreign_table_struct_ident(#foreign_table_snake_case_ident), web_common_traits::crud::CRUD::Read | web_common_traits::crud::CRUD::Create | web_common_traits::crud::CRUD::Update) => {
                        #upsert_foreign_keys_dispatch
                    }
                    (#row_enum_path::#foreign_table_struct_ident(#foreign_table_snake_case_ident), web_common_traits::crud::CRUD::Delete) => {
                        #delete_foreign_keys_dispatch
                    }
                });
            }

            let mut derives = vec![
                quote::quote! {
                    Debug
                },
                quote::quote! {
                    Clone
                },
                quote::quote! {
                    PartialEq
                },
                quote::quote! {
                    Default
                },
            ];

            let mut supports_eq = true;

            for foreign_table in &foreign_tables {
                if !foreign_table.supports_eq(conn)? {
                    supports_eq = false;
                    break;
                }
            }

            if supports_eq {
                derives.push(quote::quote! {Eq});
            }

            let mut supports_hash = true;
            for foreign_table in &foreign_tables {
                if !foreign_table.supports_hash(conn)? {
                    supports_hash = false;
                    break;
                }
            }
            if supports_hash {
                derives.push(quote::quote! {Hash});
            }

            let mut supports_ord = true;

            for foreign_table in &foreign_tables {
                if !foreign_table.supports_ord(conn)? {
                    supports_ord = false;
                    break;
                }
            }

            if supports_ord {
                derives.push(quote::quote! {PartialOrd});
                derives.push(quote::quote! {Ord});
            }

            std::fs::write(
                &foreign_keys_trait_file,
                self.beautify_code(&quote::quote! {
                    #[derive(#(#derives),*)]
                    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                    pub struct #foreign_keys_struct_ident {
                        #(#attributes),*
                    }

                    impl web_common_traits::prelude::HasForeignKeys for #table_path {
                        type ForeignKeys = #foreign_keys_struct_ident;
                        type Row = #row_enum_path;

                        fn load_foreign_keys<C>(&self, connector: &C)
                        where
                            C: web_common_traits::crud::Connector<Row = Self::Row>,
                            {
                                #load_foreign_keys_impl
                            }

                        fn foreign_keys_loaded(
                            &self,
                            foreign_keys: &Self::ForeignKeys,
                        ) -> bool {
                            #fully_loaded_method_impl
                        }

                        fn update(&self, foreign_keys: &mut Self::ForeignKeys, row: Self::Row, crud: web_common_traits::crud::CRUD) -> bool {
                            let mut updated = false;
                            match (row, crud) {
                                #(#update_impls),*
                                (_, crud) => {
                                    unreachable!("Unexpected row type with operation {crud}");
                                }
                            }
                            updated
                        }
                    }
                    impl web_common_traits::prelude::ForeignKeys for #foreign_keys_struct_ident {}
                })?,
            )?;
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_foreign_main_module)?)?;

        Ok(())
    }
}
