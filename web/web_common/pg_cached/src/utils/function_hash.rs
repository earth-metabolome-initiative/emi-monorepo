//! Returns an hash of a function based on its signature and attributes.

use quote::ToTokens;
use std::hash::Hasher;

/// Returns a u64 hash of the given function's signature and body.
///
/// # Arguments
///
/// * `item` - A reference to a `syn::ItemFn` representing the function to hash.
pub(crate) fn function_hash(item: &syn::ItemFn) -> u64 {
    let mut tokens = proc_macro2::TokenStream::new();
    item.sig.to_tokens(&mut tokens);
    item.block.to_tokens(&mut tokens);
    let input_string = tokens.to_string();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&input_string, &mut hasher);
    hasher.finish()
}
