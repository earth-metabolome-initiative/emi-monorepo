//! Submodule defining the `SQLWorkspace` trait which allows to generate the
//! `diesel` workspace from a SQL schema, based on `sql_traits`.

use std::{path::Path, sync::Arc};

use synql_attributes::traits::{TableAttributesLike, TableExtensionAttributesLike};
use synql_builders::prelude::*;
use synql_core::structs::Workspace;
use synql_diesel_schema::prelude::*;
use synql_insertable::traits::TableInsertableLike;
use synql_models::traits::TableModelLike;
use synql_relations::prelude::*;
use synql_value_settable::prelude::*;

mod builder;
pub use builder::SynQLBuilder;
use time_requirements::{prelude::TimeTracker, task::Task};

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
    /// External rust crates to include in the workspace.
    external_crates: Vec<Arc<ExternalCrate>>,
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
    pub fn generate(&self) -> std::io::Result<TimeTracker> {
        let mut workspace = Workspace::new()
            .path(self.path)
            .name(self.database.catalog_name())
            .expect("Invalid workspace name")
            .external_crates(self.external_crates.iter().cloned())
            .core()
            .std()
            .diesel_queries()
            .serde()
            .validation_errors()
            .postgis_diesel()
            .rosetta_uuid()
            .version(self.version.0, self.version.1, self.version.2)
            .edition(self.edition)
            .build()
            .expect("Unable to build workspace");

        let mut time_tracker = TimeTracker::new("SQL Workspace Generation");

        for table in self.database.table_dag() {
            if self.skip_table(table) {
                continue;
            }

            let schema_macro_task = Task::new("schema_macro");
            workspace.add_internal_crate(table.schema_macro(&workspace, self.database).into());
            time_tracker.add_or_extend_completed_task(schema_macro_task);

            let model_task = Task::new("model");
            workspace.add_internal_crate(table.model(&workspace, self.database).into());
            time_tracker.add_or_extend_completed_task(model_task);

            let relations_trait_task = Task::new("relations_trait");
            workspace.add_internal_crate(table.relations_trait(&workspace, self.database).into());
            time_tracker.add_or_extend_completed_task(relations_trait_task);

            if let Some(extension_attribute) = table.extension_attributes(&workspace, self.database)
            {
                let extension_attribute_task = Task::new("extension_attributes");
                workspace.add_internal_crate(extension_attribute.into());
                time_tracker.add_or_extend_completed_task(extension_attribute_task);
            }

            let attributes_task = Task::new("attributes");
            workspace.add_internal_crate(table.attributes(&workspace, self.database).into());
            time_tracker.add_or_extend_completed_task(attributes_task);

            let value_settable_trait_task = Task::new("value_settable_trait");
            workspace
                .add_internal_crate(table.value_settable_trait(&workspace, self.database).into());
            time_tracker.add_or_extend_completed_task(value_settable_trait_task);

            let insertable_task = Task::new("insertable");
            workspace.add_internal_crate(table.insertable(&workspace, self.database).into());
            time_tracker.add_or_extend_completed_task(insertable_task);

            let buildable_task = Task::new("buildable");
            workspace.add_internal_crate(table.buildable(&workspace, self.database).into());
            time_tracker.add_or_extend_completed_task(buildable_task);
        }

        let workspace_write_task = Task::new("workspace_write_to_disk");
        workspace.write_to_disk()?;
        time_tracker.add_or_extend_completed_task(workspace_write_task);

        if self.generate_workspace_toml {
            let workspace_toml_task = Task::new("workspace_toml");
            workspace.write_toml()?;
            time_tracker.add_or_extend_completed_task(workspace_toml_task);
        }

        if self.generate_rustfmt {
            let workspace_rustfmt_task = Task::new("workspace_rustfmt");
            workspace.write_rustfmt()?;
            time_tracker.add_or_extend_completed_task(workspace_rustfmt_task);
        }

        Ok(time_tracker)
    }
}
