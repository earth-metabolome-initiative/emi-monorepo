//! Submodule defining a `WhereClause` struct.

mod builder;
use std::sync::Arc;

pub use builder::WhereClauseBuilder;
use quote::ToTokens;

use crate::{
    structs::{ExternalCrate, InternalCrate, InternalToken},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a where clause in a method or type definition.
pub struct WhereClause {
    /// Left-hand side of the where clause.
    left: InternalToken,
    /// Right-hand side of the where clause.
    right: InternalToken,
}

impl WhereClause {
    /// Initializes a new `WhereClauseBuilder`.
    pub fn new() -> WhereClauseBuilder {
        WhereClauseBuilder::default()
    }
}

impl InternalDependencies for WhereClause {
    fn internal_dependencies(&self) -> Vec<&InternalCrate> {
        let mut deps = self.left.internal_dependencies();
        deps.extend(self.right.internal_dependencies());
        deps
    }
}

impl ExternalDependencies for WhereClause {
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>> {
        let mut deps = self.left.external_dependencies();
        deps.extend(self.right.external_dependencies());
        deps
    }
}

impl ToTokens for WhereClause {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let left_tokens = self.left.to_token_stream();
        let right_tokens = self.right.to_token_stream();
        tokens.extend(quote::quote! { #left_tokens : #right_tokens });
    }
}

impl ToString for WhereClause {
    fn to_string(&self) -> String {
        self.to_token_stream().to_string()
    }
}
