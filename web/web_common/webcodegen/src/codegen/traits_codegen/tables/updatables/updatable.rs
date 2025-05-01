//! Submodule providing the code to generate the implementation of the
//! [`Updatable`](web_common_traits::database::Updatable) trait for all required
//! tables.

use std::path::Path;

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;

use crate::{Codegen, Table, codegen::Syntax};

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
    pub async fn allows_updatable(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<bool, crate::errors::WebCodeGenError> {
        if self.has_updated_by_column(conn).await? && self.has_created_by_column(conn).await? {
            return Ok(true);
        }
        if !self.has_parents(conn).await? {
            return Ok(false);
        }
        for parent in self.parent_tables(conn).await? {
            if !Box::pin(parent.allows_updatable(conn)).await? {
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
    pub(super) async fn generate_updatable_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut updatable_main_module = TokenStream::new();
        let syntax = Syntax::PostgreSQL;
        let syntax_flag = syntax.as_feature_flag();
        let connection_type = syntax.as_connection_type(true);
        let Some(user) = self.users_table else {
            return Err(crate::errors::CodeGenerationError::UserTableNotProvided.into());
        };
        let Some(projects) = self.projects_table else {
            return Err(crate::errors::CodeGenerationError::ProjectTableNotProvided.into());
        };
        let Some(team_members) = self.team_members_table else {
            return Err(crate::errors::CodeGenerationError::TeamMembersTableNotProvided.into());
        };
        let Some(team_projects) = self.team_projects_table else {
            return Err(crate::errors::CodeGenerationError::TeamProjectsTableNotProvided.into());
        };
        let user_id_type = user.primary_key_type(conn).await?;
        let team_member_table = team_members.import_diesel_path()?;
        let team_project_table = team_projects.import_diesel_path()?;

        for table in tables {
            if !table.allows_updatable(conn).await? && table != user {
                continue;
            }

            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_path = table.import_struct_path()?;
            let table_ident = table.snake_case_ident()?;

            let team_check = if table == projects {
                // In order to check whether the user is a member of the project's team,
                // we need to join the team_projects table with the team_members table
                // and filter by the user's id.
                quote::quote! {
                    if #team_member_table::table
                        .inner_join(#team_project_table::table.on(
                            #team_member_table::team_id.eq(#team_project_table::team_id),
                        ))
                        .filter(
                            #team_member_table::member_id.eq(user_id)
                                .and(#team_project_table::project_id.eq(self.id))
                        )
                        .count()
                        .get_result::<i64>(conn)
                        .await?
                        > 0 {
                            return Ok(true);
                        }
                }
            } else {
                TokenStream::new()
            };

            let mut parent_check = TokenStream::new();

            for parent_key in table.parent_keys(conn).await? {
                let Some((parent_table, _)) = parent_key.foreign_table(conn).await? else {
                    continue;
                };

                if !parent_table.allows_updatable(conn).await? {
                    continue;
                }

                let method_ident = parent_key.getter_ident()?;
                let parent_table_ident = parent_table.snake_case_ident()?;

                let mut recursive_call = if parent_key.is_nullable() {
                    quote::quote! {#parent_table_ident.can_update(user_id, conn)}
                } else {
                    quote::quote! {
                        self.#method_ident(conn).await?.can_update(user_id, conn)
                    }
                };

                if &parent_table == table {
                    recursive_call = quote::quote! {
                        std::boxed::Box::pin(#recursive_call)
                    };
                }

                parent_check.extend(if parent_key.is_nullable() {
                    quote::quote! {
                        if let Some(#parent_table_ident) = self.#method_ident(conn).await? {
                            if !#recursive_call.await? {
                                return Ok(false);
                            }
                        }
                    }
                } else {
                    quote::quote! {
                        if !#recursive_call.await? {
                            return Ok(false);
                        }
                    }
                });
            }

            let mut imports = Vec::new();

            if !team_check.is_empty() {
                imports.push(quote::quote! { use diesel::JoinOnDsl; });
                imports.push(quote::quote! { use diesel::BoolExpressionMethods; });
                imports.push(quote::quote! { use diesel::QueryDsl; });
                imports.push(quote::quote! { use diesel_async::RunQueryDsl; });
                imports.push(quote::quote! { use diesel::ExpressionMethods; });
            }

            let conn_ident = if team_check.is_empty() && parent_check.is_empty() {
                quote::quote! { _conn }
            } else {
                quote::quote! { conn }
            };

            let user_check = if user == table {
                quote::quote! {
                    // If the user is the owner of the record, they can update it
                    if *user_id == self.id {
                        return Ok(true)
                    }
                }
            } else {
                TokenStream::new()
            };

            let created_by_check = if table.has_created_by_column(conn).await? {
                quote::quote! {
                    // If the user is the creator of the record, they can update it
                    if *user_id == self.created_by {
                        return Ok(true)
                    }
                }
            } else {
                TokenStream::new()
            };

            let updated_by_check = if table.has_updated_by_column(conn).await? {
                quote::quote! {
                    // If the user is the last updator of the record, they can update it
                    if *user_id == self.updated_by {
                        return Ok(true)
                    }
                }
            } else {
                TokenStream::new()
            };

            std::fs::write(
                &table_file,
                self.beautify_code(&quote::quote! {
                    #syntax_flag
                    impl web_common_traits::database::Updatable for #table_path {
                        type UserId = #user_id_type;
                        type Conn = #connection_type;

                        async fn can_update(
                            &self,
                            user_id: &Self::UserId,
                            #conn_ident: &mut Self::Conn,
                        ) -> Result<bool, diesel::result::Error> {
                            #(#imports)*

                            #created_by_check
                            #updated_by_check
                            #user_check

                            #team_check

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
