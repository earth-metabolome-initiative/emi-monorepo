//! Submodule implementing the method `core` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `core` crate.

use common_traits::builder::Builder;
use strum::IntoEnumIterator;

use crate::structs::{ExternalCrate, ExternalTrait, Trait};
mod numeric;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CORE_CRATE: ExternalCrate<'static> = ExternalCrate::new()
        .name("core".to_string())
        .unwrap()
        .add_types(numeric::all_types())
        .unwrap()
        .add_traits(Trait::iter().map(|t| t.into()))
        .unwrap()
        .add_traits([ExternalTrait::new()
            .name("Display")
            .unwrap()
            .path(syn::parse_quote!(core::fmt::Display))
            .build()
            .unwrap()])
        .unwrap()
        .build()
        .unwrap();
}

impl ExternalCrate<'_> {
    /// Initializes a `ExternalCrate` instance describing the `core` crate.
    pub fn core() -> &'static ExternalCrate<'static> {
        &CORE_CRATE
    }
}
