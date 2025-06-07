//! Submodule providing the code to generate the implementation of the
//! [`Updatable`](web_common_traits::database::Updatable) trait for all required
//! tables.

use std::{collections::HashMap, path::Path};

use diesel::PgConnection;
use proc_macro2::TokenStream;

use crate::{Codegen, Table};

impl Table {
    /// Returns whether the table allows for the implementation of the
    /// [`Updatable`](web_common_traits::database::Updatable) trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table allows for the implementation of
    /// the [`Updatable`](web_common_traits::database::Updatable) trait.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    pub fn allows_updatable(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::errors::WebCodeGenError> {
        if self.has_updated_by_column(conn)? && self.has_created_by_column(conn)? {
            return Ok(true);
        }
        if !self.has_parents(conn)? {
            return Ok(false);
        }
        for parent in self.parent_tables(conn)? {
            if !parent.allows_updatable(conn)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl Codegen<'_> {
    #[allow(clippy::too_many_lines)]
    /// Generates the [`Updatable`](web_common_traits::database::Updatable)
    /// trait implementation for the tables
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
    pub(super) fn generate_updatable_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut updatable_main_module = TokenStream::new();
        let Some(user) = self.users_table else {
            return Err(crate::errors::CodeGenerationError::UserTableNotProvided.into());
        };
        // let Some(projects) = self.projects_table else {
        //     return
        // Err(crate::errors::CodeGenerationError::ProjectTableNotProvided.into());
        // };
        // let Some(team_members) = self.team_members_table else {
        //     return
        // Err(crate::errors::CodeGenerationError::TeamMembersTableNotProvided.into());
        // };
        // let Some(team_projects) = self.team_projects_table else {
        //     return
        // Err(crate::errors::CodeGenerationError::TeamProjectsTableNotProvided.into());
        // };
        let user_id_type = user.primary_key_type(conn)?;

        for table in tables {
            if !table.allows_updatable(conn)? && table != user {
                continue;
            }

            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_path = table.import_struct_path()?;
            let table_ident = table.snake_case_ident()?;

            // let team_check = if table == projects {
            //     // In order to check whether the user is a member of the project's team,
            //     // we need to join the team_projects table with the team_members table
            //     // and filter by the user's id.
            //     quote::quote! {
            //         use diesel::{OptionalExtension, SelectableHelper};
            //         use #team_member_path;
            //         use #team_member_table;
            //         use #team_project_table;

            //         if #team_member_ident::table
            //
            // .inner_join(#team_project_ident::table.on(#team_member_ident::team_id.eq(#
            // team_project_ident::team_id)))
            // .filter(#team_member_ident::member_id.eq(user_id).and(#
            // team_project_ident::project_id.eq(self.id)))             .count()
            //             .get_result::<i64>(conn)? > 0 {
            //                 return Ok(true);
            //             }
            //     }
            // } else {
            //     TokenStream::new()
            // };

            let mut parent_check = TokenStream::new();
            let mut parent_check_trait_requirements = HashMap::new();

            for parent_key in table.parent_keys(conn)? {
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

                let method_ident = parent_key.getter_ident()?;
                let parent_table_ident = parent_table.snake_case_ident()?;
                let parent_table_path = parent_table.import_struct_path()?;

                let recursive_call = if parent_key.is_nullable() {
                    quote::quote! {#parent_table_ident.can_update(user_id, conn)}
                } else {
                    quote::quote! {
                        self.#method_ident(conn)?.can_update(user_id, conn)
                    }
                };

                let current_constraints = parent_check_trait_requirements
                    .entry(parent_table_ident.clone())
                    .or_insert_with(TokenStream::new);

                if current_constraints.is_empty() {
                    current_constraints.extend(
                    quote::quote! {
                        #parent_table_path: diesel::Identifiable,
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
                        >,
                    });

                    if &parent_table != table {
                        current_constraints.extend(quote::quote! {
                            #parent_table_path: web_common_traits::database::Updatable<C, UserId = #user_id_type>
                        });
                    }
                }

                parent_check.extend(if parent_key.is_nullable() {
                    quote::quote! {
                        if let Some(#parent_table_ident) = self.#method_ident(conn)? {
                            if !#recursive_call? {
                                return Ok(false);
                            }
                        }
                    }
                } else {
                    quote::quote! {
                        if !#recursive_call? {
                            return Ok(false);
                        }
                    }
                });
            }

            // let mut imports: Vec<TokenStream> = Vec::new();

            // if !team_check.is_empty() {
            //     imports.push(quote::quote! { use diesel::JoinOnDsl; });
            //     imports.push(quote::quote! { use diesel::BoolExpressionMethods; });
            //     imports.push(quote::quote! { use diesel::QueryDsl; });
            //     imports.push(quote::quote! { use diesel::RunQueryDsl; });
            //     imports.push(quote::quote! { use diesel::ExpressionMethods; });
            // }

            let conn_ident = if parent_check.is_empty() {
                quote::quote! { _conn }
            } else {
                quote::quote! { conn }
            };

            let user_check = if user == table {
                quote::quote! {
                    // If the user is the owner of the record, they can update it
                    if user_id == self.id {
                        return Ok(true)
                    }
                }
            } else {
                TokenStream::new()
            };

            let created_by_check = if table.has_created_by_column(conn)? {
                quote::quote! {
                    // If the user is the creator of the record, they can update it
                    if user_id == self.created_by {
                        return Ok(true)
                    }
                }
            } else {
                TokenStream::new()
            };

            let updated_by_check = if table.has_updated_by_column(conn)? {
                quote::quote! {
                    // If the user is the last updator of the record, they can update it
                    if user_id == self.updated_by {
                        return Ok(true)
                    }
                }
            } else {
                TokenStream::new()
            };

            let where_clause = if parent_check_trait_requirements.is_empty() {
                TokenStream::new()
            } else {
                let constraints =
                    parent_check_trait_requirements.values().cloned().collect::<Vec<_>>();

                quote::quote! { where #(#constraints),* }
            };

            std::fs::write(
                &table_file,
                self.beautify_code(&quote::quote! {
                    impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C> for #table_path #where_clause{
                        type UserId = #user_id_type;

                        fn can_update(
                            &self,
                            user_id: Self::UserId,
                            #conn_ident: &mut C,
                        ) -> Result<bool, diesel::result::Error> {
                            #created_by_check
                            #updated_by_check
                            #user_check

                            // If the user cannot update all of the parent records,
                            // they cannot update this record
                            #parent_check

                            // Otherwise, they can update the record
                            Ok(true)
                        }
                    }
                })?,
            )?;

            updatable_main_module.extend(quote::quote! {
                mod #table_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&updatable_main_module)?)?;

        Ok(())
    }
}
