//! Submodule providing a struct which defines a crate model.

mod builder;

use std::{hash::Hash, path::Path, sync::Arc};

pub use builder::InternalCrateBuilder;
use quote::{ToTokens, quote};
use syn::{Ident, Token};

use crate::{
    structs::{InternalData, InternalModule, InternalTrait, ModuleDocumentation, Workspace},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone)]
/// Struct defining a crate model.
pub struct InternalCrate {
    /// Name of the crate.
    name: String,
    /// The root modules of the crate.
    modules: Vec<Arc<InternalModule>>,
    /// Crate documentation.
    documentation: ModuleDocumentation,
}

impl PartialEq for InternalCrate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for InternalCrate {}

impl Hash for InternalCrate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialOrd for InternalCrate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for InternalCrate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

unsafe impl Send for InternalCrate {}
unsafe impl Sync for InternalCrate {}

impl ToTokens for InternalCrate {
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
            #documentation

            #(#modules)*

            #[doc = "Prelude module re-exporting commonly used items."]
            pub mod prelude {
                #(pub use crate::#module_exports::*;)*
            }
        })
    }
}

impl InternalCrate {
    /// Initializes a new `InternalCrateBuilder`.
    pub fn new() -> InternalCrateBuilder {
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
    pub fn module_path(&self, module: &InternalModule) -> Option<syn::Path> {
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

    /// Returns a reference to the internal data with the given name if it
    /// exists in the crate.
    pub fn internal_data(&self, name: &str) -> Option<&Arc<InternalData>> {
        for module in &self.modules {
            if let Some(data) = module.internal_data(name) {
                return Some(data);
            }
        }
        None
    }

    /// Returns a reference to the internal trait with the given name if it
    /// exists in the crate.
    pub fn internal_trait(&self, name: &str) -> Option<&Arc<InternalTrait>> {
        for module in &self.modules {
            if let Some(trait_) = module.internal_trait(name) {
                return Some(trait_);
            }
        }
        None
    }

    /// Returns a reference to the module with the given name if it exists in
    /// the crate.
    pub fn module(&self, name: &str) -> Option<&Arc<InternalModule>> {
        for module in &self.modules {
            if module.name() == name {
                return Some(module);
            }
        }
        None
    }

    /// Writes out the TOML metadata for the crate at the provided path.
    pub fn write_toml(&self, workspace: &Workspace, path: &std::path::Path) -> std::io::Result<()> {
        use std::io::Write;

        let cargo_toml_path = path.join("Cargo.toml");
        let (major, minor, edit) = workspace.version();

        let mut buffer = std::fs::File::create(cargo_toml_path)?;

        // Write [package] section
        writeln!(buffer, "[package]")?;
        writeln!(buffer, "name = \"{}\"", self.name)?;
        writeln!(buffer, "version = \"{}.{}.{}\"", major, minor, edit)?;
        writeln!(buffer, "edition = \"{}\"", workspace.edition())?;
        writeln!(buffer)?;

        // Collect and sort dependencies
        let mut internal_dependencies = self.internal_dependencies().collect::<Vec<_>>();
        internal_dependencies.sort_unstable();
        internal_dependencies.dedup();

        let mut external_dependencies = self.external_dependencies().collect::<Vec<_>>();
        external_dependencies.sort_unstable();
        external_dependencies.dedup();

        // Write [dependencies] section if there are any
        if !internal_dependencies.is_empty() || !external_dependencies.is_empty() {
            writeln!(buffer, "[dependencies]")?;

            for dep in internal_dependencies {
                let dep_name = dep.name();
                let crate_path = Path::new("..").join(dep_name);
                writeln!(
                    buffer,
                    "{} = {{ version = \"{}.{}.{}\", path = \"{}\" }}",
                    dep_name,
                    major,
                    minor,
                    edit,
                    crate_path.display()
                )?;
            }

            for dep in external_dependencies {
                if !dep.is_dependency() {
                    continue;
                }
                writeln!(buffer, "{} = {{ workspace = true }}", dep.name())?;
            }

            writeln!(buffer)?;
        }

        // Write [lints] section
        writeln!(buffer, "[lints]")?;
        writeln!(buffer, "workspace = true")?;

        Ok(())
    }

    /// Writes the crate to disk at the provided path.
    pub fn write_to_disk(&self, workspace: &Workspace) -> std::io::Result<()> {
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
        let token_stream = self.to_token_stream().to_string();
        let syntax_tree = syn::parse_file(&token_stream).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse generated code: {}", e),
            )
        })?;
        let formatted = prettyplease::unparse(&syntax_tree);
        std::fs::write(&lib_path, formatted)?;
        // We write each module to disk.
        for module in &self.modules {
            module.write_to_disk(&src_path)?;
        }
        Ok(())
    }
}

impl InternalDependencies for InternalCrate {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.modules
            .iter()
            .flat_map(|module| module.internal_dependencies())
            .chain(self.documentation.internal_dependencies())
    }
}

impl ExternalDependencies for InternalCrate {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &crate::structs::ExternalCrate> {
        self.modules
            .iter()
            .flat_map(|module| module.external_dependencies())
            .chain(self.documentation.external_dependencies())
    }
}
