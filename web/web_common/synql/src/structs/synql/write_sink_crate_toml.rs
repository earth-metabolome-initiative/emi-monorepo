//! Submodule implementing the writing of the sink crate toml files.

use std::{io::Write, path::Path};

use crate::{
    structs::{SynQL, Workspace},
    traits::{SynQLDatabaseLike, table::TableSynLike},
};

impl<DB: SynQLDatabaseLike> SynQL<'_, DB> {
    pub(super) fn write_sink_crate_toml(
        &self,
        workspace: &Workspace,
        sink_crate_name: &str,
        sink_crate_path: &Path,
    ) -> Result<(), crate::Error> {
        let cargo_toml_path = sink_crate_path.join("Cargo.toml");
        let mut buffer = std::fs::File::create(cargo_toml_path)?;
        let (major, minor, patch) = workspace.version();

        writeln!(
            buffer,
            r#"[package]
name = "{sink_crate_name}"
version = "{major}.{minor}.{patch}"
edition.workspace = true
"#
        )?;

        // Add dependencies
        writeln!(buffer, "\n[dependencies]")?;

        for table in self.database.tables() {
            if self.skip_table(table) {
                continue;
            }
            let crate_name = table.crate_name(workspace);
            writeln!(buffer, "{crate_name}.workspace = true")?;
        }

        // Linting
        writeln!(buffer, "\n[lints]")?;
        writeln!(buffer, "workspace = true")?;

        Ok(())
    }
}
