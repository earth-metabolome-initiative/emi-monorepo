//! Submodule providing a struct which defines a crate model.

mod builder;

use std::{path::Path, rc::Rc};

pub use builder::InternalCrateBuilder;
use quote::{ToTokens, quote};
use syn::{Ident, Token};

use crate::structs::{InternalData, InternalModule, Workspace};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining a crate model.
pub struct InternalCrate<'data> {
    /// Name of the crate.
    name: String,
    /// The root modules of the crate.
    modules: Vec<Rc<InternalModule<'data>>>,
    /// Crate documentation.
    documentation: String,
}

impl<'data> ToTokens for InternalCrate<'data> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // The crate to-tokens curresponds with the `lib` file of a crate.
        let modules = self.modules.iter().map(|m| {
            let name = &m.ident();
            let publicness = &m.publicness();
            quote! {
                #publicness mod #name;
            }
        });
        let module_exports = self.modules.iter().filter(|m| m.is_public()).map(|m| m.ident());
        let documentation = &self.documentation;
        tokens.extend(quote! {
            #![doc = #documentation]

            #(#modules)*

            #[doc = "Prelude module re-exporting commonly used items."]
            pub mod prelude {
                #(pub use crate::#module_exports::*;)*
            }
        })
    }
}

impl<'data> InternalCrate<'data> {
    /// Initializes a new `InternalCrateBuilder`.
    pub fn new() -> InternalCrateBuilder<'data> {
        InternalCrateBuilder::default()
    }

    /// Returns a reference to the name of the crate.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the ident of the crate.
    pub fn ident(&self) -> Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    /// Returns the path to the provided module within the crate, if it exists.
    pub fn module_path(&self, module: &InternalModule<'data>) -> Option<syn::Path> {
        let crate_ident = self.ident();
        for m in &self.modules {
            if let Some(path) = m.submodule_path(module) {
                let full_path = syn::Path {
                    leading_colon: Some(Token![::](proc_macro2::Span::call_site())),
                    segments: std::iter::once(syn::PathSegment::from(crate_ident))
                        .chain(path.segments.into_iter())
                        .collect(),
                };
                return Some(full_path);
            }
        }
        None
    }

    /// Returns the internal dependencies of the crate.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for module in &self.modules {
            dependencies.extend(module.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }

    /// Returns a reference to the internal data with the given name if it
    /// exists in the crate.
    pub fn internal_data(&self, name: &str) -> Option<&Rc<InternalData<'data>>> {
        for module in &self.modules {
            if let Some(data) = module.internal_data(name) {
                return Some(data);
            }
        }
        None
    }

    /// Returns a reference to the module with the given name if it exists in
    /// the crate.
    pub fn module(&self, name: &str) -> Option<&Rc<InternalModule<'data>>> {
        for module in &self.modules {
            if module.name() == name {
                return Some(module);
            }
        }
        None
    }

    /// Returns the external dependencies of the crate.
    pub fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for module in &self.modules {
            dependencies.extend(module.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }

    /// Writes out the TOML metadata for the crate at the provided path.
    pub fn write_toml(&self, workspace: &Workspace, path: &std::path::Path) -> std::io::Result<()> {
        use std::io::Write;

        use toml_edit::{DocumentMut, InlineTable, Value};

        let cargo_toml_path = path.join("Cargo.toml");
        let (major, minor, edit) = workspace.version();

        // Create a new TOML document
        let mut doc = DocumentMut::new();

        // Add [package] section
        doc["package"]["name"] = toml_edit::value(&self.name);
        doc["package"]["version"] = toml_edit::value(format!("{major}.{minor}.{edit}"));
        doc["package"]["edition"] = toml_edit::value(workspace.edition().to_string());

        // Add [dependencies] section
        let internal_dependencies = self.internal_dependencies();
        for dep in internal_dependencies {
            let dep_name = dep.name();

            // We add as a prefix "../" to the path to indicate that the dependency is
            // located in the parent directory of the current crate.
            let crate_path = Path::new("..").join(dep_name);

            let mut dep_table = InlineTable::new();
            dep_table.insert("version", Value::from(format!("{major}.{minor}.{edit}")));
            dep_table.insert("path", Value::from(crate_path.display().to_string()));
            doc["dependencies"][dep_name] = toml_edit::value(dep_table);
        }

        let external_dependencies = self.external_dependencies();
        for dep in external_dependencies {
            if !dep.is_dependency() {
                continue;
            }
            let dep_name = dep.name();
            let mut dep_table = InlineTable::new();
            dep_table.insert("workspace", Value::from(true));
            doc["dependencies"][dep_name] = toml_edit::value(dep_table);
        }

        // Add [lints] section
        let mut lints_table = InlineTable::new();
        lints_table.insert("workspace", Value::from(true));
        doc["lints"] = toml_edit::value(lints_table);

        // Write to file
        let mut buffer = std::fs::File::create(cargo_toml_path)?;
        write!(buffer, "{}", doc)
    }

    /// Writes the crate to disk at the provided path.
    pub fn write_to_disk(&self, workspace: &Workspace) -> std::io::Result<()> {
        use std::io::Write;
        // We create the crate directory.
        let crate_path = workspace.path().join(&self.name);
        std::fs::create_dir_all(&crate_path)?;
        // We write the Cargo.toml file.
        self.write_toml(workspace, &crate_path)?;
        // We create the src directory.
        let src_path = crate_path.join("src");
        std::fs::create_dir_all(&src_path)?;
        // We write the lib.rs file.
        let lib_path = src_path.join("lib.rs");
        let mut buffer = std::fs::File::create(lib_path)?;
        write!(buffer, "{}", self.to_token_stream())?;
        // We write each module to disk.
        for module in &self.modules {
            module.write_to_disk(&src_path)?;
        }
        Ok(())
    }
}
