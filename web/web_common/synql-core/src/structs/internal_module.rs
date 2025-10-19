//! Submodule defining a struct which represents a rust module.

mod builder;

use std::rc::Rc;

pub use builder::InternalModuleBuilder;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::structs::{InternalCrate, InternalData, InternalToken, InternalTrait, Publicness};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a rust module.
pub struct InternalModule<'data> {
    /// Name of the module.
    name: String,
    /// The submodules it contains.
    submodules: Vec<InternalModule<'data>>,
    /// Publicness of the module.
    publicness: Publicness,
    /// Data structs defined within the module.
    data: Vec<Rc<InternalData<'data>>>,
    /// Internal trait defined within the module.
    internal_traits: Vec<Rc<InternalTrait<'data>>>,
    /// Internal token streams defined within the module.
    internal_tokens: Vec<InternalToken<'data>>,
    /// Module documentation.
    documentation: String,
}

impl<'data> InternalModule<'data> {
    /// Initializes a new `InternalModuleBuilder`.
    pub fn new() -> InternalModuleBuilder<'data> {
        InternalModuleBuilder::default()
    }

    /// Returns the name of the module.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ident of the module.
    pub fn ident(&self) -> Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    /// Returns the publicness of the module.
    pub fn publicness(&self) -> &Publicness {
        &self.publicness
    }

    /// Returns whether the module is public.
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns whether the module has submodules.
    pub fn has_submodules(&self) -> bool {
        !self.submodules.is_empty()
    }

    /// Returns whether the module contains public items (submodules, data,
    /// other tokens).
    pub fn contains_public_items(&self) -> bool {
        self.is_public()
            && (self.submodules.iter().any(|m| m.contains_public_items() && m.is_public())
                || self.data.iter().any(|d| d.is_public())
                || self.internal_tokens.iter().any(|t| t.is_public()))
    }

    /// Returns the sorted unique internal dependencies of the module.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for submodule in &self.submodules {
            dependencies.extend(submodule.internal_dependencies());
        }
        for data in &self.data {
            dependencies.extend(data.internal_dependencies());
        }
        for internal_trait in &self.internal_traits {
            dependencies.extend(internal_trait.internal_dependencies());
        }
        for token in &self.internal_tokens {
            dependencies.extend(token.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }

    /// Returns the sorted unique external dependencies of the module.
    pub fn external_dependencies(&self) -> Vec<&'data crate::structs::ExternalCrate> {
        let mut dependencies = Vec::new();
        for submodule in &self.submodules {
            dependencies.extend(submodule.external_dependencies());
        }
        for data in &self.data {
            dependencies.extend(data.external_dependencies());
        }
        for internal_trait in &self.internal_traits {
            dependencies.extend(internal_trait.external_dependencies());
        }
        for token in &self.internal_tokens {
            dependencies.extend(token.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }

    /// Returns a reference to the internal data with the given name if it
    /// exists in the module.
    pub fn internal_data(&self, name: &str) -> Option<&Rc<InternalData<'data>>> {
        for data in &self.data {
            if data.name() == name {
                return Some(data);
            }
        }
        for submodule in &self.submodules {
            if let Some(data) = submodule.internal_data(name) {
                return Some(data);
            }
        }
        None
    }

    /// Writes the module to disk at the specified path.
    ///
    /// # Arguments
    /// * `path` - The path where to write the module.
    pub fn write_to_disk(&self, path: &std::path::Path) -> std::io::Result<()> {
        let module_path = path.join(format!("{}.rs", self.name()));
        std::fs::write(&module_path, self.to_token_stream().to_string())?;
        let module_directory = path.join(self.name());
        if self.has_submodules() {
            std::fs::create_dir_all(&module_directory)?;
        }
        for submodule in &self.submodules {
            submodule.write_to_disk(&module_directory)?;
        }
        Ok(())
    }
}

impl ToTokens for InternalModule<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let publicness = &self.publicness;
        let submodules = self.submodules.iter().map(|m| {
            let name = m.ident();
            quote! {
                #publicness mod #name;
            }
        });
        let uses = self.submodules.iter().filter(|m| m.contains_public_items()).map(|m| {
            let name = m.ident();
            quote! {
                #publicness use #name::*;
            }
        });
        let data = &self.data;
        let internal_tokens = self.internal_tokens.iter().map(|t| t.to_token_stream());
        let documentation = &self.documentation;
        tokens.extend(quote! {
            #![doc = #documentation]
            #(#submodules)*
            #(#uses)*

            #(#data)*

            #(#internal_tokens)*
        })
    }
}
