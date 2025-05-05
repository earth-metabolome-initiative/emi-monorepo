//! Submodule generating an enumeration of all the tables in the database.

use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use strum::IntoEnumIterator;
use syn::Ident;

use crate::{
    Codegen, Table,
    codegen::{CODEGEN_DIRECTORY, CODEGEN_TABLES_PATH, Syntax},
};

impl Codegen<'_> {
    pub(crate) fn row_enum_path(&self) -> TokenStream {
        let codegen_ident = Ident::new(CODEGEN_DIRECTORY, proc_macro2::Span::call_site());
        let tables_module_ident = Ident::new(CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());
        quote::quote! {
            crate::#codegen_ident::#tables_module_ident::row::Row
        }
    }

    #[allow(clippy::too_many_lines)]
    /// Generate `Row` and `Rows` structs for the provided tables.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(crate) async fn generate_row_enumeration(
        &self,
        root: &Path,
        tables: &[Table],
        _conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        let table_name_enum_path = self.table_names_enum_path();
        let table_primary_keys_path = self.table_primary_keys_enum_path();
        let rows_enum_path = self.rows_enum_path();

        let sqlite_upsert: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_name_ident = table.snake_case_ident()?;
                Ok(quote::quote! {
                    Row::#struct_ident(#struct_name_ident) => {
                        #struct_name_ident.upsert(conn)?.map(Row::from)
                    }
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let row_table_names = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                Ok(quote::quote! {
                    super::Row::#struct_ident(_) => #table_name_enum_path::#struct_ident,
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let row_primary_keys = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_name_ident = table.snake_case_ident()?;
                Ok(quote::quote! {
                    super::Row::#struct_ident(#struct_name_ident) => #struct_name_ident.primary_key(),
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let row_variants: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_path = table.import_struct_path()?;
                Ok(quote::quote! {
                    #struct_ident(#struct_path)
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let row_enumeration = quote::quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Row {
                #(#row_variants),*
            }
        };

        let row_to_rows = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let snake_case_ident = table.snake_case_ident()?;
                Ok(quote::quote! {
                    super::Row::#struct_ident(#snake_case_ident) => {
                        #rows_enum_path::from(#snake_case_ident)
                    }
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let include_upsert = if self.enable_upsertable_trait {
            quote! {
                #[cfg(feature = "sqlite")]
                /// Executes the upsert operation and returns the result.
                pub fn sqlite_upsert(
                    &self,
                    conn: &mut diesel::SqliteConnection,
                ) -> Result<Option<Row>, diesel::result::Error>
                {
                    use web_common_traits::database::Upsertable;
                    Ok(match self {
                        #(#sqlite_upsert),*
                    })
                }
            }
        } else {
            TokenStream::new()
        };

        let mut modules = Vec::new();
        for table in tables {
            let struct_path = table.import_struct_path()?;
            let struct_ident = table.struct_ident()?;
            let snake_case_ident = table.snake_case_ident()?;
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));

            modules.push(quote::quote! {
                mod #snake_case_ident;
            });
            // impl Deletable for struct_ident
            std::fs::write(
                &table_file,
                self.beautify_code(&quote! {
                    impl From<#struct_path> for super::Row {
                        fn from(value: #struct_path) -> Self {
                            super::Row::#struct_ident(value)
                        }
                    }
                    impl TryFrom<super::Row> for #struct_path {
                        type Error = std::convert::Infallible;
                        fn try_from(value: super::Row) -> Result<Self, Self::Error> {
                            match value {
                                super::Row::#struct_ident(v) => Ok(v),
                                value => {
                                    // This should never happen, but we need to handle it
                                    // because the compiler doesn't know that the enum is
                                    // exhaustive.
                                    unreachable!("Unexpected variant in Row enum: {value:?}")
                                }
                            }
                        }
                    }
                })?,
            )?;
        }

        let mut trait_modules: Vec<(&str, TokenStream)> = Vec::new();

        trait_modules.push((
            "from_row",
            quote::quote! {
                impl From<super::Row> for #rows_enum_path {
                    fn from(value: super::Row) -> Self {
                        match value {
                            #(#row_to_rows),*
                        }
                    }
                }
            },
        ));

        trait_modules.push((
            "tabular",
            quote::quote! {
                impl web_common_traits::prelude::Tabular for super::Row {
                    type TableName = #table_name_enum_path;

                    fn table_name(&self) -> Self::TableName {
                        match self {
                            #(
                                #row_table_names
                            )*
                        }
                    }
                }
            },
        ));

        trait_modules.push((
            "row",
            quote::quote! {
                impl web_common_traits::prelude::Row for super::Row {
                    type PrimaryKey = #table_primary_keys_path;

                    fn primary_key(&self) -> Self::PrimaryKey {
                        match self {
                            #(
                                #row_primary_keys
                            )*
                        }
                    }
                }
            },
        ));

        Syntax::iter()
            .try_for_each(|syntax| {
                let feature_flag = syntax.as_feature_flag();
                let table_primary_keys_path = table_primary_keys_path.clone();
                let sync_connection = syntax.as_connection_type(false);
                let async_connection = syntax.as_connection_type(true);
                if syntax.is_sqlite() {
                    let read_impls: Vec<TokenStream> = tables
                        .iter()
                        .map(|table| {
                            let struct_ident = table.struct_ident()?;
                            let struct_path = table.import_struct_path()?;
                            Ok(quote::quote! {
                                #table_primary_keys_path::#struct_ident(primary_key) => {
                                    Ok(<#struct_path as web_common_traits::database::Read<#sync_connection>>::read(primary_key, conn)?
                                        .map(super::Row::from))
                                }
                            })
                        })
                        .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;
                    trait_modules.push((
                        "sqlite_read_dispatch",
                        quote::quote! {
                            #feature_flag
                            impl web_common_traits::prelude::ReadDispatch<#sync_connection> for super::Row {
                                type PrimaryKey = #table_primary_keys_path;

                                fn read(
                                    primary_key: Self::PrimaryKey,
                                    conn: &mut #sync_connection,
                                ) -> Result<Option<Self>, diesel::result::Error> {
                                    match primary_key {
                                        #(
                                            #read_impls
                                        )*
                                    }
                                }
                            }
                        })
                    );
                } else {
                    let async_read_impls: Vec<TokenStream> = tables
                        .iter()
                        .map(|table| {
                            let struct_ident = table.struct_ident()?;
                            let struct_path = table.import_struct_path()?;
                            Ok(quote::quote! {
                                #table_primary_keys_path::#struct_ident(primary_key) => {
                                    Ok(<#struct_path as web_common_traits::database::AsyncRead<#async_connection>>::read_async(primary_key, conn).await?
                                        .map(super::Row::from))
                                }
                            })
                        })
                        .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;
                    trait_modules.push((
                        "postgres_async_read_dispatch",
                        quote::quote! {
                        #feature_flag
                        impl web_common_traits::prelude::AsyncReadDispatch<#async_connection> for super::Row {
                            type PrimaryKey = #table_primary_keys_path;

                            async fn read(
                                primary_key: Self::PrimaryKey,
                                conn: &mut #async_connection,
                            ) -> Result<Option<Self>, diesel::result::Error> {
                                match primary_key {
                                    #(
                                        #async_read_impls
                                    )*
                                }
                            }
                        }
                    }));
                }
                Ok::<(), crate::errors::WebCodeGenError>(())
            })?;

        for (trait_module_name, trait_impl) in trait_modules {
            let trait_file = root.join(format!("{}.rs", trait_module_name));
            let trait_module_ident = Ident::new(trait_module_name, proc_macro2::Span::call_site());
            std::fs::write(
                &trait_file,
                self.beautify_code(&quote! {
                    #trait_impl
                })?,
            )?;
            modules.push(quote::quote! {
                mod #trait_module_ident;
            });
        }

        let row_file = root.with_extension("rs");
        std::fs::write(
            &row_file,
            self.beautify_code(&quote! {
                #(
                    #modules
                )*

                #row_enumeration

                impl Row {
                    #include_upsert
                }
            })?,
        )?;

        Ok(())
    }
}
