//! Submodule implementing the method `pgrx_validation` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `pgrx_validation` crate.

use std::sync::Arc;

use common_traits::builder::Builder;

use crate::structs::{Argument, DataVariantRef, Documentation, ExternalCrate, Method};

impl ExternalCrate {
    /// Standard return type for validation functions.
    fn validation_result_type() -> DataVariantRef {
        DataVariantRef::unit_result(Self::unspecified_validation_error_type())
    }

    fn str_method(name: &str) -> Arc<Method> {
        Arc::new(
            Method::new()
                .name(name)
                .unwrap()
                .documentation(Documentation::default())
                .public()
                .error_documentation(Documentation::default())
                .argument(
                    Argument::new()
                        .name("value")
                        .unwrap()
                        .documentation(Documentation::default())
                        .arg_type(DataVariantRef::str())
                        .build()
                        .unwrap(),
                )
                .unwrap()
                .return_type(Self::validation_result_type())
                .build()
                .unwrap(),
        )
    }

    /// Initializes a `ExternalCrate` instance describing the
    /// `pgrx_validation` crate.
    pub fn pgrx_validation() -> Arc<ExternalCrate> {
        Arc::new(
            ExternalCrate::new()
                .name("pgrx_validation".to_string())
                .unwrap()
                .add_functions([
                    (
                        Self::str_method("must_be_font_awesome_class"),
                        Arc::new(syn::parse_quote!(pgrx_validation::must_be_font_awesome_class)),
                    ),
                    (
                        Self::str_method("must_be_paragraph"),
                        Arc::new(syn::parse_quote!(pgrx_validation::must_be_paragraph)),
                    ),
                    (
                        Self::str_method("must_be_email"),
                        Arc::new(syn::parse_quote!(pgrx_validation::must_be_email)),
                    ),
                ])
                .git(
                    "https://github.com/earth-metabolome-initiative/emi-monorepo",
                    "postgres-crate",
                )
                .build()
                .unwrap(),
        )
    }
}
