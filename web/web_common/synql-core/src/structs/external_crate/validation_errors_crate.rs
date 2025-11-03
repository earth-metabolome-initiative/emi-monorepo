//! Submodule implementing the method `validation_errors` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `validation_errors` crate.

use common_traits::builder::Builder;
use lazy_static::lazy_static;

use crate::structs::{ExternalCrate, ExternalType};

lazy_static! {
    pub static ref VALIDATION_ERRORS_CRATE: ExternalCrate<'static> = ExternalCrate::new()
        .name("validation_errors".to_string())
        .unwrap()
        .add_type(
            ExternalType::new()
                .rust_type(syn::parse_quote!(validation_errors::ValidationError))
                .build()
                .unwrap(),
        )
        .unwrap()
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .build()
        .unwrap();
}

impl ExternalCrate<'_> {
    /// Initializes a `ExternalCrate` instance describing the
    /// `validation_errors` crate.
    pub fn validation_errors() -> &'static ExternalCrate<'static> {
        &VALIDATION_ERRORS_CRATE
    }
}
