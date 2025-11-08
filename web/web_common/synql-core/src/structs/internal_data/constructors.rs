//! Submodule providing constructor methods for internal data structures.

use crate::structs::{DataVariantRef, ExternalCrate};

impl DataVariantRef {
    /// Returns a new boolean data variant reference.
    pub fn bool() -> Self {
        let core_crate = ExternalCrate::core();
        let boolean_type = core_crate
            .external_type(&syn::parse_quote!(bool))
            .expect("Failed to find the bool type in core");
        Self::External(boolean_type)
    }

    /// Returns a new `&str` data variant reference.
    pub fn str() -> Self {
        let core_crate = ExternalCrate::std();
        let str_type = core_crate
            .external_type(&syn::parse_quote!(str))
            .expect("Failed to find the &str type in core");
        Self::External(str_type).reference(None)
    }

    /// Returns a new `Unit` data variant reference.
    pub fn unit() -> Self {
        let core_crate = ExternalCrate::core();
        let unit_type = core_crate
            .external_type(&syn::parse_quote!(()))
            .expect("Failed to find the unit type in core");
        Self::External(unit_type)
    }

    /// Returns a new `Unit` `Result` with the given error type.
    pub fn unit_result(error_type: DataVariantRef) -> Self {
        Self::result(Self::unit(), error_type)
    }
}
