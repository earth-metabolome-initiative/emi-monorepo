//! Submodule defining a `Workspace` struct representing a Cargo
//! workspace.

mod builder;
use std::{path::Path, sync::Arc};

pub use builder::WorkspaceBuilder;
use syn::Type;

use crate::{
    structs::{
        ExternalCrate, ExternalFunctionRef, InternalCrate,
        external_crate::{ExternalMacroRef, ExternalTraitRef, ExternalTypeRef},
    },
    traits::ExternalDependencies,
};

#[derive(Debug, Clone)]
/// Struct defining a Cargo workspace.
pub struct Workspace<'data> {
    /// External crates made available within the workspace.
    external_crates: Vec<Arc<ExternalCrate>>,
    /// Name of the workspace.
    name: String,
    /// Path where the workspace is being created.
    path: &'data Path,
    /// Version of the workspace.
    version: (u8, u8, u8),
    /// Edition of the workspace.
    edition: u16,
    /// Internal crates created within the workspace.
    internal_crates: Vec<Arc<InternalCrate>>,
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
    pub fn add_internal_crate(&mut self, internal_crate: InternalCrate) {
        self.internal_crates.push(Arc::new(internal_crate));
    }

    /// Returns the internal crate with the given name, if any.
    pub fn internal_crate(&self, name: &str) -> Option<&Arc<InternalCrate>> {
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
    pub fn external_macro(&self, name: &str) -> Option<ExternalMacroRef> {
        for ext_crate in &self.external_crates {
            if let Some(ext_macro) = ext_crate.external_macro(name) {
                return Some(ext_macro);
            }
        }
        None
    }

    /// Returns the external trait ref corresponding to the provided name, if
    /// any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external trait.
    pub fn external_trait(&self, name: &str) -> Option<ExternalTraitRef> {
        for ext_crate in &self.external_crates {
            if let Some(ext_trait) = ext_crate.external_trait_ref(name) {
                return Some(ext_trait);
            }
        }
        None
    }

    /// Returns the external type ref corresponding to the provided Postgres
    /// name, if any.
    ///
    /// # Arguments
    /// * `postgres_type` - A string slice representing the postgres type.
    pub fn external_postgres_type(&self, postgres_type: &str) -> Option<ExternalTypeRef> {
        for ext_crate in &self.external_crates {
            if let Some(ext_type) = ext_crate.external_postgres_type(postgres_type) {
                return Some(ext_type);
            }
        }
        None
    }

    /// Returns the external type ref corresponding to the provided name, if
    /// any.
    ///
    /// # Arguments
    /// * `ident` - A reference to the type.
    pub fn external_type(&self, ident: &Type) -> Option<ExternalTypeRef> {
        for ext_crate in &self.external_crates {
            if let Some(ext_type) = ext_crate.external_type(ident) {
                return Some(ext_type);
            }
        }
        None
    }

    /// Returns the external function ref corresponding to the provided name, if
    /// any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external
    ///   function.
    pub fn external_function(&self, name: &str) -> Option<ExternalFunctionRef> {
        for ext_crate in &self.external_crates {
            if let Some(ext_function) = ext_crate.external_function_ref(name) {
                return Some(ext_function);
            }
        }
        None
    }

    /// Writes the formatting rules for the workspace.
    pub fn write_rustfmt(&self) -> std::io::Result<()> {
        use std::io::Write;

        use toml_edit::{DocumentMut, Item};
        let mut doc = DocumentMut::new();

        doc["edition"] = Item::from(self.edition.to_string());
        doc["max_width"] = Item::from(100);
        doc["use_small_heuristics"] = Item::from("Max");
        doc["reorder_imports"] = Item::from(true);
        doc["group_imports"] = Item::from("StdExternalCrate");
        doc["imports_granularity"] = Item::from("Crate");
        doc["reorder_modules"] = Item::from(true);
        doc["wrap_comments"] = Item::from(true);
        doc["format_code_in_doc_comments"] = Item::from(true);
        doc["comment_width"] = Item::from(80);
        doc["normalize_comments"] = Item::from(true);
        doc["normalize_doc_attributes"] = Item::from(true);
        doc["force_multiline_blocks"] = Item::from(true);
        doc["fn_single_line"] = Item::from(false);
        doc["where_single_line"] = Item::from(false);

        let rustfmt_path = self.path.join("rustfmt.toml");
        let mut buffer = std::fs::File::create(rustfmt_path)?;
        write!(buffer, "{}", doc)
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
            let mut dep_table = toml_edit::InlineTable::new();
            dep_table.insert("path", Value::from(crate_name));
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
            let mut dep_table = toml_edit::InlineTable::new();
            let dep_name = external_crate.name();

            // Create table with version and features
            if let Some(version) = external_crate.version() {
                dep_table.insert("version", Value::from(version));
            }

            if let Some((repository, branch)) = external_crate.git() {
                dep_table.insert("git", Value::from(repository));
                dep_table.insert("branch", Value::from(branch));
            }

            let mut features_array = Array::new();
            for feature in external_crate.features() {
                features_array.push(feature.as_str());
            }
            dep_table.insert("features", Value::from(features_array));

            doc["workspace"]["dependencies"][dep_name] = toml_edit::value(dep_table);
        }

        // Add [workspace.lints] section
        doc["workspace"]["lints"]["rust"]["missing_docs"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unused_macro_rules"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unused_doc_comments"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unconditional_recursion"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unreachable_patterns"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unused_import_braces"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["unused_must_use"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rust"]["deprecated"] = toml_edit::value("deny");
        doc["workspace"]["lints"]["rustdoc"]["broken_intra_doc_links"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rustdoc"]["bare_urls"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rustdoc"]["invalid_codeblock_attributes"] =
            toml_edit::value("forbid");
        doc["workspace"]["lints"]["rustdoc"]["invalid_html_tags"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rustdoc"]["missing_crate_level_docs"] =
            toml_edit::value("forbid");
        doc["workspace"]["lints"]["rustdoc"]["unescaped_backticks"] = toml_edit::value("forbid");
        doc["workspace"]["lints"]["rustdoc"]["redundant_explicit_links"] =
            toml_edit::value("forbid");
        doc["workspace"]["lints"]["rustdoc"]["invalid_rust_codeblocks"] =
            toml_edit::value("forbid");

        // Write to file
        let mut buffer = std::fs::File::create(toml_path)?;
        write!(buffer, "{}", doc)
    }

    /// Writes the workspace to disk.
    pub fn write_to_disk(&self) -> std::io::Result<()> {
        // First, we eliminate all existing files in the workspace path.
        if self.path.exists() {
            // We remove all contents of the directory.
            for entry in std::fs::read_dir(self.path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(path)?;
                } else {
                    std::fs::remove_file(path)?;
                }
            }
        }
        // Then, we create the workspace directory.
        std::fs::create_dir_all(self.path)?;
        // And we start writing each internal crate to disk.
        for internal_crate in &self.internal_crates {
            internal_crate.write_to_disk(self)?;
        }
        Ok(())
    }
}
