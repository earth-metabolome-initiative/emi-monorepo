//! Submodule defining a struct which represents a rust module.

mod builder;

use std::sync::Arc;

pub use builder::InternalModuleBuilder;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::{
    structs::{
        InternalCrate, InternalData, InternalToken, InternalTrait, ModuleDocumentation, Publicness,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a rust module.
pub struct InternalModule {
    /// Name of the module.
    name: String,
    /// The submodules it contains.
    submodules: Vec<InternalModule>,
    /// Publicness of the module.
    publicness: Publicness,
    /// Data structs defined within the module.
    data: Vec<Arc<InternalData>>,
    /// Internal trait defined within the module.
    internal_traits: Vec<Arc<InternalTrait>>,
    /// Internal token streams defined within the module.
    internal_tokens: Vec<InternalToken>,
    /// Module documentation.
    documentation: ModuleDocumentation,
}

impl InternalModule {
    /// Initializes a new `InternalModuleBuilder`.
    pub fn new() -> InternalModuleBuilder {
        InternalModuleBuilder::default()
    }

    /// Returns the name of the module.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ident of the module.
    #[inline]
    pub fn ident(&self) -> Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    /// Returns the publicness of the module.
    #[inline]
    pub fn publicness(&self) -> &Publicness {
        &self.publicness
    }

    /// Returns whether the module is public.
    #[inline]
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns whether the module has submodules.
    #[inline]
    pub fn has_submodules(&self) -> bool {
        !self.submodules.is_empty()
    }

    /// Returns the relative path to the provided sub-module within this module
    /// if it exists.
    ///
    /// # Arguments
    ///
    /// * `module` - The sub-module to get the path for.
    pub fn submodule_path(&self, module: &InternalModule) -> Option<syn::Path> {
        if module == self {
            return Some(syn::Path::from(self.ident()));
        }
        for submodule in &self.submodules {
            if let Some(mut path) = submodule.submodule_path(module) {
                let mut segments = syn::punctuated::Punctuated::new();
                segments.push(syn::PathSegment::from(self.ident()));
                segments.push(syn::PathSegment::from(path.segments.first().unwrap().ident.clone()));
                segments.extend(path.segments.iter().skip(1).cloned());
                path.segments = segments;
                return Some(path);
            }
        }
        None
    }

    /// Returns whether the module contains public items (submodules, data,
    /// other tokens).
    pub fn contains_public_items(&self) -> bool {
        self.is_public()
            && (self.submodules.iter().any(|m| m.contains_public_items() && m.is_public())
                || self.data.iter().any(|d| d.is_public())
                || self.internal_tokens.iter().any(|t| t.is_public()))
    }

    /// Returns a reference to the internal data with the given name if it
    /// exists in the module.
    pub fn internal_data(&self, name: &str) -> Option<&Arc<InternalData>> {
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

    /// Returns a reference to the internal trait with the given name if it
    /// exists in the module.
    pub fn internal_trait(&self, name: &str) -> Option<&Arc<InternalTrait>> {
        for internal_trait in &self.internal_traits {
            if internal_trait.name() == name {
                return Some(internal_trait);
            }
        }
        for submodule in &self.submodules {
            if let Some(internal_trait) = submodule.internal_trait(name) {
                return Some(internal_trait);
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
        let token_stream = self.to_token_stream().to_string();
        let syntax_tree = syn::parse_file(&token_stream).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse generated code: {}", e),
            )
        })?;
        let formatted = prettyplease::unparse(&syntax_tree);
        std::fs::write(&module_path, formatted)?;
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

impl InternalDependencies for InternalModule {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        let vec: Vec<&InternalCrate> = self
            .submodules
            .iter()
            .flat_map(|submodule| submodule.internal_dependencies())
            .chain(self.data.iter().flat_map(|data| data.internal_dependencies()))
            .chain(
                self.internal_traits
                    .iter()
                    .flat_map(|internal_trait| internal_trait.internal_dependencies()),
            )
            .chain(self.internal_tokens.iter().flat_map(|token| token.internal_dependencies()))
            .chain(self.documentation.internal_dependencies())
            .collect();
        vec.into_iter()
    }
}

impl ExternalDependencies for InternalModule {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &crate::structs::ExternalCrate> {
        let vec: Vec<&crate::structs::ExternalCrate> = self
            .submodules
            .iter()
            .flat_map(|submodule| submodule.external_dependencies())
            .chain(self.data.iter().flat_map(|data| data.external_dependencies()))
            .chain(
                self.internal_traits
                    .iter()
                    .flat_map(|internal_trait| internal_trait.external_dependencies()),
            )
            .chain(self.internal_tokens.iter().flat_map(|token| token.external_dependencies()))
            .chain(self.documentation.external_dependencies())
            .collect();
        vec.into_iter()
    }
}

impl ToTokens for InternalModule {
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
        let data = self.data.iter().map(|d| d.as_ref());
        let internal_tokens = &self.internal_tokens;
        let internal_traits = self.internal_traits.iter().map(|t| t.as_ref());
        let documentation = &self.documentation;
        tokens.extend(quote! {
            #documentation
            #(#submodules)*
            #(#uses)*

            #(#data)*

            #(#internal_traits)*
            #(#internal_tokens)*
        })
    }
}
