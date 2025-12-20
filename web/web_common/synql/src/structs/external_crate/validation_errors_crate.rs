//! Submodule implementing the method `validation_errors` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `validation_errors` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::{
    structs::{DataVariantRef, ExternalCrate, TraitDef, ExternalType},
    utils::generic_type,
};

static VALIDATION_ERRORS_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `validation_errors` crate.
    pub fn validation_errors() -> Arc<ExternalCrate> {
        VALIDATION_ERRORS_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("validation_errors")
                        .unwrap()
                        .add_types([
                            Arc::new(
                                ExternalType::new()
                                    .rust_type(syn::parse_quote!(
                                        validation_errors::prelude::ValidationError
                                    ))
                                    .generic(generic_type("FieldName"))
                                    .build()
                                    .unwrap(),
                            ),
                            Arc::new(
                                ExternalType::new()
                                    .rust_type(syn::parse_quote!(
                                        validation_errors::prelude::Unspecified
                                    ))
                                    .supports_copy()
                                    .supports_default()
                                    .supports_hash()
                                    .supports_ord()
                                    .build()
                                    .unwrap(),
                            ),
                        ])
                        .unwrap()
                        .add_trait(
                            TraitDef::new()
                                .name("ReplaceFieldName")
                                .unwrap()
                                .path(syn::parse_quote!(
                                    validation_errors::prelude::ReplaceFieldName
                                ))
                                .build()
                                .unwrap(),
                        )
                        .unwrap()
                        .git(
                            "https://github.com/earth-metabolome-initiative/emi-monorepo",
                            "postgres-crate",
                        )
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }

    /// Returns the `ValidationError` parametrized with the `Unspecified` type.
    #[must_use]
    pub fn unspecified_validation_error_type() -> DataVariantRef {
        let validation_errors = Self::validation_errors();
        let unspecified = validation_errors
            .external_type(&syn::parse_quote!(validation_errors::prelude::Unspecified))
            .unwrap();
        let validation_error_type = validation_errors
            .external_type(&syn::parse_quote!(validation_errors::prelude::ValidationError))
            .unwrap();
        validation_error_type
            .set_generic_field(&generic_type("FieldName"), unspecified.into())
            .unwrap()
            .into()
    }
}
