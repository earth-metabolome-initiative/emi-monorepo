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

    /// Returns the list of insertable columns for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    fn insertable_columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, WebCodeGenError> {
        let all_columns = self.columns(conn)?;
        Ok(all_columns
            .into_iter()
            .filter(|column| !column.is_always_automatically_generated())
            .collect())
    }

    /// Returns the list of insertable columns for the table and all parent
    /// extension tables, automatically removing the primary key column which
    /// links the extension table to the parent table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    fn insertable_columns_with_extension(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        let mut insertable_columns = self.insertable_columns(conn)?;
        if let Some(extension_table) = self.extension_table(conn)? {
            let primary_key_columns = self.primary_key_columns(conn)?;

            for column in primary_key_columns {
                // We remove the primary key column from the insertable columns,
                // as it is handled by the extension table.
                if let Some(index) = insertable_columns.iter().position(|c| c == &column) {
                    insertable_columns.remove(index);
                }
            }

            let extension_columns = extension_table.insertable_columns_with_extension(conn)?;
            insertable_columns.extend(extension_columns);
        }
        Ok(insertable_columns)
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

        let Some(user_table) = self.users_table else {
            return Err(crate::errors::CodeGenerationError::UserTableNotProvided.into());
        };
        let user_id_type = user_table.primary_key_type(conn)?;

        for table in tables {
            let extension_table = table.extension_table(conn)?;
            let primary_key_columns = table.primary_key_columns(conn)?;
            let extension_pk =
                if extension_table.is_some() { Some(primary_key_columns[0].clone()) } else { None };
            let nullable_extension_pk = extension_pk.as_ref().map(|column| column.to_nullable());
            let all_columns = table.columns(conn)?;
            let insertable_columns = table.insertable_columns(conn)?;
            let nullable_insertable_columns: Vec<Column> =
                insertable_columns.iter().map(|column| column.to_nullable()).collect();

            let insertable_enum = table.insertable_enum_ident()?;
            let insertable_variant_ident = table.insertable_variant_ident()?;
            let insertable_builder_ident = table.insertable_builder_ident()?;
            let insertable_enum_variants = insertable_columns
                .iter()
                .map(|column| {
                    let enum_variant = column.camel_case_ident()?;
                    Ok(if extension_pk.as_ref() == Some(column) {
                        let extension_enum_type =
                            extension_table.as_ref().unwrap().insertable_enum_ty()?;
                        quote::quote! {
                            #enum_variant(#extension_enum_type)
                        }
                    } else {
                        quote::quote! {
                            #enum_variant
                        }
                    })
                })
                .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

            let display_insertable_enum_variants = insertable_columns
                .iter()
                .map(|column| {
                    let enum_variant = column.camel_case_ident()?;
                    let column_ident = column.snake_case_ident()?;
                    if extension_pk.as_ref() == Some(column) {
                        Ok(quote::quote! {
                            #insertable_enum::#enum_variant(#column_ident) => write!(f, "{}", #column_ident)
                        })
                    } else {
                        let column_name = column.snake_case_name()?;
                        Ok(quote::quote! {
                            #insertable_enum::#enum_variant => write!(f, #column_name)
                        })
                    }
                })
                .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

            let mut insertable_attributes = Vec::new();

            for column in &insertable_columns {
                let column_name = column.snake_case_ident()?;
                let column_type = column.rust_data_type(conn)?;
                insertable_attributes.push(quote::quote! {
                    #column_name: #column_type
                });
            }

            let insertable_variant_methods = table.foreign_key_methods(conn)?;

            let mut insertable_builder_attributes = Vec::new();

            for column in &nullable_insertable_columns {
                let column_name = column.snake_case_ident()?;
                let column_type = if nullable_extension_pk.as_ref() == Some(column) {
                    extension_table.as_ref().unwrap().insertable_builder_ty()?
                } else {
                    column.rust_data_type(conn)?
                };
                insertable_builder_attributes.push(quote::quote! {
                    #column_name: #column_type
                });
            }
            let mut insertable_builder_methods: Vec<TokenStream> = Vec::new();

            for column in &insertable_columns {
                if extension_pk.as_ref() == Some(column) {
                    // We skip the extension primary key column, as it is handled
                    // by the extension table.
                    continue;
                }

                let column_name = column.snake_case_ident()?;
                let column_type = column.rust_data_type(conn)?;

                let mut check_constraints = TokenStream::new();

                for constraint in column.check_constraints(conn)? {
                    if constraint.is_postgis_constraint() {
                        continue;
                    }

                    let outcome = constraint.to_syn(
                        &[column],
                        &nullable_insertable_columns,
                        self.check_constraints_extensions.as_slice(),
                        &insertable_enum,
                        conn,
                    );
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
                let updated_by_exception =
                    if column.is_created_by(conn)? && table.has_updated_by_column(conn)? {
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
                    ) -> Result<Self, web_common_traits::database::InsertError<#insertable_enum>>
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

            // We re-export all of the setter methods associated with the extended
            // table, if any.
            if let (Some(extension_table), Some(extension_pk)) = (&extension_table, &extension_pk) {
                let extension_pk_ident = extension_pk.snake_case_ident()?;
                for column in extension_table.insertable_columns_with_extension(conn)? {
                    let column_name = column.snake_case_ident()?;
                    let column_type = column.rust_data_type(conn)?;

                    insertable_builder_methods.push(quote::quote! {
                        pub fn #column_name<P>(
                            mut self, #column_name: P
                        ) -> Result<Self, web_common_traits::database::InsertError<#insertable_enum>>
                        where
                            P: TryInto<#column_type>,
                            <P as TryInto<#column_type>>::Error: Into<validation_errors::SingleFieldError>,
                        {
                            self.#extension_pk_ident = self.#extension_pk_ident.#column_name(#column_name).map_err(|err| {
                                err.into_field_name()
                            })?;
                            Ok(self)
                        }
                    });
                }
            }

            let table_diesel_ident = table.import_diesel_path()?;
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
                        let default_value = insertable_column.rust_default_value(conn)?;
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

            let maybe_enum_from_impl = if let (Some(extension_table), Some(extension_pk)) =
                (&extension_table, &extension_pk)
            {
                let extension_table_enum = extension_table.insertable_enum_ty()?;
                let extension_enum_variant = extension_pk.camel_case_ident()?;
                quote::quote! {
                    impl From<#extension_table_enum> for #insertable_enum {
                        fn from(extension: #extension_table_enum) -> Self {
                            Self::#extension_enum_variant(extension)
                        }
                    }
                }
            } else {
                TokenStream::new()
            };

            // When the table associated with the struct we are generating is not an
            // extension, we can implement the `TryFrom` trait to convert the insertable
            // builder into the insertable variant, which we will then be able to directly
            // insert into the database.
            let try_into_insert_impl = if let (Some(extension_table), Some(extension_pk)) =
                (&extension_table, &extension_pk)
            {
                let populating_product = insertable_columns.iter().map(|column| {
                    if extension_pk == column {
                        // We skip the extension primary key column, as it is handled
                        // by the extension table.
                        return Ok(TokenStream::new());
                    }
                    let column_ident = column.snake_case_ident()?;
                    Ok(if column.is_nullable() {
                        quote::quote! {
                            #column_ident: self.#column_ident,
                        }
                    } else {
                        let camel_cased_column_ident = column.camel_case_ident()?;
                        quote::quote! {
                            #column_ident: self.#column_ident.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(#insertable_enum::#camel_cased_column_ident))?,
                        }
                    })
                }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
                let extension_insertable_builder = extension_table.insertable_builder_ty()?;
                let extension_pk_ident = extension_pk.snake_case_ident()?;
                let extension_row = extension_table.import_struct_path()?;
                let extension_attributes_enum = extension_table.insertable_enum_ty()?;
                quote::quote! {
                    impl #insertable_builder_ident {
                        pub(crate) fn try_insert<C>(
                            self,
                            user_id: #user_id_type,
                            conn: &mut C
                        ) -> Result<
                            #insertable_variant_ident,
                            web_common_traits::database::InsertError<#insertable_enum>
                        >
                            where
                                #extension_insertable_builder: web_common_traits::database::InsertableVariant<
                                    C,
                                    UserId = #user_id_type,
                                    Row = #extension_row,
                                    Error = web_common_traits::database::InsertError<#extension_attributes_enum>
                                >,
                        {
                            use web_common_traits::database::InsertableVariant;
                            use diesel::associations::Identifiable;
                            Ok(#insertable_variant_ident {
                                #(#populating_product)*
                                #extension_pk_ident: self.#extension_pk_ident.insert(user_id, conn).map_err(|err| {
                                    err.into_field_name()
                                })?.id()
                            })
                        }
                    }
                }
            } else {
                let populating_product = insertable_columns.iter().map(|column| {
                    let column_ident = column.snake_case_ident()?;
                    Ok(if column.is_nullable() {
                        quote::quote! {
                            #column_ident: builder.#column_ident
                        }
                    } else {
                        let camel_cased_column_ident = column.camel_case_ident()?;
                        quote::quote! {
                            #column_ident: builder.#column_ident.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(#insertable_enum::#camel_cased_column_ident))?
                        }
                    })
                }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

                quote::quote! {
                    impl TryFrom<#insertable_builder_ident> for #insertable_variant_ident {
                        type Error = common_traits::prelude::BuilderError<#insertable_enum>;

                        fn try_from(builder: #insertable_builder_ident) -> Result<#insertable_variant_ident, Self::Error> {
                            Ok(Self {
                                #(#populating_product),*
                            })
                        }
                    }
                }
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

                        #maybe_enum_from_impl

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

                        #try_into_insert_impl
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
