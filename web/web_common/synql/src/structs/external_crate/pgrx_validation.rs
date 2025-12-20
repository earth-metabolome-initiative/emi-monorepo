//! Submodule implementing the method `pgrx_validation` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `pgrx_validation` crate.

use crate::structs::ExternalCrate;

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `pgrx_validation` crate.
    pub fn pgrx_validation() -> ExternalCrate {
        ExternalCrate::new("pgrx_validation")
            .unwrap()
            .functions([
                (
                    Self::str_method("must_be_font_awesome_class"),
                    syn::parse_quote!(pgrx_validation::must_be_font_awesome_class),
                ),
                (
                    Self::str_method("must_be_paragraph"),
                    syn::parse_quote!(pgrx_validation::must_be_paragraph),
                ),
                (
                    Self::str_method("must_be_email"),
                    syn::parse_quote!(pgrx_validation::must_be_email),
                ),
            ])
            .git(
                "https://github.com/earth-metabolome-initiative/emi-monorepo",
                "postgres-crate",
            )
            .into()
    }
}
