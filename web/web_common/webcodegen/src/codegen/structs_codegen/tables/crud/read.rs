//! Submodule providing the code generation for `Read` CRUD operations.

use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Table};

impl Codegen<'_> {
    #[allow(clippy::too_many_lines)]
    /// Generate implementations of the structs able to implement the
    /// [`Insertable`] and [`Insertable`]-adjacent traits for the provided
    /// tables.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The tables for which to generate the diesel code.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(crate) fn generate_read_structs(
        &self,
        root: &Path,
        tables: &[Table],
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let f2b_variants: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_path = table.import_struct_path()?;
                Ok(quote::quote! {
                    #struct_ident(ReadAll<#struct_path>)
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let b2f_variants: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_path = table.import_struct_path()?;
                Ok(quote::quote! {
                    #struct_ident(Vec<#struct_path>)
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

        let f2b_read_enumeration = quote::quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum F2BReadAll {
                #(#f2b_variants),*
            }
        };

        let b2f_read_enumeration = quote::quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum B2FReadAll {
                #(#b2f_variants),*
            }
        };

        let from_impls: TokenStream = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let struct_path = table.import_struct_path()?;
                Ok(quote! {
                    impl From<ReadAll<#struct_path>> for F2BReadAll {
                        fn from(value: ReadAll<#struct_path>) -> Self {
                            F2BReadAll::#struct_ident(value)
                        }
                    }
                    impl From<Vec<#struct_path>> for B2FReadAll {
                        fn from(value: Vec<#struct_path>) -> Self {
                            B2FReadAll::#struct_ident(value)
                        }
                    }
                    impl TryFrom<B2FReadAll> for Vec<#struct_path> {
                        type Error = ReadError;
                        fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
                            match value {
                                B2FReadAll::#struct_ident(v) => Ok(v),
                                _ => Err(ReadError::ConversionError)
                            }
                        }
                    }
                })
            })
            .collect::<Result<_, crate::errors::WebCodeGenError>>()?;

        let reads_file = root.with_extension("rs");
        std::fs::write(
            &reads_file,
            self.beautify_code(&quote! {
                use web_common_traits::crud::ReadAll;

                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub enum ReadError {
                    ConversionError
                }

                #f2b_read_enumeration
                #b2f_read_enumeration
                #from_impls
            })?,
        )?;

        Ok(())
    }
}
