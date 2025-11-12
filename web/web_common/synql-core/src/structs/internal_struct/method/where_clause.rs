//! Submodule defining a `WhereClause` struct.

mod builder;

pub use builder::WhereClauseBuilder;
use quote::ToTokens;

use crate::{
    structs::{ExternalCrate, InternalCrate, InternalToken},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    /// Returns a reference to the left-hand side of the where clause.
    pub fn left(&self) -> &InternalToken {
        &self.left
    }

    /// Returns a reference to the right-hand side of the where clause.
    pub fn right(&self) -> &InternalToken {
        &self.right
    }
}

impl InternalDependencies for WhereClause {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.left.internal_dependencies().chain(self.right.internal_dependencies())
    }
}

impl ExternalDependencies for WhereClause {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.left.external_dependencies().chain(self.right.external_dependencies())
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
