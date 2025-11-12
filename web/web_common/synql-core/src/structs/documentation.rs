//! Submodule providing a documentation struct which contains the documentation
//! string, and potentially other crate dependencies when the documentation
//! contains links to other crates.

mod builder;

use std::sync::Arc;

pub use builder::DocumentationBuilder;
use quote::ToTokens;

use crate::{
    structs::{ExternalCrate, InternalCrate},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining documentation for various structs, enums, traits, etc.
pub struct Documentation {
    /// The documentation string.
    documentation: String,
    /// The external crate dependencies required by this documentation.
    external_dependencies: Vec<Arc<ExternalCrate>>,
    /// The internal crate dependencies required by this documentation.
    internal_dependencies: Vec<Arc<InternalCrate>>,
}

impl Default for Documentation {
    fn default() -> Self {
        Self {
            documentation: String::new(),
            external_dependencies: Vec::new(),
            internal_dependencies: Vec::new(),
        }
    }
}

impl Documentation {
    /// Initializes a new `DocumentationBuilder`.
    pub fn new() -> DocumentationBuilder {
        DocumentationBuilder::default()
    }

    /// Returns the documentation string.
    pub fn documentation(&self) -> &str {
        &self.documentation
    }

    /// Extends the current documentation with another documentation.
    pub fn extend(&mut self, other: &str) {
        if !self.documentation.is_empty() {
            self.documentation.push('\n');
        }
        self.documentation.push_str(other);
    }
}

impl ExternalDependencies for Documentation {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.external_dependencies.iter().map(|c| c.as_ref())
    }
}

impl InternalDependencies for Documentation {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.internal_dependencies.iter().map(|c| c.as_ref())
    }
}

impl ToTokens for Documentation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // Split documentation by newlines to create separate doc attributes
        let documentation = &self.documentation;
        tokens.extend(quote::quote! {
            #[doc = #documentation]
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining documentation specifically for modules, which uses a
/// different ToTokens implementation (with `#![doc = ...]` instead of `#[doc =
/// ...]`).
pub struct ModuleDocumentation {
    /// The underlying documentation.
    documentation: Documentation,
}

impl From<Documentation> for ModuleDocumentation {
    fn from(documentation: Documentation) -> Self {
        ModuleDocumentation { documentation }
    }
}

impl ModuleDocumentation {
    /// Returns a reference to the underlying documentation.
    pub fn documentation(&self) -> &Documentation {
        &self.documentation
    }

    /// Returns the documentation string.
    pub fn documentation_string(&self) -> &str {
        self.documentation.documentation()
    }
}

impl ExternalDependencies for ModuleDocumentation {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.documentation.external_dependencies()
    }
}

impl InternalDependencies for ModuleDocumentation {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.documentation.internal_dependencies()
    }
}

impl ToTokens for ModuleDocumentation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // For modules, use inner doc attributes (#![doc = ...])
        let doc_string = self.documentation.documentation();
        tokens.extend(quote::quote! {
            #![doc = #doc_string]
        });
    }
}
