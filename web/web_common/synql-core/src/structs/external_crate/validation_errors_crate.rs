//! Submodule implementing the method `validation_errors` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `validation_errors` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;
use syn::Ident;

use crate::structs::{ExternalCrate, ExternalType};

static VALIDATION_ERRORS_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the
    /// `validation_errors` crate.
    pub fn validation_errors() -> Arc<ExternalCrate> {
        VALIDATION_ERRORS_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("validation_errors".to_string())
                        .unwrap()
                        .add_type(Arc::new(
                            ExternalType::new()
                                .rust_type(syn::parse_quote!(
                                    validation_errors::prelude::ValidationError
                                ))
                                .generic(Ident::new("FieldName", proc_macro2::Span::call_site()))
                                .build()
                                .unwrap(),
                        ))
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
}
