//! Submodule providing a struct which defines a struct model.

use syn::Data;

/// Struct defining a struct model.
pub struct Struct {
    /// Syn tree representing the struct name.
    data: Data,
}
