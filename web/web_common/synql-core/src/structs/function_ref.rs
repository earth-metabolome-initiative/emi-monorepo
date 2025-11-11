//! Submodule providing the `FunctionRef` struct representing a reference to a
//! function defined in an external crate.

use std::{fmt::Debug, sync::Arc};

use quote::ToTokens;

use crate::{
    structs::{DataVariantRef, ExternalCrate, Method},
    traits::ExternalDependencies,
};

#[derive(Clone, Debug)]
/// Struct representing a reference to a function defined in an external crate.
pub struct ExternalFunctionRef {
    /// The underlying method.
    method: Arc<Method>,
    /// The path to the function in the external crate.
    path: Arc<syn::Path>,
    /// The crate from which the function is referenced.
    crate_ref: Arc<ExternalCrate>,
}

impl ExternalFunctionRef {
    /// Creates a new `ExternalFunctionRef` instance.
    pub fn new(method: Arc<Method>, path: Arc<syn::Path>, crate_ref: Arc<ExternalCrate>) -> Self {
        Self { method, path, crate_ref }
    }

    /// Returns the underlying method.
    #[must_use]
    pub fn method(&self) -> &Method {
        self.method.as_ref()
    }

    /// Returns the crate from which the function is referenced.
    #[must_use]
    pub fn crate_ref(&self) -> &Arc<ExternalCrate> {
        &self.crate_ref
    }

    /// Returns whether the function returns a result type.
    pub fn can_fail(&self) -> bool {
        self.method.can_fail()
    }

    /// Returns the return type of the function, if any.
    pub fn return_type(&self) -> Option<&DataVariantRef> {
        self.method.return_type()
    }
}

impl ToTokens for ExternalFunctionRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.path.to_tokens(tokens);
    }
}

impl ExternalDependencies for ExternalFunctionRef {
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>> {
        vec![self.crate_ref.clone()]
    }
}
