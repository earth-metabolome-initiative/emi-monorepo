//! Submodule providing the code to generate the implementation of the
//! [`InsertableVariant`] trait for all required tables.

use std::{collections::HashMap, path::Path};

use diesel::PgConnection;
use proc_macro2::TokenStream;

use crate::{Codegen, Column, KeyColumnUsage, Table};

impl Codegen<'_> {
    #[allow(clippy::too_many_lines)]
    /// Generates the [`InsertableVariant`] trait implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) fn generate_insertable_variant_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut ifvb_main_module = TokenStream::new();
        let Some(user_table) = self.users_table else {
            return Err(crate::errors::CodeGenerationError::UserTableNotProvided.into());
        };
        let user_id_type = user_table.primary_key_type(conn)?;

        for table in tables {
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_path = table.import_struct_path()?;
            let table_ident = table.snake_case_ident()?;
            let insertable_struct = table.insertable_variant_ty()?;
            let insertable_builder = table.insertable_builder_ty()?;
            let extension_table = table.extension_table(conn)?;

            // We build a check to see whether the user is authorized to update
            // the parent tables.

            let mut parent_check = TokenStream::new();
            let mut parent_check_trait_requirements = HashMap::new();

            for parent_key in table.parent_keys(conn)? {
                let parent_key_method = parent_key.getter_ident()?;
                let mut parent_tables = Vec::new();

                for foreign_key in parent_key.foreign_keys(conn)? {
                    if foreign_key.columns(conn)?.len() > 1 {
                        continue;
                    }
                    parent_tables
                        .push(foreign_key.foreign_table(conn)?.expect("Parent table not found"));
                }

                if parent_tables.len() > 1 {
                    unimplemented!("This is a new case!")
                }

                let parent_table = parent_tables.pop().expect("Parent table not found");
                if !parent_table.allows_updatable(conn)? {
                    continue;
                }
                let parent_table_path = parent_table.import_struct_path()?;

                parent_check.extend(if parent_key.is_nullable() {
                        quote::quote! {
                            if let Some(parent) = insertable_struct.#parent_key_method(conn)? {
                                if !parent.can_update(user_id, conn)? {
                                    return Err(generic_backend_request_errors::GenericBackendRequestError::Unauthorized.into());
                                }
                            }
                        }
                    } else {
                        quote::quote! {
                            if !insertable_struct.#parent_key_method(conn)?.can_update(user_id, conn)? {
                                return Err(generic_backend_request_errors::GenericBackendRequestError::Unauthorized.into());
                            }
                        }
                    });

                let current_constraints = parent_check_trait_requirements
                    .entry(parent_table.clone())
                    .or_insert_with(|| TokenStream::new());

                if current_constraints.is_empty() {
                    current_constraints.extend(
                    quote::quote! {
                        #parent_table_path: diesel::Identifiable + web_common_traits::database::Updatable<C, UserId = #user_id_type>,
                        <#parent_table_path as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
                            <#parent_table_path as diesel::Identifiable>::Id,
                        >,
                        <<#parent_table_path as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
                            <#parent_table_path as diesel::Identifiable>::Id,
                        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
                        <<<#parent_table_path as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
                            <#parent_table_path as diesel::Identifiable>::Id,
                        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
                            'a,
                            C,
                            #parent_table_path,
                        >
                    });
                }
            }

            let additional_imports = if parent_check.is_empty() {
                TokenStream::new()
            } else {
                quote::quote! {
                    use web_common_traits::database::Updatable;
                }
            };

            let user_id = if parent_check.is_empty() && extension_table.is_none() {
                quote::quote! { _user_id }
            } else {
                quote::quote! { user_id }
            };

            let mut additional_where_clause = if parent_check_trait_requirements.is_empty() {
                TokenStream::new()
            } else {
                let constraints =
                    parent_check_trait_requirements.values().cloned().collect::<Vec<_>>();

                quote::quote! { #(#constraints),*, }
            };

            let attributes_enum = table.insertable_enum_ty()?;

            let maybe_insert_recursion = if let Some(extension_table) = &extension_table {
                let extension_insertable_builder = extension_table.insertable_builder_ty()?;
                let extension_row = extension_table.import_struct_path()?;
                let extension_attributes_enum = extension_table.insertable_enum_ty()?;

                // We need to add to the where constraints that the extension insertable builder
                // actually implements the `InsertableVariant` trait.
                additional_where_clause.extend(quote::quote! {
                    #extension_insertable_builder: web_common_traits::database::InsertableVariant<
                        C,
                        UserId = #user_id_type,
                        Row = #extension_row,
                        Error = web_common_traits::database::InsertError<#extension_attributes_enum>,
                    >,
                });

                let mut partial_builder_columns: HashMap<Column, KeyColumnUsage> = HashMap::new();
                for column in table.columns(conn)? {
                    if let Some(foreign_key) = column.requires_partial_builder(conn)? {
                        partial_builder_columns.insert(column, foreign_key);
                    }
                }

                let mut covered_implementations = vec![extension_table.table_name.clone()];

                for foreign_key in partial_builder_columns.values() {
                    let foreign_table =
                        foreign_key.foreign_table(conn)?.expect("Foreign table not found");

                    if covered_implementations.contains(&foreign_table.table_name) {
                        continue;
                    }

                    covered_implementations.push(foreign_table.table_name.clone());
                    let foreign_table_path = foreign_table.import_struct_path()?;
                    let attributes_enum = foreign_table.insertable_enum_ty()?;
                    let builder = foreign_table.insertable_builder_ty()?;

                    additional_where_clause.extend(quote::quote! {
                        #builder: web_common_traits::database::InsertableVariant<
                            C,
                            UserId = #user_id_type,
                            Row = #foreign_table_path,
                            Error = web_common_traits::database::InsertError<#attributes_enum>,
                        >,
                    });
                }

                quote::quote! {
                    let insertable_struct: #insertable_struct = self.try_insert(#user_id, conn)?;
                }
            } else {
                quote::quote! {
                    let insertable_struct: #insertable_struct = self.try_into()?;
                }
            };

            std::fs::write(&table_file, self.beautify_code(&quote::quote!{
				impl<C: diesel::connection::LoadConnection> web_common_traits::database::InsertableVariant<C> for #insertable_builder
                where
                    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
                    diesel::query_builder::InsertStatement<
                        <#table_path as diesel::associations::HasTable>::Table,
                        <#insertable_struct as diesel::Insertable<<#table_path as diesel::associations::HasTable>::Table>>::Values,
                    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, #table_path>,
                    #additional_where_clause
                {
					type Row = #table_path;
                    type InsertableVariant = #insertable_struct;
                    type Error = web_common_traits::database::InsertError<#attributes_enum>;
                    type UserId = #user_id_type;

                    fn insert(
                        self,
                        #user_id: Self::UserId,
                        conn: &mut C,
                    ) -> Result<Self::Row, Self::Error> {
                        use diesel::RunQueryDsl;
                        use diesel::associations::HasTable;
                        #additional_imports

                        #maybe_insert_recursion

                        #parent_check

                        Ok(diesel::insert_into(Self::Row::table())
                            .values(insertable_struct)
                            .get_result(conn)?)
                    }
				}
			})?)?;

            ifvb_main_module.extend(quote::quote! {
                mod #table_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&ifvb_main_module)?)?;

        Ok(())
    }
}
