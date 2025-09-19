//! Submodule handling the generation of insertable enums and relative traits
//! and structs.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    Table,
    codegen::{
        CODEGEN_DIRECTORY, CODEGEN_INSERTABLES_PATH, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
    },
    errors::WebCodeGenError,
    traits::TableLike,
};

impl Table {
    /// Returns the name for the attributes that may be set in the insertable
    /// variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder attributes cannot be
    ///   retrieved.
    pub fn attributes_enum_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("{}Attribute", self.struct_name()?))
    }

    /// Returns the [`Ident`](syn::Ident) for the attributes that may be set in
    /// the insertable variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder attributes cannot be
    ///   retrieved.
    pub fn attributes_enum_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.attributes_enum_name()?, proc_macro2::Span::call_site()))
    }

    /// Returns the name for the extension attributes.
    ///
    /// # Errors
    ///
    /// * If the name of the extension attributes cannot be retrieved.
    fn attributes_extension_enum_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("{}ExtensionAttribute", self.struct_name()?))
    }

    /// Returns the [`Ident`](syn::Ident) for the extension attributes.
    ///
    /// # Errors
    ///
    /// * If the name of the extension attributes cannot be retrieved.
    pub(super) fn attributes_extension_enum_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.attributes_extension_enum_name()?, proc_macro2::Span::call_site()))
    }

    /// Returns the [`Type`](syn::Type) for the extension attributes.
    ///
    /// # Errors
    ///
    /// * If the name of the extension attributes cannot be retrieved.
    pub fn attributes_extension_enum_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.attributes_extension_enum_name()?
        ))?)
    }

    /// Returns the [`Type`](syn::Type) for the insertable attributes.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable attributes cannot be retrieved.
    pub fn insertable_enum_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.attributes_enum_name()?
        ))?)
    }

    /// Returns the definition and implementation of the extension attributes
    /// enumeration for the current [`Table`].
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    fn insertable_extension_enum_definition(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        let extension_tables = self.extension_tables(conn)?;

        if extension_tables.is_empty() {
            // If there is only one extension table, we do not need a separate enum for the
            // insertable attributes.
            return Ok(None);
        }

        let insertable_extension_enum = self.attributes_extension_enum_ident()?;
        let mut display_insertable_extension_enum_variants = Vec::new();
        let mut insertable_extension_enum_variants = Vec::new();
        let mut from_implementations = Vec::new();
        let table_name = self.table_name.as_str();
        let display_pattern = format!("{table_name}({{e}})");

        for extension_table in extension_tables.iter() {
            let struct_ident = extension_table.struct_ident()?;
            let extension_table_enum_ty = extension_table.insertable_enum_ty()?;
            display_insertable_extension_enum_variants.push(quote::quote! {
                Self::#struct_ident(e) => write!(f, #display_pattern)
            });
            insertable_extension_enum_variants.push(quote::quote! {
                #struct_ident(#extension_table_enum_ty)
            });
            from_implementations.push(quote::quote! {
                impl From<#extension_table_enum_ty> for #insertable_extension_enum {
                    fn from(attribute: #extension_table_enum_ty) -> Self {
                        Self::#struct_ident(attribute)
                    }
                }
            });
        }

        if !extension_tables.is_empty() {
            from_implementations.push(quote::quote! {
                impl From<common_traits::builder::EmptyTuple> for #insertable_extension_enum {
                    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
                        unreachable!("Some code generation error occurred to reach this point.")
                    }
                }
            });
        }

        Ok(Some(quote::quote! {
            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #insertable_extension_enum {
                #(#insertable_extension_enum_variants),*
            }

            impl core::fmt::Display for #insertable_extension_enum {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    match self {
                        #(#display_insertable_extension_enum_variants),*
                    }
                }
            }

            #(#from_implementations)*
        }))
    }

    /// Returns the implementation of the `FromStr` trait for the insertable
    /// attributes enumeration.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    fn insertable_enum_from_str_impl(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        // We retrieve the insertable columns for the current table, without
        // the extension tables (i.e. not executing recursion to the ancestors).
        let insertable_columns = self.insertable_columns(conn, true)?;

        // We obtain the enum identifier.
        let insertable_enum = self.attributes_enum_ident()?;

        // We obtain the identifiers for the columns.
        let mut column_camel_case_idents = Vec::new();
        let mut column_snake_case_names = Vec::new();
        let mut column_camel_case_names = Vec::new();

        for insertable_column in &insertable_columns {
            let column_camel_case_ident = insertable_column.camel_case_ident()?;
            let column_snake_case_name = insertable_column.snake_case_name()?;
            let column_camel_case_name = insertable_column.camel_case_name()?;

            column_camel_case_idents.push(
                if let Some((_partial_builder_kind, _, partial_builder_constraint)) =
                    insertable_column.requires_partial_builder(conn)?
                {
                    let foreign_table = partial_builder_constraint
                        .foreign_table(conn)?;
                    assert!(!foreign_table.has_composite_primary_key(conn)?);
                    let foreign_primary_key_columns = foreign_table.primary_key_columns(conn)?;

                    let foreign_builder_enum_type = foreign_table.insertable_enum_ty()?;
                    let foreign_column_camel_case_ident = foreign_primary_key_columns[0].camel_case_ident()?;
                    quote::quote! {
                        #column_camel_case_ident(#foreign_builder_enum_type::#foreign_column_camel_case_ident)
                    }
                } else {
                    // Otherwise, we use the column identifier.
                    quote::quote! {
                        #column_camel_case_ident
                    }
                },
            );
            column_snake_case_names.push(column_snake_case_name);
            column_camel_case_names.push(column_camel_case_name);
        }

        // We generate the `FromStr` implementation for the insertable enum.
        Ok(quote::quote! {
            impl core::str::FromStr for #insertable_enum {
                type Err = web_common_traits::database::InsertError<Self>;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #(
                            // For each column, we match the string against the
                            // camel case name of the column.
                            #column_camel_case_names => Ok(Self::#column_camel_case_idents),
                        )*
                        // We also match against the snake case names.
                        #(
                            #column_snake_case_names => Ok(Self::#column_camel_case_idents),
                        )*
                        // If the string does not match any of the columns,
                        // we return an error.
                        _ => Err(
                            web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())
                        ),
                    }
                }
            }
        })
    }

    #[allow(clippy::too_many_lines)]
    /// Returns the definition and implementation of the attributes enumeration
    /// for the current [`Table`].
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    pub(super) fn insertable_enum_definition(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let insertable_columns = self.columns(conn)?;
        let insertable_enum = self.attributes_enum_ident()?;
        let extension_tables = self.extension_tables(conn)?;
        let insertable_extension_enum_definition =
            self.insertable_extension_enum_definition(conn)?;
        let mut insertable_enum_variants = Vec::new();
        let mut display_insertable_enum_variants = Vec::new();

        if !extension_tables.is_empty() {
            let insertable_extension_enum = self.attributes_extension_enum_ident()?;
            insertable_enum_variants.push(quote::quote! {
                Extension(#insertable_extension_enum)
            });
            display_insertable_enum_variants.push(quote::quote! {
                Self::Extension(e) => write!(f, "{e}")
            });
        }

        for insertable_column in insertable_columns.iter() {
            let enum_variant = insertable_column.camel_case_ident()?;

            if let Some((_partial_builder_kind, _, foreign_key)) =
                insertable_column.requires_partial_builder(conn)?
            {
                let foreign_table_enum = foreign_key.foreign_table(conn)?.insertable_enum_ty()?;
                insertable_enum_variants.push(quote::quote! {
                    #enum_variant(#foreign_table_enum)
                });
                let pattern = format!("{}.{{e}}", self.snake_case_name()?,);
                display_insertable_enum_variants.push(quote::quote! {
                    Self::#enum_variant(e) => write!(f, #pattern)
                });
            } else {
                insertable_enum_variants.push(quote::quote! {
                    #enum_variant
                });
                let enum_variant_name =
                    format!("{}.{}", self.snake_case_name()?, insertable_column.snake_case_name()?);
                display_insertable_enum_variants.push(quote::quote! {
                    Self::#enum_variant => write!(f, #enum_variant_name)
                });
            }
        }

        // For the singleton foreign key columns in the cases where there are no
        // multiple foreign keys to the same foreign table, we can define the
        // `From` trait to convert the insertable enum of the foreign table into
        // the insertable enum of the current table. For the cases where there
        // exist multiple foreign keys to the same foreign table, there will
        // need to apply a custom conversion.

        let mut singleton_foreign_keys = Vec::new();

        for foreign_key in self.foreign_keys(conn)?.iter() {
            if foreign_key.is_singleton(conn)?
                && (foreign_key.is_partial_builder_constraint(conn)?.is_some()
                    || foreign_key.is_extension(conn)?)
            {
                singleton_foreign_keys.push(foreign_key.clone());
            }
        }

        let from_implementations = singleton_foreign_keys
            .into_iter()
            .map(|foreign_key| {
                let foreign_table = foreign_key.foreign_table(conn)?;
                let foreign_table_enum_ty = foreign_table.insertable_enum_ty()?;
                let foreign_table_snake_case_ident = foreign_table.snake_case_ident()?;
                let foreign_table_camel_case_ident = foreign_table.struct_ident()?;
                let conversion = if foreign_key.is_extension(conn)? {
                    let extension_table_enum_ident = self.attributes_extension_enum_ident()?;
                    quote::quote! {
                        Self::Extension(
                            #extension_table_enum_ident::#foreign_table_camel_case_ident(#foreign_table_snake_case_ident)
                        )
                    }
                } else if foreign_key.is_partial_builder_constraint(conn)?.is_some() {
                    let local_columns = foreign_key.columns(conn)?;
                    let local_column_camel_cased = local_columns[0].camel_case_ident()?;

                    quote::quote! {
                        Self::#local_column_camel_cased(#foreign_table_snake_case_ident)
                    }
                } else {
                    unreachable!("Foreign key `{}` on `{}.{}` is not an extension or same as constraint", foreign_key.constraint_name, foreign_key.table_name, foreign_key.column_name)
                };

                Ok(quote::quote! {
                    impl From<#foreign_table_enum_ty> for #insertable_enum {
                        fn from(#foreign_table_snake_case_ident: #foreign_table_enum_ty) -> Self {
                            #conversion
                        }
                    }
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let from_str_impl = self.insertable_enum_from_str_impl(conn)?;
        let builder = self.insertable_builder_ty()?;

        let maybe_generics = if extension_tables.is_empty() {
            quote::quote! {}
        } else {
            let generics = extension_tables
                .iter()
                .enumerate()
                .map(|(table_number, _)| {
                    syn::Ident::new(
                        &format!("T{}", table_number + 1),
                        proc_macro2::Span::call_site(),
                    )
                })
                .collect::<Vec<_>>();
            quote::quote! { <#(#generics),*> }
        };

        Ok(quote::quote! {
            #insertable_extension_enum_definition

            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #insertable_enum {
                #(#insertable_enum_variants),*
            }

            #(#from_implementations)*

            #from_str_impl

            impl #maybe_generics common_traits::builder::Attributed for #builder #maybe_generics{
                type Attribute = #insertable_enum;
            }

            impl core::fmt::Display for #insertable_enum {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    match self {
                        #(#display_insertable_enum_variants),*
                    }
                }
            }
        })
    }
}
