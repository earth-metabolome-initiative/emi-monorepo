use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper, TextExpressionMethods,
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use snake_case_sanitizer::Sanitizer as SnakeCaseSanizer;
use syn::{parse_str, File, Ident, Type};

use crate::errors::WebCodeGenError;
use crate::CheckConstraint;
use crate::Column;
use crate::Index;
use crate::TableConstraint;

/// Reserved Rust words that cannot be used as identifiers.
pub const RESERVED_RUST_WORDS: [&str; 49] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", // Future reserved keywords
    "abstract", "async", "await", "become", "box", "do", "final", "macro", "override", "priv",
    "try", "typeof", "unsized", "virtual", "yield",
];

/// Diesel collisions that need to be handled.
pub const RESERVED_DIESEL_WORDS: [&str; 1] = ["columns"];

/// Struct defining the `information_schema.tables` table.
#[derive(Queryable, QueryableByName, PartialEq, Eq, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::tables)]
pub struct Table {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_type: String,
    pub self_referencing_column_name: Option<String>,
    pub reference_generation: Option<String>,
    pub user_defined_type_catalog: Option<String>,
    pub user_defined_type_schema: Option<String>,
    pub user_defined_type_name: Option<String>,
    pub is_insertable_into: String,
    pub is_typed: String,
    pub commit_action: Option<String>,
}

impl Table {
    /// Returns the correct diesel feature flag for the number of columns in the table.
    pub fn diesel_feature_flag_name(
        &self,
        conn: &mut PgConnection,
    ) -> Result<&str, WebCodeGenError> {
        let number_of_columns = self.columns(conn)?.len();
        if number_of_columns > 128 {
            Err(WebCodeGenError::ExcessiveNumberOfColumns(
                self.clone(),
                number_of_columns,
            ))
        } else if number_of_columns > 64 {
            Ok("128-column-tables")
        } else if number_of_columns > 32 {
            Ok("64-column-tables")
        } else if number_of_columns > 16 {
            Ok("32-column-tables")
        } else {
            Ok("diesel")
        }
    }

