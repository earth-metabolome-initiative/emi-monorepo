//! Submodule defining the structs supporting the [`Insertable`] and
//! [`Insertable`]-adjacent traits.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    Codegen, Column, Table,
    codegen::{
        CODEGEN_DIRECTORY, CODEGEN_INSERTABLES_PATH, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
    },
    errors::{CheckConstraintError, CodeGenerationError, WebCodeGenError},
};

impl Table {
    /// Returns the name for the attributes that may be set in the insertable
    /// variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder attributes cannot be
    ///   retrieved.
    pub fn insertable_enum_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("Insertable{}Attributes", self.struct_name()?))
    }

    /// Returns the [`Ident`](syn::Ident) for the attributes that may be set in
    /// the insertable variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder attributes cannot be
    ///   retrieved.
    pub fn insertable_enum_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.insertable_enum_name()?, proc_macro2::Span::call_site()))
    }

    /// Returns the name for the insertable variant.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant cannot be retrieved.
    pub fn insertable_variant_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("Insertable{}", self.struct_name()?))
    }

    /// Returns the [`Type`](syn::Type) for the insertable attributes.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable attributes cannot be retrieved.
    pub fn insertable_enum_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.insertable_enum_name()?
        ))?)
    }

    /// Returns the [`Type`](syn::Type) for the insertable variant.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant cannot be retrieved.
    pub fn insertable_variant_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.insertable_variant_name()?
        ))?)
    }

    /// Returns the [`Ident`](syn::Ident) for the insertable variant.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant cannot be retrieved.
    pub fn insertable_variant_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.insertable_variant_name()?, proc_macro2::Span::call_site()))
    }

    /// Returns the name for the insertable variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder cannot be retrieved.
    pub fn insertable_builder_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("{}Builder", self.insertable_variant_name()?))
    }

    /// Returns the [`Ident`](syn::Ident) for the insertable variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder cannot be retrieved.
    pub fn insertable_builder_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.insertable_builder_name()?, proc_macro2::Span::call_site()))
    }

    /// Returns the [`Type`](syn::Type) for the insertable variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder cannot be retrieved.
    pub fn insertable_builder_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.insertable_builder_name()?
        ))?)
    }
}

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
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) fn generate_insertable_structs(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut insertables_main_module = TokenStream::new();

        for table in tables {
            let all_columns = table.columns(conn)?;
            let insertable_columns = all_columns
                .iter()
                .filter(|column| !column.is_always_automatically_generated())
                .collect::<Vec<_>>();
            let nullable_insertable_columns: Vec<Column> =
                insertable_columns.iter().map(|column| column.to_nullable()).collect();

            let insertable_enum = table.insertable_enum_ident()?;
            let insertable_variant_ident = table.insertable_variant_ident()?;
            let insertable_builder_ident = table.insertable_builder_ident()?;
            let insertable_enum_variants = insertable_columns
                .iter()
                .map(|column| column.camel_case_ident())
                .collect::<Result<Vec<Ident>, WebCodeGenError>>()?;
            let display_insertable_enum_variants = insertable_columns
                .iter()
                .map(|column| {
                    let enum_variant = column.camel_case_ident()?;
                    let column_name = column.snake_case_name()?;
                    Ok(quote::quote! {
                        #insertable_enum::#enum_variant => write!(f, #column_name)
                    })
                })
                .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

            let insertable_attributes = insertable_columns
                .iter()
                .map(|column| {
                    let column_name = column.snake_case_ident()?;
                    let column_type = column.rust_data_type(conn)?;
                    Ok(quote::quote! {
                        #column_name: #column_type
                    })
                })
                .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

            let insertable_variant_methods = table.foreign_key_methods(conn, &self.syntax)?;

            let insertable_builder_attributes = nullable_insertable_columns
                .iter()
                .map(|column| {
                    let column_name = column.snake_case_ident()?;
                    let column_type = column.rust_data_type(conn)?;
                    Ok(quote::quote! {
                        #column_name: #column_type
                    })
                })
                .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
            let populating_product = insertable_columns.iter().map(|column| {
                    let column_ident = column.snake_case_ident()?;
                    Ok(if column.is_nullable() {
                        quote::quote! {
                            #column_ident: self.#column_ident
                        }
                    } else {
                        let camel_cased_column_ident = column.camel_case_ident()?;
                        quote::quote! {
                            #column_ident: self.#column_ident.ok_or_else(|| common_traits::prelude::BuilderError::IncompleteBuild(#insertable_enum::#camel_cased_column_ident))?
                        }
                    })
                }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
            let insertable_builder_methods = insertable_columns
                    .iter()
                    .map(|column| {
                        let column_name = column.snake_case_ident()?;
                        let column_type = column.rust_data_type(conn)?;

                        let check_constraints = column.check_constraints(conn)?.into_iter().map(|constraint| {
                            if constraint.is_postgis_constraint() {
                                return Ok(TokenStream::new());
                            }

                            let outcome = constraint.to_syn(&[column], &nullable_insertable_columns, self.check_constraints_extensions.as_slice(), &insertable_enum, conn);
                            if let Err(WebCodeGenError::CodeGenerationError(CodeGenerationError::CheckConstraintError(CheckConstraintError::NoInvolvedColumns(unknown_column, _)))) = &outcome {
                                if all_columns.contains(unknown_column.as_ref()) && !insertable_columns.contains(&unknown_column.as_ref()) {
                                    return Ok(TokenStream::new());
                                }
                            }
                            outcome
                        }).collect::<Result<TokenStream, WebCodeGenError>>()?;

                        // TODO! Add `async` check for UNIQUE constraint when generating the code.
                        // Such check should only be active when the user is online.

                        let column_assignment = if column.is_nullable() {
                            quote::quote! {
                                self.#column_name = #column_name;
                            }
                        } else {
                            quote::quote! {
                                self.#column_name = Some(#column_name);
                            }
                        };
                        Ok(quote::quote! {
                            pub fn #column_name(mut self, #column_name: #column_type) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
                                #check_constraints
                                #column_assignment
                                Ok(self)
                            }
                        })
                    })
                    .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

            let table_diesel_ident = table.import_diesel_path()?;

            let insertable_variant_builder_assignments = insertable_columns
                .iter()
                .map(|column| {
                    let column_name = column.snake_case_ident()?;
                    Ok(quote::quote! {
                        .#column_name(insertable_variant.#column_name)?
                    })
                })
                .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

            let ifv_file = root.join(format!("{}.rs", table.snake_case_name()?));
            std::fs::write(
                    &ifv_file,
                    self.beautify_code(&quote::quote! {
                        #[derive(Clone, core::fmt::Debug)]
                        #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
                        pub enum #insertable_enum {
                            #(#insertable_enum_variants),*
                        }

                        impl core::fmt::Display for #insertable_enum {
                            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                                match self {
                                    #(#display_insertable_enum_variants),*
                                }
                            }
                        }

                        #[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
                        #[cfg_attr(any(feature = "postgres", feature = "sqlite"), diesel(table_name = #table_diesel_ident))]
                        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                        pub struct #insertable_variant_ident {
                            #(#insertable_attributes),*
                        }

                        impl #insertable_variant_ident {
                            #insertable_variant_methods
                        }

                        #[derive(Default)]
                        pub struct #insertable_builder_ident {
                            #(#insertable_builder_attributes),*
                        }

                        impl #insertable_builder_ident {
                            #(#insertable_builder_methods)*
                        }

                        impl common_traits::prelude::Builder for #insertable_builder_ident {
                            type Error = web_common_traits::database::InsertError<#insertable_enum>;
                            type Object = #insertable_variant_ident;
                            type Attribute = #insertable_enum;

                            fn build(self) -> Result<Self::Object, Self::Error> {
                                Ok(Self::Object {
                                    #(#populating_product),*
                                })
                            }
                        }

                        impl TryFrom<#insertable_variant_ident> for #insertable_builder_ident {
                            type Error = <Self as common_traits::prelude::Builder>::Error;
                            fn try_from(insertable_variant: #insertable_variant_ident) -> Result<Self, Self::Error> {
                                Ok(Self::default() #(#insertable_variant_builder_assignments)*)
                            }
                        }
                    })?,
                )?;

            let table_identifier = table.snake_case_ident()?;
            insertables_main_module.extend(quote::quote! {
                mod #table_identifier;
                pub use #table_identifier::{#insertable_variant_ident, #insertable_builder_ident, #insertable_enum};
            });
        }

        let insertables_file = root.with_extension("rs");
        std::fs::write(&insertables_file, self.beautify_code(&insertables_main_module)?)?;

        Ok(())
    }
}
