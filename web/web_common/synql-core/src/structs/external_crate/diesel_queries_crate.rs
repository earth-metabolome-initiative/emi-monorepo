//! Submodule implementing the method `diesel` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel` crate.

use common_traits::builder::Builder;
use lazy_static::lazy_static;

use crate::structs::{ExternalCrate, ExternalTrait};

lazy_static! {
    pub static ref DIESEL_QUERIES_CRATE: ExternalCrate<'static> = ExternalCrate::new()
        .name("diesel-queries".to_string())
        .unwrap()
        .version("0.1.0")
        .add_traits([ExternalTrait::new()
            .name("Read")
            .unwrap()
            .path(syn::parse_quote!(diesel_queries::prelude::Read))
            .build()
            .unwrap(),])
        .unwrap()
        .build()
        .unwrap();
}

impl ExternalCrate<'_> {
    /// Initializes a `ExternalCrate` instance describing the `diesel-queries`
    /// crate.
    pub fn diesel_queries() -> &'static ExternalCrate<'static> {
        &DIESEL_QUERIES_CRATE
    }
}
