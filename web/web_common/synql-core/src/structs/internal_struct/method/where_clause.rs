//! Submodule defining a `WhereClause` struct.

mod builder;
pub use builder::{WhereClauseAttribute, WhereClauseBuilder};
use quote::ToTokens;

use crate::{
    structs::{ExternalCrate, InternalCrate, InternalToken},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a where clause in a method or type definition.
pub struct WhereClause<'data> {
    /// Left-hand side of the where clause.
    left: InternalToken<'data>,
    /// Right-hand side of the where clause.
    right: InternalToken<'data>,
}

impl<'data> WhereClause<'data> {
    /// Initializes a new `WhereClauseBuilder`.
    pub fn new() -> WhereClauseBuilder<'data> {
        WhereClauseBuilder::default()
    }
}

impl<'data> InternalDependencies<'data> for WhereClause<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut deps = self.left.internal_dependencies();
        deps.extend(self.right.internal_dependencies());
        deps
    }
}

impl<'data> ExternalDependencies<'data> for WhereClause<'data> {
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
        let mut deps = self.left.external_dependencies();
        deps.extend(self.right.external_dependencies());
        deps
    }
}

impl ToTokens for WhereClause<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let left_tokens = self.left.to_token_stream();
        let right_tokens = self.right.to_token_stream();
        tokens.extend(quote::quote! { #left_tokens : #right_tokens });
    }
}
