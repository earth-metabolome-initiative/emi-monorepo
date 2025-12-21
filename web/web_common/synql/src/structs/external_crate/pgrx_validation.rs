//! Submodule implementing the method `pgrx_validation` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `pgrx_validation` crate.

use crate::structs::{ExternalCrate, ExternalFunction};

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `pgrx_validation` crate.
    pub fn pgrx_validation() -> ExternalCrate {
        ExternalCrate::new("pgrx_validation")
            .unwrap()
            .functions([
                ExternalFunction::new(
                    "must_be_font_awesome_class",
                    syn::parse_quote!(pgrx_validation::must_be_font_awesome_class),
                ),
                ExternalFunction::new(
                    "must_be_paragraph",
                    syn::parse_quote!(pgrx_validation::must_be_paragraph),
                ),
                ExternalFunction::new(
                    "must_be_email",
                    syn::parse_quote!(pgrx_validation::must_be_email),
                ),
            ])
            .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
            .into()
    }
}
