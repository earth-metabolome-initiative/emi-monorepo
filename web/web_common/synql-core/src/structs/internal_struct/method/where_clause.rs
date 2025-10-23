//! Submodule defining a `WhereClause` struct.

use quote::ToTokens;

use crate::structs::InternalToken;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a where clause in a method or type definition.
pub struct WhereClause<'data> {
    /// Left-hand side of the where clause.
    left: InternalToken<'data>,
    /// Right-hand side of the where clause.
    right: InternalToken<'data>,
}

impl ToTokens for WhereClause<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let left_tokens = self.left.to_token_stream();
        let right_tokens = self.right.to_token_stream();
        tokens.extend(quote::quote! { #left_tokens : #right_tokens });
    }
}
