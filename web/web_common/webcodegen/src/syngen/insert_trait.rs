use common_traits::prelude::*;
use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{errors::WebCodeGenError, Column, Table};

impl Table {
    /// Returns the identifier for the session insert variant.
    pub fn session_insert_variant_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(
            &format!("SessionInsert{}", self.struct_name()?),
            proc_macro2::Span::call_site(),
        ))
    }

    /// Returns the identifier for the session insert variant builder.
    pub fn session_insert_variant_builder_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(
            &format!("SessionInsert{}Builder", self.struct_name()?),
            proc_macro2::Span::call_site(),
        ))
    }

    /// Returns the identifier for the enumeration of attributes that can be
    /// set.
    pub fn insertable_variant_attribute_enum_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(
            &format!("Insertable{}Attribute", self.struct_name()?),
            proc_macro2::Span::call_site(),
        ))
    }

    /// Returns the implementation of the `InsertableRow` from
    /// `web_common_traits` for the table.
    pub fn insertable_row_impl(&self) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let session_insert_variant_name = self.session_insert_variant_ident()?;
        let session_insert_variant_builder_name = self.session_insert_variant_builder_ident()?;

        Ok(quote! {
            impl web_common_traits::prelude::InsertableRow for #struct_name {
                type New = #session_insert_variant_name;
                type Builder = #session_insert_variant_builder_name;
            }
        })
    }

    /// Returns vector of the columns that compose the `SessionInsertVariant`.
    pub fn session_insert_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        Ok(self
            .columns(conn)?
            .into_iter()
            .filter(|column| !column.is_automatically_generated())
            .collect())
    }

    /// Returns the enumeration of the attributes that can be set.
    pub fn insertable_variant_attribute_enum(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let enum_name = self.insertable_variant_attribute_enum_ident()?;
        let columns = self.session_insert_columns(conn)?;
        let attributes = columns
            .iter()
            .map(|column| {
                let column_name = column.column_name.to_camel_case();
                Ident::new(&column_name, proc_macro2::Span::call_site())
            })
            .collect::<Vec<Ident>>();
        let display_attributes =
            columns.iter().zip(attributes.iter()).map(|(column, attribute)| {
                let display_name = format!("{}.{}", self.table_name, column.column_name);
                quote! {
                    #enum_name::#attribute => write!(f, #display_name),
                }
            });

        Ok(quote! {
            #[common_traits::prelude::basic]
            pub enum #enum_name {
                #(#attributes),*
            }

            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match self {
                        #(#display_attributes)*
                    }
                }
            }
        })
    }

    /// Returns the struct and implementation of the `InsertableVariant` trait
    /// for the table.
    pub fn insertable_session_variant_impl(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let table_name = self.snake_case_ident()?;
        let struct_name = self.struct_ident()?;
        let builder_name = self.session_insert_variant_builder_ident()?;
        let session_insert_variant_name = self.session_insert_variant_ident()?;
        let session_insert_variable_attributes = self
            .session_insert_columns(conn)?
            .into_iter()
            .map(|column| {
                let column_name = column.sanitized_snake_case_ident()?;
                let column_type = column.rust_data_type(conn)?;
                Ok(quote! {
                    #column_name: #column_type,
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;
        Ok(quote! {
            #[common_traits::prelude::basic]
            #[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
            #[cfg_attr(feature = "diesel", diesel(table_name = #table_name))]
            pub struct #session_insert_variant_name {
                #(#session_insert_variable_attributes)*
            }

            impl web_common_traits::prelude::InsertableVariant for #session_insert_variant_name {
                type Row = #struct_name;
                type Builder = #builder_name;

                #[cfg(feature = "backend")]
                /// Insert the variant into the database.
                async fn insert(
                    self,
                    conn: &mut web_common_traits::prelude::DBConn,
                ) -> Result<Self::Row, diesel::result::Error> {
                    diesel::insert_into(#table_name::table)
                        .values(self)
                        .get_result(conn)
                        .await
                }
            }
        })
    }

    /// Returns the struct and implementation of the `InsertBuider` trait for
    /// the table.
    ///
    /// # Implementative details
    ///
    /// The struct generated has optional fields for all columns present in the
    /// associated session table. The `build` method will return an error if
    /// any of the required fields are not set. Most importantly, the single
    /// setter methods hereditate the validation of the column by using the
    /// `CHECK` constraints defined in the database, and implemented concretely
    /// via the `pgrx` crates. Furthermore, inter-column table `CHECK`
    /// constraints are also implemented in the same manner.
    /// !TODO!: ACTUALLY IMPLEMENT PGRX-BASED MONOLITHIC VALIDATION!
    pub fn insertable_variant_builder_impl(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let session_insert_variant_name = self.session_insert_variant_ident()?;
        let enum_name = self.insertable_variant_attribute_enum_ident()?;
        let enum_impl = self.insertable_variant_attribute_enum(conn)?;
        let builder_ident = self.session_insert_variant_builder_ident()?;
        let attributes = self
            .session_insert_columns(conn)?
            .into_iter()
            .map(|column| {
                let column_name = column.sanitized_snake_case_ident()?;
                let column_type_str = column.str_rust_data_type(conn)?;
                let column_type: syn::Type = syn::parse_str(&column_type_str)?;
                Ok(quote! {
                    #column_name: Option<#column_type>,
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let completeness_check = self
            .session_insert_columns(conn)?
            .into_iter()
            .filter(|column| !column.is_nullable())
            .map(|column| {
                let column_name = column.sanitized_snake_case_ident()?;
                let camel_cased_column_name = column.column_name.to_camel_case();
                let camel_cased_column_ident =
                    Ident::new(&camel_cased_column_name, proc_macro2::Span::call_site());
                Ok(quote! {
                    let #column_name = self.#column_name.ok_or_else(|| {
                        common_traits::prelude::BuilderError::IncompleteBuild {
                            missing_attribute: #enum_name::#camel_cased_column_ident,
                        }
                    })?;
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let object_attributes = self
            .session_insert_columns(conn)?
            .into_iter()
            .map(|column| {
                let column_name = column.sanitized_snake_case_ident()?;
                Ok(if column.is_nullable() {
                    quote! {
                        #column_name: self.#column_name,
                    }
                } else {
                    quote! {
                        #column_name,
                    }
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let setter_methods = self.session_insert_columns(conn)?.iter().map(|column| {
            let column_name = column.sanitized_snake_case_ident()?;
            let column_type = column.rust_data_type(conn)?;
            // TODO! ADD VALIDATION BASED ON THE COLUMN CHECK CONSTRAINTS!

            // Depending on whether the table is nullable or not, we need to add the `Option` type
            let assignment = if column.is_nullable() {
                quote! {
                    self.#column_name = #column_name;
                }
            } else {
                quote! {
                    self.#column_name = Some(#column_name);
                }
            };

            Ok(quote! {
                pub fn #column_name(mut self, #column_name: #column_type) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
                    #assignment
                    Ok(self)
                }
            })
        }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        // TODO! ADD VALIDATION BASED ON THE TABLE CHECK CONSTRAINTS!
        let inter_columns_checks = TokenStream::new();

        Ok(quote! {
            #enum_impl

            #[common_traits::prelude::basic]
            #[derive(Default)]
            pub struct #builder_ident {
                #(#attributes)*
            }

            impl #builder_ident {
                #(#setter_methods)*
            }

            impl web_common_traits::prelude::InsertableVariantBuilder for #builder_ident {
                type New = #session_insert_variant_name;
            }

            impl common_traits::prelude::Builder for #builder_ident {
                type Object = #session_insert_variant_name;
                type Error = common_traits::prelude::BuilderError<#enum_name>;
                type Attribute = #enum_name;

                fn build(self) -> Result<Self::Object, Self::Error> {
                    #(#completeness_check)*

                    #inter_columns_checks

                    Ok(Self::Object {
                        #(#object_attributes)*
                    })
                }
            }
        })
    }

    /// Returns the Syn `TokenStream` for the implementation of the
    /// `InsertableVariant` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the implementation of the
    /// `InsertableVariant` trait.
    ///
    /// # Errors
    ///
    /// * If the struct name cannot be generated.
    pub fn insert_trait_impls(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let insertable_row_impl = self.insertable_row_impl()?;
        let insertable_session = self.insertable_session_variant_impl(conn)?;
        let builder_impl = self.insertable_variant_builder_impl(conn)?;

        Ok(quote! {
            #insertable_row_impl
            #insertable_session
            #builder_impl
        })
    }
}
