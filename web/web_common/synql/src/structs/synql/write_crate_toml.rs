//! Submodule implementing the writing of the crate toml files.

use std::{borrow::Borrow, io::Write};

use sql_relations::prelude::TableLike;

use crate::{
    structs::{SynQL, Workspace},
    traits::{SynQLDatabaseLike, table::TableSynLike},
};

impl<DB: SynQLDatabaseLike> SynQL<'_, DB> {
    pub(super) fn write_crate_toml(
        &self,
        table: &DB::Table,
        workspace: &Workspace,
    ) -> Result<(), crate::Error> {
        let cargo_toml_path = table.crate_path(workspace).join("Cargo.toml");
        let mut buffer = std::fs::File::create(cargo_toml_path)?;
        let name = table.crate_name(workspace);
        let (major, minor, patch) = workspace.version();

        writeln!(
            buffer,
            r#"[package]
name = "{name}"
version = "{major}.{minor}.{patch}"
edition.workspace = true
"#
        )?;

        // Add dependencies
        writeln!(buffer, "\n[dependencies]")?;
        writeln!(buffer, "serde.workspace = true")?;
        writeln!(buffer, "diesel-builders.workspace = true")?;
        writeln!(buffer, "diesel.workspace = true")?;

        // If the table has check constraints, it will require the `validation_errors`
        // crate.
        if table.has_check_constraints(self.database) {
            writeln!(buffer, "validation_errors.workspace = true")?;
        }

        // The crate might have external dependencies relative to the types it uses
        // and the function employed in its check constraints.
        for external_crate in table.external_crates(self.database, workspace) {
            let crate_name = external_crate.name();
            if crate_name == "serde"
                || crate_name == "std"
                || crate_name == "core"
                || crate_name == "diesel"
                || crate_name == "diesel-builders"
            {
                continue;
            }
            writeln!(buffer, "{crate_name}.workspace = true")?;
        }

        // We include the crates associated to tables this table depends on
        for dependency in table.referenced_tables(self.database) {
            // We skip eventual self-dependencies.
            if dependency.borrow() == table {
                continue;
            }
            let dep_crate_name = dependency.crate_name(workspace);
            writeln!(buffer, "{dep_crate_name}.workspace = true")?;
        }

        // Linting
        writeln!(buffer, "\n[lints]")?;
        writeln!(buffer, "workspace = true")?;

        Ok(())
    }
}
