//! Submodule implementing the
//! [`ForeignKeys`](web_common_traits::prelude::ForeignKeys) trait for a struct.

use std::path::Path;

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{Codegen, Column, Table, errors::WebCodeGenError};

impl Codegen<'_> {
    /// Returns the identifier for the struct which implements the `ForeignKeys`
    /// trait.
    pub(crate) fn foreign_keys_struct_ident(table: &Table) -> Result<Ident, WebCodeGenError> {
        let table_name = table.struct_name()?;
        Ok(Ident::new(&format!("{table_name}ForeignKeys"), proc_macro2::Span::call_site()))
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
    pub(super) async fn generate_foreign_keys_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        let mut table_foreign_main_module = TokenStream::new();
        let table_primary_keys_enum_path = Self::table_primary_keys_enum_path();
        let row_enum_path = Self::row_enum_path();
        for table in tables {
            let table_path = table.import_struct_path()?;
            let foreign_keys_trait_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let snake_case_ident = table.snake_case_ident()?;
            let foreign_keys = table.foreign_keys(conn).await?;

            if foreign_keys.is_empty() {
                continue;
            }

            table_foreign_main_module.extend(quote::quote! {
                mod #snake_case_ident;
            });

            let mut foreign_tables = Vec::new();

            for foreign_key in &foreign_keys {
                let foreign_table = foreign_key.foreign_table(conn).await?.unwrap().0;
                foreign_tables.push(foreign_table);
            }

            let foreign_keys_struct_ident = Self::foreign_keys_struct_ident(table)?;
            let attributes = foreign_keys
                .iter()
                .zip(foreign_tables.iter())
                .map(|(foreign_key, foreign_table)| {
                    let getter_ident = foreign_key.getter_ident()?;
                    let foreign_table_struct_type = foreign_table.import_struct_path()?;

                    Ok(quote::quote! {
                        pub #getter_ident: Option<#foreign_table_struct_type>
                    })
                })
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;

            let fully_loaded_method_impl = foreign_keys
                .iter()
                .map(|foreign_key| {
                    let attribute = foreign_key.snake_case_ident()?;
                    let getter_ident = foreign_key.getter_ident()?;
                    if foreign_key.is_nullable() {
                        Ok((
                            quote::quote! {
                                foreign_keys.#getter_ident.is_some() || self.#attribute.is_none()
                            },
                            true,
                        ))
                    } else {
                        Ok((
                            quote::quote! {
                                foreign_keys.#getter_ident.is_some()
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
                    let snake_case_ident = foreign_key.snake_case_ident()?;
                    let foreign_table_struct_ident = foreign_table.struct_ident()?;

                    Ok(if foreign_key.is_nullable() {
                        quote::quote! {
                            if let Some(#snake_case_ident) = self.#snake_case_ident {
                                connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                                    #table_primary_keys_enum_path::#foreign_table_struct_ident(#snake_case_ident)
                                ));
                            }
                        }
                    } else {
                        quote::quote! {
                            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                                #table_primary_keys_enum_path::#foreign_table_struct_ident(self.#snake_case_ident)
                            ));
                        }
                    })
                })
                .collect::<Result<TokenStream, WebCodeGenError>>()?;

            let mut unique_table_types = foreign_tables.clone();
            unique_table_types.sort_unstable();
            unique_table_types.dedup();
            let mut update_impls: Vec<TokenStream> = Vec::new();

            for unique_foreign_table in unique_table_types {
                let mut grouped_columns: Vec<(&Column, Column)> = Vec::new();

                for foreign_key in &foreign_keys {
                    let (foreign_table, column_in_foreign_table) =
                        foreign_key.foreign_table(conn).await?.unwrap();
                    if foreign_table == unique_foreign_table {
                        grouped_columns.push((foreign_key, column_in_foreign_table));
                    }
                }

                let foreign_table_snake_case_ident = unique_foreign_table.snake_case_ident()?;
                let foreign_table_struct_ident = unique_foreign_table.struct_ident()?;

                let maybe_cloned = if grouped_columns.len() > 1
                    && !unique_foreign_table.supports_copy(conn).await?
                {
                    quote::quote! {
                        #foreign_table_snake_case_ident.clone()
                    }
                } else {
                    quote::quote! {
                        #foreign_table_snake_case_ident
                    }
                };

                let upsert_foreign_keys_dispatch = grouped_columns.iter()
                        .map(|(column, column_in_foreign_table)| {
                            let column_snake_case_ident = column.snake_case_ident()?;
                            let column_in_foreign_table_snake_case_ident = column_in_foreign_table.snake_case_ident()?;
                            let getter_ident = column.getter_ident()?;

                            Ok(if column.is_nullable() {
                                let wrapper_column_snake_case_ident = if column_in_foreign_table.is_nullable () {
                                    quote::quote! {
                                        Some(#column_snake_case_ident)
                                    }
                                } else {
                                    quote::quote! {#column_snake_case_ident}
                                };

                                quote::quote! {
                                    if let Some(#column_snake_case_ident) = self.#column_snake_case_ident {
                                        if #foreign_table_snake_case_ident.#column_in_foreign_table_snake_case_ident == #wrapper_column_snake_case_ident {
                                            foreign_keys.#getter_ident = Some(#maybe_cloned);
                                            updated = true;
                                        }
                                    }
                                }
                            } else {
                                let wrapper_column_snake_case_ident = if column_in_foreign_table.is_nullable () {
                                    quote::quote! {
                                        Some(self.#column_snake_case_ident)
                                    }
                                } else {
                                    quote::quote! {self.#column_snake_case_ident}
                                };

                                quote::quote! {
                                    if #foreign_table_snake_case_ident.#column_in_foreign_table_snake_case_ident == #wrapper_column_snake_case_ident {
                                        foreign_keys.#getter_ident = Some(#maybe_cloned);
                                        updated = true;
                                    }
                                }
                            })
                        })
                        .collect::<Result<TokenStream, WebCodeGenError>>()?;

                let delete_foreign_keys_dispatch = grouped_columns.iter()
                        .map(|(column, column_in_foreign_table)| {
                            let column_snake_case_ident = column.snake_case_ident()?;
                            let column_in_foreign_table_snake_case_ident = column_in_foreign_table.snake_case_ident()?;
                            let getter_ident = column.getter_ident()?;

                            Ok(if column.is_nullable() {
                                let wrapper_column_snake_case_ident = if column_in_foreign_table.is_nullable () {
                                    quote::quote! {
                                        Some(#column_snake_case_ident)
                                    }
                                } else {
                                    quote::quote! {#column_snake_case_ident}
                                };

                                quote::quote! {
                                    if let Some(#column_snake_case_ident) = self.#column_snake_case_ident {
                                        if #foreign_table_snake_case_ident.#column_in_foreign_table_snake_case_ident == #wrapper_column_snake_case_ident {
                                            foreign_keys.#getter_ident = None;
                                            updated = true;
                                        }
                                    }
                                }
                            } else {
                                let wrapper_column_snake_case_ident = if column_in_foreign_table.is_nullable () {
                                    quote::quote! {
                                        Some(self.#column_snake_case_ident)
                                    }
                                } else {
                                    quote::quote! {self.#column_snake_case_ident}
                                };

                                quote::quote! {
                                    if #foreign_table_snake_case_ident.#column_in_foreign_table_snake_case_ident == #wrapper_column_snake_case_ident {
                                        foreign_keys.#getter_ident = None;
                                        updated = true;
                                    }
                                }
                            })
                        })
                        .collect::<Result<TokenStream, WebCodeGenError>>()?;

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
                if !foreign_table.supports_eq(conn).await? {
                    supports_eq = false;
                    break;
                }
            }

            if supports_eq {
                derives.push(quote::quote! {Eq});
            }

            let mut supports_hash = true;
            for foreign_table in &foreign_tables {
                if !foreign_table.supports_hash(conn).await? {
                    supports_hash = false;
                    break;
                }
            }
            if supports_hash {
                derives.push(quote::quote! {Hash});
            }

            let mut supports_ord = true;

            for foreign_table in &foreign_tables {
                if !foreign_table.supports_ord(conn).await? {
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
