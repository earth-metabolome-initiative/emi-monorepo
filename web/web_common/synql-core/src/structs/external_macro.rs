//! Submodule providing a struct defining the macros available in an external
//! crate.

mod builder;
use std::{fmt::Debug, hash::Hash};

use builder::ExternalMacroBuilder;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining a macro available in an external crate.
pub struct ExternalMacro {
    /// The name of the macro.
    name: String,
}

unsafe impl Send for ExternalMacro {}
unsafe impl Sync for ExternalMacro {}

impl ExternalMacro {
    /// Inizializes a new `ExternalMacroBuilder`.
    pub fn new() -> ExternalMacroBuilder {
        ExternalMacroBuilder::default()
    }

    /// Returns the name of the macro.
    pub fn name(&self) -> &str {
        &self.name
    }
}
