//! Submodule handling the generation of insertable enums and relative traits
//! and structs.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    Codegen, Column, Table,
    codegen::{
        CODEGEN_DIRECTORY, CODEGEN_INSERTABLES_PATH, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
    },
    errors::WebCodeGenError,
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

    /// Returns the name for the extension attributes.
    pub fn insertable_extension_enum_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("Insertable{}ExtensionAttributes", self.struct_name()?))
    }

    /// Returns the [`Ident`](syn::Ident) for the extension attributes.
    pub fn insertable_extension_enum_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.insertable_extension_enum_name()?, proc_macro2::Span::call_site()))
    }

    /// Returns the [`Type`](syn::Type) for the extension attributes.
    pub fn insertable_extension_enum_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.insertable_extension_enum_name()?
        ))?)
    }

    /// Returns the extension enum for the insertable attributes of the
    /// current [`Table`].
    ///
    /// # Arguments
    ///
    /// * `extension_table` - The table for which the extension enum is
    ///   generated.
    /// * `ident` - The identifier for the enum variant.
    pub fn into_extension_field_name(
        &self,
        extension_table: &Table,
        ident: Ident,
    ) -> Result<TokenStream, WebCodeGenError> {
        let insertable_enum = self.insertable_enum_ident()?;
        let insertable_extension_enum = self.insertable_extension_enum_ident()?;
        let struct_ident = extension_table.struct_ident()?;

        Ok(quote::quote! {
            #insertable_enum::Extension(
                #insertable_extension_enum::#struct_ident(#ident)
            )
        })
    }

    /// Returns the enum path lambda for the insertable attributes of the
    /// extension of the current [`Table`].
    ///
    /// # Arguments
    ///
    /// * `extension_table` - The table for which the lambda is generated.
    pub fn into_extension_field_name_lambda(
        &self,
        extension_table: &Table,
    ) -> Result<TokenStream, WebCodeGenError> {
        let enum_field_name = self.into_extension_field_name(
            extension_table,
            syn::Ident::new("attribute", proc_macro2::Span::call_site()),
        )?;

        Ok(quote::quote! {
            |attribute| #enum_field_name
        })
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

        let insertable_extension_enum = self.insertable_extension_enum_ident()?;
        let mut display_insertable_extension_enum_variants = Vec::new();
        let mut insertable_extension_enum_variants = Vec::new();

        for extension_table in extension_tables {
            let struct_ident = extension_table.struct_ident()?;
            let extension_table_enum_ty = extension_table.insertable_enum_ty()?;
            display_insertable_extension_enum_variants.push(quote::quote! {
                Self::#struct_ident(e) => write!(f, "{e}")
            });
            insertable_extension_enum_variants.push(quote::quote! {
                #struct_ident(#extension_table_enum_ty)
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
        }))
    }

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
        let insertable_enum = self.insertable_enum_ident()?;
        let extension_tables = self.extension_tables(conn)?;
        let insertable_extension_enum_definition =
            self.insertable_extension_enum_definition(conn)?;
        let mut insertable_enum_variants = Vec::new();
        let mut display_insertable_enum_variants = Vec::new();

        if !extension_tables.is_empty() {
            let insertable_extension_enum = self.insertable_extension_enum_ident()?;
            insertable_enum_variants.push(quote::quote! {
                Extension(#insertable_extension_enum)
            });
            display_insertable_enum_variants.push(quote::quote! {
                Self::Extension(e) => write!(f, "{e}")
            });
        }

        for insertable_column in insertable_columns {
            let enum_variant = insertable_column.camel_case_ident()?;

            if let Some(foreign_key) = insertable_column.requires_partial_builder(conn)? {
                let foreign_table_enum =
                    foreign_key.foreign_table(conn)?.unwrap().insertable_enum_ty()?;
                insertable_enum_variants.push(quote::quote! {
                    #enum_variant(#foreign_table_enum)
                });
                display_insertable_enum_variants.push(quote::quote! {
                    Self::#enum_variant(e) => write!(f, "{e}")
                });
            } else {
                insertable_enum_variants.push(quote::quote! {
                    #enum_variant
                });
                let enum_variant_name = insertable_column.snake_case_name()?;
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

        for foreign_key in self.foreign_keys(conn)? {
            if foreign_key.is_singleton_foreign_key(conn)?
                && (foreign_key.is_same_as_constraint(conn)?.is_some()
                    || foreign_key.is_extension(conn)?)
            {
                singleton_foreign_keys.push(foreign_key);
            }
        }

        let from_implementations = singleton_foreign_keys
            .into_iter()
            .map(|foreign_key| {
                let foreign_table = foreign_key.foreign_table(conn)?.unwrap();
                let foreign_table_enum_ty = foreign_table.insertable_enum_ty()?;
                let foreign_table_snake_case_ident = foreign_table.snake_case_ident()?;
                let foreign_table_camel_case_ident = foreign_table.struct_ident()?;
                let conversion = if foreign_key.is_extension(conn)? {
                    let extension_table_enum_ident = self.insertable_extension_enum_ident()?;
                    quote::quote! {
                        Self::Extension(
                            #extension_table_enum_ident::#foreign_table_camel_case_ident(#foreign_table_snake_case_ident)
                        )
                    }
                } else if foreign_key.is_same_as_constraint(conn)?.is_some() {
                    let local_columns = foreign_key.columns(conn)?;
                    assert_eq!(
                        local_columns.len(),
                        1,
                        "Foreign key `{}` on `{}.{}` must have exactly one column",
                        foreign_key.constraint_name,
                        foreign_key.table_name,
                        foreign_key.column_name
                    );
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

        Ok(quote::quote! {
            #insertable_extension_enum_definition

            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #insertable_enum {
                #(#insertable_enum_variants),*
            }

            #(#from_implementations)*

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

impl Codegen<'_> {
    /// Returns the enum path lambda for the insertable attributes of the
    /// provided [`Column`] for the current [`Table`].
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which the lambda is generated.
    /// * `column` - The column for which the lambda is generated.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub fn into_field_name_lambda(
        &self,
        table: &Table,
        column: &Column,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        // If the column is from the current table, it is an error
        // to call this method.

        assert!(
            !table.has_column(column),
            "The column `{}.{}` is from the current table `{}`, cannot generate a lambda for it",
            column.table_name,
            column.column_name,
            table.table_name
        );

        // We retrieve the path of foreign keys to the column.
        let foreign_key_path = self
            .table_extension_network()
            .unwrap()
            .extension_foreign_keys_path(table, column, conn)
            .expect("The column must have a foreign key path");

        // We only care about the first ancestor of the foreign key path.
        let first_ancestor =
            foreign_key_path.first().expect("The foreign key path must have at least one ancestor");

        Ok(if first_ancestor.is_singleton_foreign_key(conn)? {
            // If the first ancestor is a singleton foreign key, it means
            // that the enum implements the `From` trait for the conversion
            // from the enum associated with the foreign key to the current enum.
            // Therefore, we can simply use `From::from` to convert the
            // attribute to the current enum.
            quote::quote! {
                From::from
            }
        } else {
            // Otherwise, we cannot rely on the `From` trait, and we need to
            // generate a custom lambda that converts the attribute
            // to the current enum.
            if first_ancestor.is_extension(conn)? {
                let foreign_table = first_ancestor.foreign_table(conn)?.unwrap();
                table.into_extension_field_name_lambda(&foreign_table)?
            } else {
                // If the first ancestor is not an extension, we can simply
                // return the enum variant associated with the column.
                let insertable_enum_type = table.insertable_enum_ty()?;
                let local_columns = first_ancestor.columns(conn)?;
                assert_eq!(
                    local_columns.len(),
                    1,
                    "Foreign key `{}` on `{}.{}` must have exactly one column",
                    first_ancestor.constraint_name,
                    first_ancestor.table_name,
                    first_ancestor.column_name
                );
                let local_column_camel_cased = local_columns[0].camel_case_ident()?;
                quote::quote! {
                    #insertable_enum_type::#local_column_camel_cased
                }
            }
        })
    }

    /// Returns the enum for the insertable attributes of the current
    /// [`Table`] for the provided [`Column`].
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which the enum is generated.
    /// * `column` - The column for which the lambda is generated.
    /// * `passing_through` - An optional column that is the foreign key to the
    ///   associated table which contains the column.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub fn column_enum_path(
        &self,
        table: &Table,
        column: &Column,
        passing_through: Option<&Column>,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let insertable_enum = table.insertable_enum_ty()?;
        let column_ident = column.camel_case_ident()?;

        if table.has_column(column) {
            assert!(
                passing_through.is_none(),
                "Passing through column must be None if the column is from the current table"
            );
            // If the provided column is from the current table, we can simply return the
            // associated enum variant.
            return Ok(quote::quote! {
                #insertable_enum::#column_ident
            });
        }

        if let Some(passing_through) = passing_through {
            if table.has_column(passing_through) {
                let passing_through_ident = passing_through.camel_case_ident()?;
                let foreign_key = passing_through
                    .requires_partial_builder(conn)?
                    .expect("Passing through column must require a partial builder");
                let foreign_table = foreign_key.foreign_table(conn)?.unwrap();
                assert_eq!(
                    foreign_table,
                    column.table(conn)?,
                    "The passing through column must be from the same table as the column"
                );
                let recursion = self.column_enum_path(&foreign_table, column, None, conn)?;
                return Ok(quote::quote! {
                    #insertable_enum::#passing_through_ident(#recursion)
                });
            }
        }

        let path = self.table_extension_network().unwrap().extension_path(table, column).unwrap();
        let insertable_extension_enum = table.insertable_extension_enum_ty()?;
        let extension_table = path[0];
        let extension_table_ident = extension_table.struct_ident()?;
        let recursion = self.column_enum_path(extension_table, column, passing_through, conn)?;

        Ok(quote::quote! {
            #insertable_enum::Extension(
                #insertable_extension_enum::#extension_table_ident(#recursion)
            )
        })
    }
}
