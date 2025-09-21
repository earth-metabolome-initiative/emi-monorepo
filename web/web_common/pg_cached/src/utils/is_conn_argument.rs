//! Submodule providing a function to determine whether a function argument
//! is to be considered a diesel DB connection.

/// Returns `true` if the given type is a reference to a `PgConnection`.
/// Otherwise, returns `false`.
///
/// # Arguments
///
/// * `ty` - A reference to a `syn::Type` representing the type of the argument.
pub(super) fn is_conn_argument(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(syn::TypeReference { elem, .. }) = ty {
        if let syn::Type::Path(syn::TypePath { path, .. }) = &**elem {
            if let Some(segment) = path.segments.last() {
                return segment.ident == "PgConnection";
            }
        }
    }
    false
}
