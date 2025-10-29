//! Submodule defining the `SQLWorkspace` trait which allows to generate the
//! `diesel` workspace from a SQL schema, based on `sql_traits`.

use std::path::Path;

use synql_attributes::traits::TableAttributesLike;
use synql_core::structs::Workspace;
use synql_diesel_schema::prelude::*;
use synql_insertable::traits::TableInsertableLike;
use synql_models::traits::TableModelLike;
use synql_relations::prelude::*;

mod builder;
pub use builder::SynQLBuilder;

use crate::traits::SynQLDatabaseLike;

/// Struct representing a SQL workspace.
pub struct SynQL<'a, DB: SynQLDatabaseLike> {
    /// The underlying database which will be used to generate the workspace.
    database: &'a DB,
    /// The path to the workspace.
    path: &'a Path,
    /// List of tables to be excluded from the workspace, which also imply
    /// excluding all of the tables that depend on them via foreign keys.
    deny_list: Vec<&'a DB::Table>,
    /// Version of the generated workspace.
    version: (u8, u8, u8),
    /// Edition of the generated workspace.
    edition: u16,
    /// Whether to also generate the workspace TOML.
    generate_workspace_toml: bool,
    /// Whether to also generate the rustfmt configuration file.
    generate_rustfmt: bool,
}

impl<'a, DB: SynQLDatabaseLike> SynQL<'a, DB> {
    /// Create a new `SynQL` instance from a given database.
    pub fn new() -> SynQLBuilder<'a, DB> {
        SynQLBuilder::default()
    }

    fn skip_table(&self, table: &DB::Table) -> bool {
        for deny_table in &self.deny_list {
            if table.depends_on(self.database, deny_table) {
                return true;
            }
        }
        false
    }

    /// Executes the workspace generation.
    pub fn generate(&self) -> std::io::Result<()> {
        let mut workspace = Workspace::new()
            .path(self.path)
            .name(self.database.catalog_name())
            .expect("Invalid workspace name")
            .core()
            .expect("Unable to register `core` crate")
            .std()
            .expect("Unable to register `std` crate")
            .diesel_queries()
            .expect("Unable to register `diesel-queries` crate")
            .serde()
            .expect("Unable to register `serde` crate")
            .version(self.version.0, self.version.1, self.version.2)
            .edition(self.edition)
            .build()
            .expect("Unable to build workspace");

        for table in self.database.table_dag() {
            if self.skip_table(table) {
                continue;
            }
            workspace.add_internal_crate(table.schema_macro(&workspace, self.database).into());
            workspace.add_internal_crate(table.model(&workspace, self.database).into());
            workspace.add_internal_crate(table.relations_trait(&workspace, self.database).into());
            workspace.add_internal_crate(table.attributes(&workspace, self.database).into());
            // If the table has some generated columns, we need to generate an alternative
            // insertable struct that excludes those columns.
            if table.has_generated_columns(self.database) {
                workspace.add_internal_crate(table.insertable(&workspace, self.database).into());
            }
        }

        workspace.write_to_disk()?;

        if self.generate_workspace_toml {
            workspace.write_toml()?;
        }

        if self.generate_rustfmt {
            workspace.write_rustfmt()?;
        }

        Ok(())
    }
}
