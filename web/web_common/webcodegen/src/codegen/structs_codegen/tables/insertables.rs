//! Submodule defining the structs supporting the [`Insertable`] and
//! [`Insertable`]-adjacent traits.

use std::path::Path;

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    Codegen, Column, Table,
    codegen::{
        CODEGEN_DIRECTORY, CODEGEN_INSERTABLES_PATH, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
        Syntax,
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
    pub(super) async fn generate_insertable_structs(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut insertables_main_module = TokenStream::new();
        let syntax = Syntax::PostgreSQL;

        for table in tables {
            let all_columns = table.columns(conn).await?;
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

            let mut insertable_attributes = Vec::new();

            for column in &insertable_columns {
                let column_name = column.snake_case_ident()?;
                let column_type = column.rust_data_type(conn).await?;
                insertable_attributes.push(quote::quote! {
                    #column_name: #column_type
                });
            }

            let insertable_variant_methods = table.foreign_key_methods(conn, &syntax).await?;

            let mut insertable_builder_attributes = Vec::new();

            for column in &nullable_insertable_columns {
                let column_name = column.snake_case_ident()?;
                let column_type = column.rust_data_type(conn).await?;
                insertable_builder_attributes.push(quote::quote! {
                    #column_name: #column_type
                });
            }
            let populating_product = insertable_columns.iter().map(|column| {
                    let column_ident = column.snake_case_ident()?;
                    Ok(if column.is_nullable() {
                        quote::quote! {
                            #column_ident: self.#column_ident
                        }
                    } else {
                        let camel_cased_column_ident = column.camel_case_ident()?;
                        quote::quote! {
                            #column_ident: self.#column_ident.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(#insertable_enum::#camel_cased_column_ident))?
                        }
                    })
                }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
            let mut insertable_builder_methods: Vec<TokenStream> = Vec::new();

            for column in &insertable_columns {
                let column_name = column.snake_case_ident()?;
                let column_type = column.rust_data_type(conn).await?;

                let mut check_constraints = TokenStream::new();

                for constraint in column.check_constraints(conn).await? {
                    if constraint.is_postgis_constraint() {
                        continue;
                    }

                    let outcome = constraint
                        .to_syn(
                            &[column],
                            &nullable_insertable_columns,
                            self.check_constraints_extensions.as_slice(),
                            &insertable_enum,
                            conn,
                        )
                        .await;
                    if let Err(WebCodeGenError::CodeGenerationError(
                        CodeGenerationError::CheckConstraintError(
                            CheckConstraintError::NoInvolvedColumns(unknown_column, _),
                        ),
                    )) = &outcome
                    {
                        if all_columns.contains(unknown_column.as_ref())
                            && !insertable_columns.contains(&unknown_column.as_ref())
                        {
                            continue;
                        }
                    }
                    check_constraints.extend(outcome?);
                }

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

                // If the current table has both `created_by` and `updated_by`, and
                // we are currently assigning the `created_by` column, we need to
                // assign the `updated_by` column as well.
                let updated_by_exception = if column.is_created_by(conn).await
                    && table.has_updated_by_column(conn).await?
                {
                    quote::quote! {
                        self = self.updated_by(#column_name)?;
                    }
                } else {
                    TokenStream::new()
                };

                let attribute = {
                    let camel_cased = column.camel_case_ident()?;
                    quote::quote! { #insertable_enum::#camel_cased }
                };

                insertable_builder_methods.push(quote::quote! {
                    pub fn #column_name<P>(
                        mut self, #column_name: P
                    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
                    where
                        P: TryInto<#column_type>,
                        <P as TryInto<#column_type>>::Error: Into<validation_errors::SingleFieldError>,
                    {
                        let #column_name = #column_name
                            .try_into()
                            .map_err(|err: <P as TryInto<#column_type>>::Error| {
                                Into::into(err)
                                    .rename_field(#attribute)
                            })?;
                        #check_constraints
                        #column_assignment
                        #updated_by_exception
                        Ok(self)
                    }
                });
            }

            let table_diesel_ident = table.import_diesel_path()?;

            let insertable_variant_builder_assignments = insertable_columns
                .iter()
                .map(|column| {
                    let column_name = column.snake_case_ident()?;
                    Ok(quote::quote! {
                        .#column_name(insertable_variant.#column_name)
                    })
                })
                .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

            let has_default_types = insertable_columns.iter().any(|column| column.has_default());

            let insertable_builder_default_derive = if has_default_types {
                // We need to manually implement Default for the insertable variant
                TokenStream::new()
            } else {
                // Since no column has some special default type, we can
                // derive Default for the insertable variant
                quote::quote! {
                    #[derive(Default)]
                }
            };

            let insertable_builder_default_impl = if has_default_types {
                let mut default_impl_attributes = Vec::new();

                for insertable_column in &insertable_columns {
                    let column_name = insertable_column.snake_case_ident()?;
                    if insertable_column.has_default() {
                        let default_value = insertable_column.rust_default_value(conn).await?;
                        default_impl_attributes.push(quote::quote! {
                            #column_name: Some(#default_value)
                        });
                    } else {
                        default_impl_attributes.push(quote::quote! {
                            #column_name: None
                        });
                    }
                }

                quote::quote! {
                    impl Default for #insertable_builder_ident {
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

            let ifv_file = root.join(format!("{}.rs", table.snake_case_name()?));
            std::fs::write(
                    &ifv_file,
                    self.beautify_code(&quote::quote! {
                        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
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

                        #insertable_builder_default_derive
                        pub struct #insertable_builder_ident {
                            #(#insertable_builder_attributes),*
                        }

                        #insertable_builder_default_impl

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
                                Self::default() #(#insertable_variant_builder_assignments)?*
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
