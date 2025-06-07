//! Submodule generating an enumeration of all the tables in the database.
//! Submodule providing the code generation for `Read` CRUD operations.

use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    Codegen, Table,
    codegen::{CODEGEN_DIRECTORY, CODEGEN_TABLES_PATH},
};

impl Codegen<'_> {
    pub(crate) fn rows_enum_path() -> TokenStream {
        let codegen_ident = Ident::new(CODEGEN_DIRECTORY, proc_macro2::Span::call_site());
        let tables_module_ident = Ident::new(CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());
        quote::quote! {
            crate::#codegen_ident::#tables_module_ident::rows::Rows
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
    pub(crate) fn generate_rows_enumeration(
        &self,
        root: &Path,
        tables: &[Table],
        _conn: &mut diesel::PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        let table_name_enum_path = Self::table_names_enum_path();
        let table_primary_keys_path = Self::table_primary_keys_enum_path();
        let row_enum_path = Self::row_enum_path();
        let sqlite_upsert: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_name_ident = table.snake_case_ident()?;
                Ok(quote::quote! {
                    Rows::#struct_ident(#struct_name_ident) => {
                        #struct_name_ident.iter()
                            .filter_map(|entry| entry.upsert(conn).transpose())
                            .collect::<Result<Vec<_>, diesel::result::Error>>()?.into()
                    }
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let rows_table_names = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                Ok(quote::quote! {
                    super::Rows::#struct_ident(_) => #table_name_enum_path::#struct_ident,
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let rows_primary_keys = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_name_ident = table.snake_case_ident()?;
                Ok(quote::quote! {
                    Rows::#struct_ident(#struct_name_ident) => #struct_name_ident.primary_keys(),
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let rows_variants: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_path = table.import_struct_path()?;
                Ok(quote::quote! {
                    #struct_ident(Vec<#struct_path>)
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let rows_read_enumeration = quote::quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum Rows {
                #(#rows_variants),*
            }
        };

        let include_upsert = if self.enable_upsertable_trait {
            quote! {
                #[cfg(feature = "sqlite")]
                /// Executes the upsert operation and returns the result.
                pub fn sqlite_upsert(
                    &self,
                    conn: &mut diesel::SqliteConnection,
                ) -> Result<Rows, diesel::result::Error>
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
                    impl From<#struct_path> for super::Rows {
                        fn from(value: #struct_path) -> Self {
                            Self::from(vec![value])
                        }
                    }
                    impl From<Vec<#struct_path>> for super::Rows {
                        fn from(value: Vec<#struct_path>) -> Self {
                            super::Rows::#struct_ident(value)
                        }
                    }
                    impl TryFrom<super::Rows> for Vec<#struct_path> {
                        type Error = std::convert::Infallible;
                        fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
                            match value {
                                super::Rows::#struct_ident(v) => Ok(v),
                                value => {
                                    unreachable!("Unexpected variant in Rows enum: {value:?}")
                                }
                            }
                        }
                    }
                })?,
            )?;
        }

        let mut trait_modules: Vec<(String, TokenStream)> = Vec::new();

        trait_modules.push((
            "tabular".to_owned(),
            quote::quote! {
                impl web_common_traits::prelude::Tabular for super::Rows {
                    type TableName = #table_name_enum_path;

                    fn table_name(&self) -> Self::TableName {
                        match self {
                            #(
                                #rows_table_names
                            )*
                        }
                    }
                }
            },
        ));

        let len_impls = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                Ok(quote::quote! {
                    super::Rows::#struct_ident(rows) => rows.len(),
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        trait_modules.push((
            "len".to_owned(),
            quote::quote! {
                impl super::Rows {
                    pub fn len(&self) -> usize {
                        match self {
                            #(
                                #len_impls
                            )*
                        }
                    }

                    pub fn is_empty(&self) -> bool {
                        self.len() == 0
                    }
                }
            },
        ));

        let from_rows_to_row_vec_impls: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                Ok(quote::quote! {
                    super::Rows::#struct_ident(rows) => {
                        rows.into_iter()
                            .map(#row_enum_path::#struct_ident)
                            .collect::<Vec<_>>()
                    }
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        trait_modules.push((
            "into_iter".to_owned(),
            quote::quote! {
                impl From<super::Rows> for Vec<#row_enum_path> {
                    fn from(rows: super::Rows) -> Self {
                        match rows {
                            #(#from_rows_to_row_vec_impls)*
                        }
                    }
                }

                impl IntoIterator for super::Rows {
                    type Item = #row_enum_path;
                    type IntoIter = std::vec::IntoIter<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        let row_vec: Vec<#row_enum_path> = self.into();
                        row_vec.into_iter()
                    }
                }
            },
        ));

        let table_name_enum_path = table_name_enum_path.clone();
        let mut where_statements = Vec::new();
        let bounded_read_impls: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_path = table.import_struct_path()?;
                where_statements.push(quote::quote! {
                    #struct_path: web_common_traits::prelude::BoundedRead<C>
                });
                Ok(quote::quote! {
                    #table_name_enum_path::#struct_ident => {
                        #struct_path::bounded_read(offset, limit, conn)
                            .map(super::Rows::from)
                    }
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;
        trait_modules.push((
            "bounded_read_dispatch".to_owned(),
            quote::quote! {
                impl<C> web_common_traits::prelude::BoundedReadDispatch<C> for super::Rows
                where
                    #(#where_statements),*
                {
                    type TableName = #table_name_enum_path;

                    fn bounded_read(
                        table_name: Self::TableName,
                        offset: u16,
                        limit: u16,
                        conn: &mut C,
                    ) -> Result<Self, diesel::result::Error> {
                        use web_common_traits::database::BoundedRead;
                        match table_name {
                            #(
                                #bounded_read_impls
                            )*
                        }
                    }
                }
            },
        ));

        for (trait_module_name, trait_impl) in trait_modules {
            let trait_file = root.join(format!("{trait_module_name}.rs"));
            let trait_module_ident = Ident::new(&trait_module_name, proc_macro2::Span::call_site());
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

        let rows_file = root.with_extension("rs");
        std::fs::write(
            &rows_file,
            self.beautify_code(&quote! {
                #(
                    #modules
                )*

                #rows_read_enumeration

                impl Rows {
                    #include_upsert
                }

                impl web_common_traits::prelude::Rows for Rows {
                    type PrimaryKey = #table_primary_keys_path;

                    fn primary_keys(&self) -> Vec<Self::PrimaryKey> {
                        match self {
                            #(
                                #rows_primary_keys
                            )*
                        }
                    }
                }
            })?,
        )?;

        Ok(())
    }
}
