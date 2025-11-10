//! Submodule implementing the method `validation_errors` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `validation_errors` crate.

use std::sync::Arc;

use common_traits::builder::Builder;
use syn::Ident;

use crate::structs::{DataVariantRef, ExternalCrate, ExternalTrait, ExternalType};

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the
    /// `validation_errors` crate.
    pub fn validation_errors() -> Arc<ExternalCrate> {
        Arc::new(
            ExternalCrate::new()
                .name("validation_errors".to_string())
                .unwrap()
                .add_types([
                    Arc::new(
                        ExternalType::new()
                            .rust_type(syn::parse_quote!(
                                validation_errors::prelude::ValidationError
                            ))
                            .generic(Ident::new("FieldName", proc_macro2::Span::call_site()))
                            .build()
                            .unwrap(),
                    ),
                    Arc::new(
                        ExternalType::new()
                            .rust_type(syn::parse_quote!(validation_errors::prelude::Unspecified))
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
                    ExternalTrait::new()
                        .name("ReplaceFieldName")
                        .unwrap()
                        .path(syn::parse_quote!(validation_errors::prelude::ReplaceFieldName))
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
    }

    /// Returns the `ValidationError` parametrized with the `Unspecified` type.
    pub fn unspecified_validation_error_type() -> DataVariantRef {
        let validation_errors = Self::validation_errors();
        let unspecified = validation_errors
            .external_type(&syn::parse_quote!(validation_errors::prelude::Unspecified))
            .unwrap();
        let validation_error_type = validation_errors
            .external_type(&syn::parse_quote!(validation_errors::prelude::ValidationError))
            .unwrap();
        validation_error_type
            .set_generic_field(
                &Ident::new("FieldName", proc_macro2::Span::call_site()),
                unspecified.into(),
            )
            .unwrap()
            .into()
    }
}