    /// Returns the correct diesel feature flag for the number of columns in the table.
    pub fn diesel_column_feature_flag(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let flag_name = self.diesel_feature_flag_name(conn)?;
        Ok(quote! {#[cfg(feature = #flag_name)]})
    }

    /// Returns whether the table is a view.
    pub fn is_view(&self) -> bool {
        self.table_type == "VIEW"
    }

    /// Returns whether the table is temporary.
    pub fn is_temporary(&self) -> bool {
        self.table_type == "LOCAL TEMPORARY" || self.table_type == "GLOBAL TEMPORARY"
    }

    /// Returns the name of the struct converted from the table name.
    pub fn struct_name(&self) -> Result<String, WebCodeGenError> {
        let sanitizer = SnakeCaseSanizer::new()
            .include_defaults()
            .remove_leading_underscores()
            .remove_trailing_underscores();
        Ok(sanitizer.to_camel_case(&self.table_name)?)
    }

    /// Returns the sanitized snake case name of the table.
    pub fn snake_case_name(&self) -> Result<String, WebCodeGenError> {
        let sanitizer = SnakeCaseSanizer::new()
            .include_defaults()
            .remove_leading_underscores()
            .remove_trailing_underscores();
        Ok(sanitizer.to_snake_case(&self.table_name)?)
    }

    /// Returns whether the table has a sanitized snake case name.
    pub fn has_snake_case_name(&self) -> Result<bool, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        Ok(snake_case_name != self.table_name)
    }

    /// Returns the sanitized snake case syn Ident of the table.
    pub fn snake_case_ident(&self) -> Result<Ident, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        if RESERVED_RUST_WORDS.contains(&snake_case_name.as_str()) {
            Ok(Ident::new_raw(
                &snake_case_name,
                proc_macro2::Span::call_site(),
            ))
        } else {
            Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns the singular name of the table.
    pub fn singular_table_name(&self) -> String {
        // TODO: make singular
        self.table_name.clone()
    }

    /// Shared traits for all tables.
    pub fn shared_traits() -> TokenStream {
        quote! {
            pub trait Connection {
                type Error: std::error::Error;
            }

            #[cfg(feature = "diesel")]
            impl Connection for diesel_async::AsyncPgConnection {
                type Error = diesel::result::Error;
            }
        }
    }

    /// Defines the insertion-related traits.
    pub fn insert_variant_trait() -> TokenStream {
        quote! {
            #[cfg(feature = "diesel")]
            pub trait IntoSessionInsertVariant {
                type SessionInsertVariant: InsertableVariant<diesel_async::AsyncPgConnection>;

                fn into_session_insert_variant(self, created_by: i32) -> Self::SessionInsertVariant;
            }

            pub trait InsertableVariant<C: Connection> {
                type Row;

                fn insert(&self, conn: &mut C) -> impl std::future::Future<Output = Result<Self::Row, C::Error>> + Send;
            }
        }
    }

    /// Defines the update-related traits.
    pub fn update_variant_trait() -> TokenStream {
        quote! {
            #[cfg(feature = "diesel")]
            pub trait IntoSessionUpdateVariant {
                type SessionUpdateVariant: UpdateableVariant<diesel_async::AsyncPgConnection>;

                fn into_session_update_variant(self, updated_by: i32) -> Self::SessionUpdateVariant;
            }

            pub trait UpdateableVariant<C: Connection> {
                type Row;

                fn update(&self, conn: &mut C) -> impl std::future::Future<Output = Result<Self::Row, C::Error>> + Send;
            }
        }
    }

    /// Defines the HasFilterVariant trait.
    pub fn filter_variant_trait() -> TokenStream {
        quote! {
            pub trait HasFilterVariant {
                type FilterVariant;
            }
        }
    }

    /// Returns all relevant traits for the table.
    pub fn traits() -> TokenStream {
        let shared_traits = Self::shared_traits();
        let insert_variant_trait = Self::insert_variant_trait();
        let update_variant_trait = Self::update_variant_trait();
        let filter_variant_trait = Self::filter_variant_trait();
        quote! {
            #shared_traits
            #insert_variant_trait
            #update_variant_trait
            #filter_variant_trait
        }
    }

    /// Returns the Syn TokenStream for the diesel schema definition for the table.
    pub fn to_schema(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        let table_schema = Ident::new(&self.table_schema, proc_macro2::Span::call_site());
        let original_table_name = &self.table_name;
        let sanitized_table_name_ident =
            Ident::new(&self.snake_case_name()?, proc_macro2::Span::call_site());
        let columns = self
            .columns(conn)?
            .into_iter()
            .map(|column| {
                let original_column_name = &column.column_name;
                let column_attribute: Ident = column.sanitized_snake_case_ident()?;
                let column_type = column.diesel_type(conn)?;
                Ok(if original_column_name != &column_attribute.to_string() {
                    quote! {
                        #[sql_name = #original_column_name]
                        #column_attribute -> #column_type
                    }
                } else {
                    quote! {
                        #column_attribute -> #column_type
                    }
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
        let primary_key_names = self
            .primary_key_columns(conn)?
            .into_iter()
            .map(|column| Ident::new(&column.column_name, proc_macro2::Span::call_site()))
            .collect::<Vec<Ident>>();

        let primary_key_names = if primary_key_names.is_empty() {
            TokenStream::new()
        } else {
            quote! {
                (#(#primary_key_names),*)
            }
        };

        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        Ok(if self.has_snake_case_name()? {
            quote! {
                #[cfg(feature = #columns_feature_flag_name)]
                diesel::table! {
                    #[sql_name = #original_table_name]
                    #table_schema.#sanitized_table_name_ident #primary_key_names {
                        #(#columns),*
                    }
                }
            }
        } else {
            quote! {
                #[cfg(feature = #columns_feature_flag_name)]
                diesel::table! {
                    #table_schema.#sanitized_table_name_ident #primary_key_names {
                        #(#columns),*
                    }
                }
            }
        })
    }

    /// Returns wether a table require authorizations to be viewed
    pub fn require_view_authorizations(&self, _conn: &mut PgConnection) -> bool {
        false
    }

    /// Returns wether a table require authorizations to be modified
    pub fn require_modify_authorizations(&self, _conn: &mut PgConnection) -> bool {
        false
    }

    /// Returns the primary key identifiers for the table.
    pub fn primary_key_identifiers(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Ident>, WebCodeGenError> {
        self.primary_key_columns(conn)?
            .into_iter()
            .map(|column| column.sanitized_snake_case_ident())
            .collect::<Result<Vec<Ident>, WebCodeGenError>>()
    }

    /// Returns the primary key decorator to be used for this table, if any.
    pub fn primary_key_decorator(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        // In some cases, the table will not have a primary key. In which case, we cannot specify the primary key
        // decorator on the struct.
        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;
        Ok(if self.has_primary_keys(conn)? {
            let primary_key_identifiers = self.primary_key_identifiers(conn)?;
            quote! {
                #[cfg_attr(feature = #columns_feature_flag_name, diesel(primary_key(#(#primary_key_identifiers),*)))]
            }
        } else {
            TokenStream::new()
        })
    }

    /// Returns the diesel derives supported by the table.
    pub fn diesel_derives(&self, conn: &mut PgConnection) -> Result<Vec<Type>, WebCodeGenError> {
        let mut derives = Vec::new();

        derives.push(parse_str("diesel::Selectable").unwrap());

        if self.has_primary_keys(conn)? {
            derives.push(parse_str("diesel::Queryable").unwrap());
            derives.push(parse_str("diesel::Identifiable").unwrap());
        }

        Ok(derives)
    }

    /// Returns the diesel derives decorator for the table.
    pub fn diesel_derives_decorator(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let diesel_derives = self.diesel_derives(conn)?;
        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;
        Ok(if diesel_derives.is_empty() {
            TokenStream::new()
        } else {
            quote! {
                #[cfg_attr(feature = #columns_feature_flag_name, derive(#(#diesel_derives),*))]
            }
        })
    }

    /// Returns the Syn TokenStream for the struct definition and associated methods.
    pub fn to_syn(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        if self.columns(conn)?.len() > 128 {
            return Err(WebCodeGenError::ExcessiveNumberOfColumns(
                self.clone(),
                self.columns(conn)?.len(),
            ));
        }

        let sanitized_table_name = self.snake_case_ident()?;
        let struct_name: Ident = Ident::new(&self.struct_name()?, proc_macro2::Span::call_site());
        let attributes = self
            .columns(conn)?
            .into_iter()
            .map(|column| {
                let column_attribute: Ident = column.sanitized_snake_case_ident()?;
                let column_type = column.data_type(conn)?;
                Ok(quote! {
                    pub #column_attribute: #column_type
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let mutability_impls = if self.has_session_user_generated_columns(conn) {
            let insert_trait_impls = self.insert_trait_impls(conn)?;
            let update_trait_impls = if self.has_primary_keys(conn)? {
                self.update_trait_impls(conn)?
            } else {
                TokenStream::new()
            };
            quote! {
                #insert_trait_impls
                #update_trait_impls
            }
        } else {
            TokenStream::new()
        };
        let foreign_key_methods = self.foreign_key_methods(conn)?;
        let delete_method = if self.has_primary_keys(conn)? {
            self.delete_method(conn)?
        } else {
            TokenStream::new()
        };
        let primary_key_decorator = self.primary_key_decorator(conn)?;
        let diesel_derives_decorator = self.diesel_derives_decorator(conn)?;
        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        let built_table_syn = quote! {
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #diesel_derives_decorator
            #primary_key_decorator
            #[cfg_attr(feature = #columns_feature_flag_name, diesel(table_name = #sanitized_table_name))]
            pub struct #struct_name {
                #(#attributes),*
            }

            impl #struct_name {
                #(#foreign_key_methods)*
                #delete_method
            }

            #mutability_impls
        };

        // Convert the generated TokenStream to a string
        let code_string = built_table_syn.to_string();

        // Parse the generated code string into a syn::Item
        if let Err(error) = syn::parse_str::<File>(&code_string) {
            return Err(WebCodeGenError::IllegalTableCodegen(
                format!("Error building table struct: {}", error),
                code_string,
                self.clone(),
            ));
        }

        Ok(built_table_syn)
    }

    fn foreign_key_methods<'a>(
        &'a self,
        conn: &'a mut PgConnection,
    ) -> Result<Vec<TokenStream>, WebCodeGenError> {
        self
            .columns(conn)?
            .into_iter()
            .filter(|column| column.is_foreign_key(conn))
            .collect::<Vec<Column>>()
            .into_iter()
            .map(|column| {
                let (foreign_key_table, foreign_key_column) = column.foreign_table(conn).unwrap().unwrap();
                let foreign_key_table_name = Ident::new(&foreign_key_table.snake_case_name()?, proc_macro2::Span::call_site());
                let foreign_key_column_name: Ident = Ident::new(&foreign_key_column.column_name, proc_macro2::Span::call_site());
                let method_name: Ident = if column.column_name.ends_with("_id") {
                    Ident::new(&column.column_name[..column.column_name.len() - 3], proc_macro2::Span::call_site())
                } else {
                    Ident::new(&column.column_name, proc_macro2::Span::call_site())
                };
                let current_column_ident: Ident = column.sanitized_snake_case_ident()?;
                let foreign_key_struct_name: Ident = Ident::new(&foreign_key_table.struct_name()?, proc_macro2::Span::call_site());

                // If the current column has a Nullable (Option) type, the return type of the method should be an Option
                let return_type_ident = if column.is_nullable() {
                    quote! { Option<#foreign_key_struct_name> }
                } else {
                    quote! { #foreign_key_struct_name }
                };

                // Analogously, we check before executing the query whether the current column is None. If so,
                // we return None as well.
                let column_value_retrieval = if column.is_nullable() {
                    quote! {
                        let #current_column_ident = if let Some(#current_column_ident) = self.#current_column_ident.as_ref() {
                            #current_column_ident
                        } else {
                            return Ok(None);
                        };
                    }
                } else {
                    TokenStream::new()
                };

                // It follows that we need to determine whether the right term of the equality for
                // the filter should be prefixed with 'self.' or not (if the column is nullable).
                let filter_statement = if column.is_nullable() {
                    quote! { #foreign_key_table_name::#foreign_key_column_name.eq(&#current_column_ident) }
                } else {
                    quote! { #foreign_key_table_name::#foreign_key_column_name.eq(&self.#current_column_ident) }
                };

                // Finally, when we are returning a Result<Option<TableStructType>, ...>, 
                // we need to wrap the result of the query in a Some.
                let map_ops = if column.is_nullable() {
                    quote! { .map(Some) }
                } else {
                    TokenStream::new()
                };

                let stricter_flag_name = if self.columns(conn)?.len() > foreign_key_table.columns(conn)?.len() {
                    self.diesel_feature_flag_name(conn)?
                } else {
                    foreign_key_table.diesel_feature_flag_name(conn)?
                };

                Ok(quote! {
                    #[cfg(feature = #stricter_flag_name)]
                    pub async fn #method_name(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<#return_type_ident, diesel::result::Error> {
                        #column_value_retrieval
                        #foreign_key_table_name::table
                            .filter(#filter_statement)
                            .select(<#foreign_key_struct_name as diesel::SelectableHelper<diesel::pg::Pg>>::as_select())
                            .first::<#foreign_key_struct_name>(conn)
                            .await
                            #map_ops
                    }
                })
            }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()
    }

    pub fn delete_method(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        let sanitized_table_name =
            Ident::new(&self.snake_case_name()?, proc_macro2::Span::call_site());
        let primary_key_columns = self.primary_key_columns(conn).unwrap();

        let where_clause = primary_key_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                quote! {
                    #sanitized_table_name::#column_name.eq(&self.#column_name)
                }
            })
            .collect::<Vec<_>>();

        // Join the where clauses with an and
        let where_clause = where_clause
            .into_iter()
            .reduce(|a, b| quote! { diesel::BoolExpressionMethods::and(#a, #b) })
            .unwrap();

        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        Ok(quote! {
            #[cfg(feature = #columns_feature_flag_name)]
            pub async fn delete(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<usize, diesel::result::Error> {
                diesel::delete(#sanitized_table_name::table.filter(#where_clause)).execute(conn).await
            }
        })
    }

    pub fn has_session_user_generated_columns(&self, conn: &mut PgConnection) -> bool {
        self.columns(conn)
            .unwrap()
            .iter()
            .any(|column| column.is_session_user_generated(conn))
    }

    pub fn insert_trait_impls(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name: Ident = Ident::new(&self.struct_name()?, proc_macro2::Span::call_site());
        let table_name = self.snake_case_ident()?;
        let columns = self.columns(conn)?;

        let session_insert_columns = columns
            .iter()
            .filter(|column| !column.is_automatically_generated())
            .collect::<Vec<&Column>>();

        let insert_columns = session_insert_columns
            .iter()
            .filter(|column| !column.is_session_user_generated(conn))
            .collect::<Vec<&&Column>>();

        let session_insert_variant_name = Ident::new(
            &format!("SessionInsert{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let session_insert_variable_attributes = session_insert_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let column_type = column.data_type(conn)?;
                Ok(quote! {
                    pub #column_name: #column_type,
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let insert_variant_name = Ident::new(
            &format!("Insert{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let insertable_variant_attributes = insert_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let column_type = column.data_type(conn)?;
                Ok(quote! {
                    pub #column_name: #column_type,
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let into_session_insert_variant_map = insert_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            quote! {
                #column_name: self.#column_name,
            }
        });

        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        let new_structs_implementation = quote! {
            #[cfg(feature = #columns_feature_flag_name)]
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = #columns_feature_flag_name, derive(diesel::Insertable))]
            #[cfg_attr(feature = #columns_feature_flag_name, diesel(table_name = #table_name))]
            pub struct #session_insert_variant_name {
                #(#session_insert_variable_attributes)*
            }

            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #insert_variant_name {
                #(#insertable_variant_attributes)*
            }
        };

        Ok(quote! {
            #new_structs_implementation

            #[cfg(feature = #columns_feature_flag_name)]
            impl IntoSessionInsertVariant for #insert_variant_name {
                type SessionInsertVariant = #session_insert_variant_name;

                fn into_session_insert_variant(self, created_by: i32) -> Self::SessionInsertVariant {
                    #session_insert_variant_name {
                        #(#into_session_insert_variant_map)*
                        created_by,
                        updated_by: created_by
                    }
                }
            }

            #[cfg(feature = #columns_feature_flag_name)]
            impl InsertableVariant<diesel_async::AsyncPgConnection> for #session_insert_variant_name {
                type Row = #struct_name;

                async fn insert(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<Self::Row, diesel::result::Error> {
                    diesel::insert_into(#table_name::table)
                        .values(self)
                        .get_result(conn)
                        .await
                }
            }
        })
    }

    pub fn update_trait_impls(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name: Ident = Ident::new(&self.struct_name()?, proc_macro2::Span::call_site());
        let table_name = self.snake_case_ident()?;
        let columns = self.columns(conn)?;
        let primary_key_names = self
            .primary_key_columns(conn)?
            .into_iter()
            .map(|column| column.column_name.clone())
            .collect::<Vec<String>>();

        let session_update_columns = columns
            .iter()
            .filter(|column| {
                (!column.is_automatically_generated()
                    || primary_key_names.contains(&column.column_name))
                    && !column.is_created_by(conn)
            })
            .collect::<Vec<&Column>>();

        let update_columns = session_update_columns
            .iter()
            .filter(|column| {
                !column.is_session_user_generated(conn)
                    || primary_key_names.contains(&column.column_name)
            })
            .collect::<Vec<&&Column>>();

        let session_update_variant_name = Ident::new(
            &format!("SessionUpdate{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let session_update_variable_attributes = session_update_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let column_type = column.data_type(conn)?;
                Ok(
                    if column.is_updated_by(conn) || primary_key_names.contains(&column.column_name)
                    {
                        quote! {
                            pub #column_name: #column_type,
                        }
                    } else {
                        quote! {
                            pub #column_name: Option<#column_type>,
                        }
                    },
                )
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let update_variant_name = Ident::new(
            &format!("Update{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let updateable_variant_attributes = update_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let column_type = column.data_type(conn)?;

                Ok(if primary_key_names.contains(&column.column_name) {
                    quote! {
                        pub #column_name: #column_type,
                    }
                } else {
                    quote! {
                        pub #column_name: Option<#column_type>,
                    }
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let into_session_update_variant_map = update_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            quote! {
                #column_name: self.#column_name,
            }
        });

        // In some cases, the table will not have a primary key. In which case, we cannot specify the primary key
        // decorator on the struct.
        let primary_key_decorator = self.primary_key_decorator(conn)?;
        let diesel_derives_decorator = self.diesel_derives_decorator(conn)?;
        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        let new_structs_implementation = quote! {
            #[cfg(feature = #columns_feature_flag_name)]
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = #columns_feature_flag_name, derive(diesel::AsChangeset))]
            #diesel_derives_decorator
            #primary_key_decorator
            #[cfg_attr(feature = #columns_feature_flag_name, diesel(table_name = #table_name))]
            pub struct #session_update_variant_name {
                #(#session_update_variable_attributes)*
            }

            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #update_variant_name {
                #(#updateable_variant_attributes)*
            }
        };

        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        Ok(quote! {
            #new_structs_implementation

            #[cfg(feature = #columns_feature_flag_name)]
            impl IntoSessionUpdateVariant for #update_variant_name {
                type SessionUpdateVariant = #session_update_variant_name;

                fn into_session_update_variant(self, updated_by: i32) -> Self::SessionUpdateVariant {
                    #session_update_variant_name {
                        #(#into_session_update_variant_map)*
                        updated_by
                    }
                }
            }

            #[cfg(feature = #columns_feature_flag_name)]
            impl UpdateableVariant<diesel_async::AsyncPgConnection> for #session_update_variant_name {
                type Row = #struct_name;

                async fn update(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<Self::Row, diesel::result::Error> {
                    diesel::update(#table_name::table)
                        .filter(#table_name::id.eq(self.id))
                        .set(self)
                        .get_result(conn)
                        .await
                }
            }
        })
    }

    pub fn unique_indexes(&self, conn: &mut PgConnection) -> Result<Vec<Index>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::table
            .filter(pg_indexes::schemaname.eq(&self.table_schema))
            .filter(pg_indexes::tablename.eq(&self.table_name))
            .filter(pg_indexes::indexdef.like("%UNIQUE%"))
            .load::<Index>(conn)
    }

    pub fn gin_indexes(&self, conn: &mut PgConnection) -> Result<Vec<Index>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::table
            .filter(pg_indexes::schemaname.eq(&self.table_schema))
            .filter(pg_indexes::tablename.eq(&self.table_name))
            .filter(pg_indexes::indexdef.like("%USING gin%"))
            .load::<Index>(conn)
    }

    pub fn gist_indexes(&self, conn: &mut PgConnection) -> Result<Vec<Index>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::table
            .filter(pg_indexes::schemaname.eq(&self.table_schema))
            .filter(pg_indexes::tablename.eq(&self.table_name))
            .filter(pg_indexes::indexdef.like("%USING gist%"))
            .load::<Index>(conn)
    }

    pub fn load_all(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::tables;
        tables::table
            .filter(tables::table_catalog.eq(table_catalog))
            .filter(tables::table_schema.eq(table_schema.unwrap_or("public")))
            .filter(tables::table_name.ne("__diesel_schema_migrations"))
            .load::<Table>(conn)
    }

    pub fn load(
        conn: &mut PgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Option<Self> {
        use crate::schema::tables;
        let table_schema = table_schema.unwrap_or("public");
        tables::table
            .filter(tables::table_name.eq(table_name))
            .filter(tables::table_schema.eq(table_schema))
            .filter(tables::table_catalog.eq(table_catalog))
            .first::<Table>(conn)
            .ok()
    }

    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, WebCodeGenError> {
        use crate::schema::columns;
        Ok(columns::table
            .filter(columns::table_name.eq(&self.table_name))
            .filter(columns::table_schema.eq(&self.table_schema))
            .filter(columns::table_catalog.eq(&self.table_catalog))
            .load::<Column>(conn)?)
    }

    pub fn column_by_name(
        &self,
        conn: &mut PgConnection,
        column_name: &str,
    ) -> Result<Column, DieselError> {
        use crate::schema::columns;
        columns::table
            .filter(columns::table_name.eq(&self.table_name))
            .filter(columns::table_schema.eq(&self.table_schema))
            .filter(columns::table_catalog.eq(&self.table_catalog))
            .filter(columns::column_name.eq(column_name))
            .first::<Column>(conn)
    }

    pub fn unique_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Vec<Column>>, WebCodeGenError> {
        use crate::schema::columns;
        use crate::schema::key_column_usage;
        use crate::schema::table_constraints;
        Ok(key_column_usage::table
            .inner_join(
                columns::table.on(key_column_usage::table_name
                    .nullable()
                    .eq(columns::table_name.nullable())
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(columns::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(columns::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::column_name
                            .nullable()
                            .eq(columns::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::table.on(key_column_usage::constraint_name
                    .nullable()
                    .eq(table_constraints::constraint_name.nullable())
                    .and(
                        key_column_usage::constraint_schema
                            .nullable()
                            .eq(table_constraints::constraint_schema.nullable()),
                    )
                    .and(
                        key_column_usage::constraint_catalog
                            .nullable()
                            .eq(table_constraints::constraint_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::table_name
                            .nullable()
                            .eq(table_constraints::table_name.nullable()),
                    )
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(table_constraints::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(table_constraints::table_catalog.nullable()),
                    )),
            )
            .filter(key_column_usage::table_name.eq(&self.table_name))
            .filter(key_column_usage::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::table_catalog.eq(&self.table_catalog))
            .filter(table_constraints::constraint_type.eq("UNIQUE"))
            .order_by(table_constraints::constraint_name)
            .select((TableConstraint::as_select(), Column::as_select()))
            .load::<(TableConstraint, Column)>(conn)
            .map(|rows| {
                rows.into_iter()
                    .chunk_by(|(constraint, _)| constraint.constraint_name.clone())
                    .into_iter()
                    .map(|(_, group)| {
                        group
                            .into_iter()
                            .map(|(_, column)| column)
                            .collect::<Vec<Column>>()
                    })
                    .collect()
            })?)
    }

    /// Returns whether the table has primary keys.
    pub fn has_primary_keys(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.primary_key_columns(conn)
            .map(|columns| !columns.is_empty())
    }

    /// Returns the columns composing the primary keys.
    pub fn primary_key_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        use crate::schema::columns;
        use crate::schema::key_column_usage;
        use crate::schema::table_constraints;
        Ok(key_column_usage::table
            .inner_join(
                columns::table.on(key_column_usage::table_name
                    .nullable()
                    .eq(columns::table_name.nullable())
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(columns::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(columns::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::column_name
                            .nullable()
                            .eq(columns::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::table.on(key_column_usage::constraint_name
                    .nullable()
                    .eq(table_constraints::constraint_name.nullable())
                    .and(
                        key_column_usage::constraint_schema
                            .nullable()
                            .eq(table_constraints::constraint_schema.nullable()),
                    )
                    .and(
                        key_column_usage::constraint_catalog
                            .nullable()
                            .eq(table_constraints::constraint_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::table_name
                            .nullable()
                            .eq(table_constraints::table_name.nullable()),
                    )
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(table_constraints::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(table_constraints::table_catalog.nullable()),
                    )),
            )
            .filter(key_column_usage::table_name.eq(&self.table_name))
            .filter(key_column_usage::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::table_catalog.eq(&self.table_catalog))
            .filter(table_constraints::constraint_type.eq("PRIMARY KEY"))
            .select(Column::as_select())
            .load::<Column>(conn)?)
    }

    pub fn check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<CheckConstraint>, DieselError> {
        use crate::schema::check_constraints;
        use crate::schema::table_constraints;

        check_constraints::table
            .inner_join(
                table_constraints::table.on(check_constraints::constraint_name
                    .eq(table_constraints::constraint_name)
                    .and(
                        check_constraints::constraint_schema
                            .eq(table_constraints::constraint_schema),
                    )
                    .and(
                        check_constraints::constraint_catalog
                            .eq(table_constraints::constraint_catalog),
                    )),
            )
            .filter(table_constraints::table_name.eq(&self.table_name))
            .filter(table_constraints::table_schema.eq(&self.table_schema))
            .filter(table_constraints::table_catalog.eq(&self.table_catalog))
            .select(CheckConstraint::as_select())
            .load::<CheckConstraint>(conn)
    }
}
