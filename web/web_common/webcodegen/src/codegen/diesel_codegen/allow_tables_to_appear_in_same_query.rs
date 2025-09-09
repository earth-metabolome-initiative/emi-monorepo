//! Submodule implementing code relative to diesel's [`allow_tables_to_appear_in_same_query`](https://docs.rs/diesel/latest/diesel/macro.allow_tables_to_appear_in_same_query.html) macro.

use std::{collections::HashSet, path::Path};

use diesel::PgConnection;
use proc_macro2::TokenStream;

use super::Codegen;
use crate::{Table, traits::TableLike};

impl Codegen<'_> {
    /// Generate implementations of the `allow_tables_to_appear_in_same_query`
    /// diesel macro.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) fn generate_allow_tables_to_appear_in_same_query(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        // https://github.com/earth-metabolome-initiative/emi-monorepo/issues/37

        // We create the path where we are going to output the modules
        // with all its submodules.
        std::fs::create_dir_all(root)?;

        // create token stream for importing the
        // diffent submodules
        // For example, this toekn stream will output the following:
        // mod table1;
        // mod table2;
        //
        // And then we will create the files table1.rs and table2.rs
        // and in there we generate another tokenstream with the
        // allow_tables_to_appear_in_same_query!() macro with the correct tables
        let mut allow_table_query_module = TokenStream::new();

        let mut table_hashset: HashSet<(&Table, &Table)> = HashSet::new();

        for table in tables {
            let mut submodule_token_stream = TokenStream::new();
            let table_name = table.snake_case_ident()?;
            for foreign_table in table
                .foreign_tables(conn)?
                .iter()
                .map(|table| table.as_ref())
                .chain(table.ancestral_extension_tables(conn)?.iter())
            {
                // if the foreign table is the same as table we continue
                if foreign_table == table {
                    continue;
                }

                // even if the tuple direction is reversed.
                if table_hashset.contains(&(table, &foreign_table))
                    || table_hashset.contains(&(&foreign_table, table))
                {
                    continue;
                }

                let Some(foreign_table_ref) = tables.iter().find(|&t| t == foreign_table) else {
                    continue;
                };
                table_hashset.insert((table, foreign_table_ref));

                // with the correct tables
                let foreign_table_name = foreign_table.snake_case_ident()?;
                let foreign_table_path = foreign_table.import_diesel_path()?;
                submodule_token_stream.extend(quote::quote! {
                    use #foreign_table_path;
                    diesel::allow_tables_to_appear_in_same_query!(
                        #table_name,
                        #foreign_table_name
                    );
                });
            }

            // If the user has specified a `team_members` table, we additional
            // generate the join with the `team_projects` table. These tables
            // are siblings, but if we were to
            if let (Some(team_members), Some(team_projects)) =
                (self.team_members_table, self.team_projects_table)
            {
                if table == team_members {
                    let foreign_table_path = team_projects.import_diesel_path()?;
                    submodule_token_stream.extend(quote::quote! {
                        use #foreign_table_path;
                        diesel::allow_tables_to_appear_in_same_query!(
                            #table_name,
                            team_projects
                        );
                    });
                }
            }

            // it out to the expected file.
            if submodule_token_stream.is_empty() {
                continue;
            }
            let table_path = table.import_diesel_path()?;
            let table_name = table.snake_case_ident()?;
            let table_file = root.join(format!("{table_name}.rs"));
            std::fs::write(
                &table_file,
                self.beautify_code(&quote::quote! {
                    use #table_path;
                    #submodule_token_stream
                }),
            )?;

            // main token stream
            allow_table_query_module.extend(quote::quote! {
                mod #table_name;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&allow_table_query_module))?;
        Ok(())
    }
}
