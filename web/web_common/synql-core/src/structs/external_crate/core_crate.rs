//! Submodule implementing the method `core` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `core` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;
use strum::IntoEnumIterator;

use crate::{
    structs::{ExternalCrate, ExternalTrait, ExternalType, Trait},
    utils::generic_type,
};
mod numeric;

static CORE_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `core` crate.
    pub fn core() -> Arc<ExternalCrate> {
        CORE_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("core")
                        .unwrap()
                        .add_types(numeric::all_types())
                        .unwrap()
                        .add_type(Arc::new(
                            ExternalType::new()
                                .rust_type(syn::parse_quote!(()))
                                .supports_copy()
                                .supports_default()
                                .supports_hash()
                                .supports_ord()
                                .supports_serde()
                                .build()
                                .unwrap(),
                        ))
                        .unwrap()
                        .add_traits(Trait::iter().map(|t| t.into()))
                        .unwrap()
                        .add_traits([
                            ExternalTrait::new()
                                .name("Display")
                                .unwrap()
                                .path(syn::parse_quote!(core::fmt::Display))
                                .build()
                                .unwrap(),
                            ExternalTrait::from_trait(),
                        ])
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}

impl ExternalTrait {
    fn from_trait() -> Self {
        ExternalTrait::new()
            .name("From")
            .unwrap()
            .path(syn::parse_quote!(core::convert::From))
            .generic(generic_type("T"))
            .build()
            .unwrap()
    }
}
