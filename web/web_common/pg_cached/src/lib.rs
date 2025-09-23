#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ItemFn, parse_macro_input};

use crate::utils::first_non_conn_argument_ident;
use crate::utils::function_hash;
use crate::utils::non_conn_arguments_idents;

mod utils;

/// Struct to hold arguments for the auto_cached macro.
struct AutoCachedArgs {
    /// The type of the cache key (e.g., "u32" or "u64").
    key: String,
    /// The expression used to convert the function arguments into the cache key.
    convert: String,
}

impl AutoCachedArgs {
    /// Creates an `AutoCachedArgs` instance for the `oid_auto_cached` macro.
    /// It uses the `oid` field of the first non-connection argument as the cache key.
    ///
    /// # Arguments
    /// * `item` - A reference to the `ItemFn` representing the function.
    ///
    /// # Errors
    /// Returns a `syn::Error` if there are no non-connection arguments.
    fn oid(item: &ItemFn) -> Result<Self, syn::Error> {
        let Some(first_non_conn_arg) = first_non_conn_argument_ident(&item.sig.inputs) else {
            return Err(syn::Error::new(
                item.sig.inputs.span(),
                "Expected at least one argument other than the connection",
            ));
        };
        Ok(Self {
            key: "u32".to_string(),
            convert: quote! { {
                use crate::traits::HasOid;
                #first_non_conn_arg.oid()
            } }.to_string(),
        })
    }

    /// Creates an `AutoCachedArgs` instance for the `auto_cached` macro.
    /// It hashes all non-connection arguments to create a unique cache key.
    ///
    /// # Arguments
    /// * `item` - A reference to the `ItemFn` representing the function.
    ///
    /// # Errors
    /// Returns a `syn::Error` if there are no non-connection arguments.
    fn hash_based(item: &ItemFn) -> Result<Self, syn::Error> {
        let arguments = non_conn_arguments_idents(&item.sig.inputs);

        if arguments.is_empty() {
            return Err(syn::Error::new(
                item.sig.inputs.span(),
                "Expected at least one argument other than the connection",
            ));
        }

        Ok(Self {
            key: "u64".to_string(),
            convert: quote! {
                {
                    use std::hash::{Hasher, Hash};
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    #(Hash::hash(&#arguments, &mut hasher);)*
                    hasher.finish()
                }
            }
            .to_string(),
        })
    }
}

fn auto_cached_dispatch(args: AutoCachedArgs, item: ItemFn) -> TokenStream {
    let key = args.key;
    let convert = args.convert;

    // Branch based on enabled features
    #[cfg(feature = "io_cached")]
    {
        let function_hash = function_hash(&item);
        let create = format!(
            r##" {{
            let dir = std::env::var("PG_CACHE_DIR")
                .unwrap_or_else(|_| "cache".to_string());
            cached::DiskCache::new("{function_hash}")
                .set_disk_directory(&dir)
                .build()
                .expect("error building disk cache")
        }} "##
        );
        let map_error = quote! {
            |e| {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::SerializationFailure,
                    Box::new(format!("DiskCache error: {}", e))
                )
            }
        }.to_string();
        let decorator = quote! {
            #[cached::proc_macro::io_cached(
                disk = true,
                sync_to_disk_on_cache_change = true,
                create = #create,
                map_error = #map_error,
                key = #key,
                convert = #convert
            )]
            #item
        };
        return decorator.into();
    }

    #[cfg(all(feature = "cached", not(feature = "io_cached")))]
    {
        let decorator = quote! {
            #[cached::proc_macro::cached(
                key = #key,
                convert = #convert
            )]
            #item
        };
        return decorator.into();
    }

    #[cfg(not(feature = "cached"))]
    {
        // passthrough if no caching
        return quote!(#item).into();
    }
}

#[proc_macro_attribute]
/// An attribute macro that applies caching to a function based on its arguments.
/// This macro uses the `io_cached` crate for caching if the `io_cached` feature is enabled,
/// otherwise it falls back to the `cached` crate if the `cached` feature is enabled.
/// If neither feature is enabled, the macro leaves the function unchanged.
/// The cache key is derived from hashing all arguments that are not a database connection.
///
/// # Arguments
/// * `attr` - The attribute arguments (not used in this macro).
/// * `item` - The function to which the macro is applied.
///
/// # Returns
/// A `TokenStream` representing the modified function with caching applied, or the original
/// function if no caching features are enabled.
///
/// # Errors
/// If the function does not have at least one argument other than the connection,
/// a compile-time error is returned.
pub fn auto_cached(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let args = match AutoCachedArgs::hash_based(&item) {
        Ok(args) => args,
        Err(error) => return error.to_compile_error().into(),
    };
    auto_cached_dispatch(args, item)
}

#[proc_macro_attribute]
/// An attribute macro that applies caching to a function based on its first argument's `oid` field.
/// This macro uses the `io_cached` crate for caching if the `io_cached` feature is enabled,
/// otherwise it falls back to the `cached` crate if the `cached` feature is enabled.
/// If neither feature is enabled, the macro leaves the function unchanged.
/// The cache key is derived from the `oid` field of the first argument that is not a database
/// connection.
///
/// # Arguments
///
/// * `attr` - The attribute arguments (not used in this macro).
/// * `item` - The function to which the macro is applied.
///
/// # Returns
/// A `TokenStream` representing the modified function with caching applied, or the original
/// function if no caching features are enabled.
///
/// # Errors
/// If the function does not have at least one argument other than the connection,
/// or if the first non-connection argument does not have an `oid` field,
/// a compile-time error is returned.
pub fn oid_auto_cached(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let args = match AutoCachedArgs::oid(&item) {
        Ok(args) => args,
        Err(error) => return error.to_compile_error().into(),
    };
    auto_cached_dispatch(args, item)
}
