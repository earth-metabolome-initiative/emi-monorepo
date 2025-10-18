//! Submodule defining a `Workspace` struct representing a Cargo
//! workspace.

mod builder;
use std::{path::Path, rc::Rc};

pub use builder::WorkspaceBuilder;

use crate::structs::{ExternalCrate, InternalCrate, external_crate::ExternalMacroRef};

/// Struct defining a Cargo workspace.
pub struct Workspace<'data> {
    /// External crates made available within the workspace.
    external_crates: Vec<&'data ExternalCrate>,
    /// Name of the workspace.
    name: String,
    /// Path where the workspace is being created.
    path: &'data Path,
    /// Version of the workspace.
    version: (u8, u8, u8),
    /// Edition of the workspace.
    edition: u16,
    /// Internal crates created within the workspace.
    internal_crates: Vec<Rc<InternalCrate<'data>>>,
}

impl<'data> Workspace<'data> {
    /// Inizializes a new `WorkspaceBuilder`.
    pub fn new() -> WorkspaceBuilder<'data> {
        WorkspaceBuilder::default()
    }

    /// Returns the version tuple of the workspace.
    pub fn version(&self) -> (u8, u8, u8) {
        self.version
    }

    /// Returns the edition of the workspace.
    pub fn edition(&self) -> u16 {
        self.edition
    }

    /// Returns the path where the workspace is being created.
    pub fn path(&self) -> &Path {
        self.path
    }

    /// Adds a new internal crate to the workspace.
    pub fn add_internal_crate(&mut self, internal_crate: InternalCrate<'data>) {
        self.internal_crates.push(Rc::new(internal_crate));
    }

    /// Returns the internal crate with the given name, if any.
    pub fn internal_crate(&self, name: &str) -> Option<&Rc<InternalCrate<'data>>> {
        for internal_crate in &self.internal_crates {
            if internal_crate.name() == name {
                return Some(internal_crate);
            }
        }
        None
    }

    /// Returns the external macro ref corresponding to the provided name, if
    /// any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external macro.
    pub fn external_macro(&self, name: &str) -> Option<ExternalMacroRef<'data>> {
        for ext_crate in &self.external_crates {
            if let Some(ext_macro) = ext_crate.external_macro(name) {
                return Some(ext_macro.clone());
            }
        }
        None
    }

    /// Returns the diesel [`Type`](syn::Type) corresponding to the
    /// provided postgres type.
    ///
    /// # Arguments
    ///
    /// * `pg_type` - A string slice representing the postgres type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use synql_core::prelude::*;
    /// use quote::ToTokens;
    /// let workspace = Workspace::new().name("my_workspace")?.version(0, 1, 0).core()?.build()?;
    ///
    /// let diesel_type = workspace.diesel_type("int4").expect("Type not found").clone();
    /// assert_eq!(diesel_type.to_token_stream().to_string(), "diesel :: sql_types :: Integer");
    /// # Ok(())
    /// # }
    /// ```
    pub fn diesel_type(&self, pg_type: &str) -> Option<&syn::Type> {
        for ext_crate in &self.external_crates {
            if let Some(compatible_type) = ext_crate.compatible_type(pg_type) {
                return Some(compatible_type.diesel_type());
            }
        }
        None
    }

    /// Writes the workspace TOML.
    pub fn write_toml(&self) -> std::io::Result<()> {
        use std::io::Write;

        use toml_edit::{Array, DocumentMut, Value};

        let toml_path = self.path.join("Cargo.toml");

        // Create a new TOML document
        let mut doc = DocumentMut::new();

        // Add [workspace] section
        doc["workspace"]["resolver"] = toml_edit::value("2");

        // Add [workspace.package] section
        doc["workspace"]["package"]["edition"] = toml_edit::value(self.edition.to_string());

        // Add members array
        let mut members = Array::new();
        for internal_crate in &self.internal_crates {
            let crate_path = internal_crate.name().to_string();
            members.push(crate_path);
        }
        doc["workspace"]["members"] = toml_edit::value(members);

        // Add [workspace.dependencies] section
        for internal_crate in &self.internal_crates {
            let crate_name = internal_crate.name();
            let crate_path = self.path.join(crate_name);
            let mut dep_table = toml_edit::InlineTable::new();
            dep_table.insert("path", Value::from(crate_path.display().to_string()));
            doc["workspace"]["dependencies"][crate_name] = toml_edit::value(dep_table);
        }

        // Add external dependencies that are required
        let mut external_deps = Vec::new();
        for internal_crate in &self.internal_crates {
            external_deps.extend(internal_crate.external_dependencies());
        }
        external_deps.sort_unstable();
        external_deps.dedup();

        for external_crate in external_deps {
            if !external_crate.is_dependency() {
                continue;
            }
            let dep_name = external_crate.name();
            doc["workspace"]["dependencies"][dep_name] =
                toml_edit::value(external_crate.version().as_deref().unwrap_or("*"));
        }

        // Add [workspace.lints] section
        doc["workspace"]["lints"]["rust"]["missing_docs"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unused_macro_rules"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unconditional_recursion"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unreachable_patterns"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unused_import_braces"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unused_must_use"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["deprecated"] = toml_edit::value("deny");

        // Write to file
        let mut buffer = std::fs::File::create(toml_path)?;
        write!(buffer, "{}", doc)
    }

    /// Writes the workspace to disk.
    pub fn write_to_disk(&self) -> std::io::Result<()> {
        // First, we eliminate all existing files in the workspace path.
        if self.path.exists() {
            std::fs::remove_dir_all(self.path)?;
        }
        // Then, we create the workspace directory.
        std::fs::create_dir_all(self.path)?;
        // And we start writing each internal crate to disk.
        for internal_crate in &self.internal_crates {
            println!("Writing crate {} to disk", internal_crate.name());
            internal_crate.write_to_disk(self)?;
        }
        Ok(())
    }
}
