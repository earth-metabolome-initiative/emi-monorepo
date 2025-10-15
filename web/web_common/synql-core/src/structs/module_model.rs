//! Submodule defining a struct which represents a rust module.

use crate::structs::Publicness;

/// Struct representing a rust module.
pub struct Module {
    /// Name of the module.
    name: String,
    /// The submodules it contains.
    submodules: Vec<Module>,
    /// Publicness of the module.
    publicness: Publicness,
}
