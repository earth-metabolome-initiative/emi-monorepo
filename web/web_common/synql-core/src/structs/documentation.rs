//! Submodule providing a documentation struct which contains the documentation
//! string, and potentially other crate dependencies when the documentation
//! contains links to other crates.

mod builder;

use std::rc::Rc;

pub use builder::DocumentationBuilder;
use quote::ToTokens;

use crate::{
    structs::{ExternalCrate, InternalCrate},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining documentation for various structs, enums, traits, etc.
pub struct Documentation<'data> {
    /// The documentation string.
    documentation: String,
    /// The external crate dependencies required by this documentation.
    external_dependencies: Vec<&'data ExternalCrate<'data>>,
    /// The internal crate dependencies required by this documentation.
    internal_dependencies: Vec<Rc<InternalCrate<'data>>>,
}

impl<'data> Documentation<'data> {
    /// Initializes a new `DocumentationBuilder`.
    pub fn new() -> DocumentationBuilder<'data> {
        DocumentationBuilder::default()
    }

    /// Returns the documentation string.
    pub fn documentation(&self) -> &str {
        &self.documentation
    }
}

impl<'data> ExternalDependencies<'data> for Documentation<'data> {
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
        self.external_dependencies.clone()
    }
}

impl<'data> InternalDependencies<'data> for Documentation<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        self.internal_dependencies.iter().map(|c| c.as_ref()).collect()
    }
}

impl ToTokens for Documentation<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // Split documentation by newlines to create separate doc attributes
        let doc_lines: Vec<&str> = self.documentation.lines().collect();
        tokens.extend(quote::quote! {
            #(#[doc = #doc_lines])*
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining documentation specifically for modules, which uses a
/// different ToTokens implementation (with `#![doc = ...]` instead of `#[doc =
/// ...]`).
pub struct ModuleDocumentation<'data> {
    /// The underlying documentation.
    documentation: Documentation<'data>,
}

impl<'data> From<Documentation<'data>> for ModuleDocumentation<'data> {
    fn from(documentation: Documentation<'data>) -> Self {
        ModuleDocumentation { documentation }
    }
}

impl<'data> ModuleDocumentation<'data> {
    /// Returns a reference to the underlying documentation.
    pub fn documentation(&self) -> &Documentation<'data> {
        &self.documentation
    }

    /// Returns the documentation string.
    pub fn documentation_string(&self) -> &str {
        self.documentation.documentation()
    }
}

impl<'data> ExternalDependencies<'data> for ModuleDocumentation<'data> {
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
        self.documentation.external_dependencies()
    }
}

impl<'data> InternalDependencies<'data> for ModuleDocumentation<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        self.documentation.internal_dependencies()
    }
}

impl ToTokens for ModuleDocumentation<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // For modules, use inner doc attributes (#![doc = ...])
        let doc_string = self.documentation.documentation();
        tokens.extend(quote::quote! {
            #![doc = #doc_string]
        });
    }
}
