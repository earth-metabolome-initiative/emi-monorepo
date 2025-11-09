//! Submodule implementing the method `core` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `core` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;
use strum::IntoEnumIterator;

use crate::structs::{ExternalCrate, ExternalTrait, ExternalType, Trait};
mod numeric;

static CORE_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `core` crate.
    pub fn core() -> Arc<ExternalCrate> {
        CORE_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("core".to_string())
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
                        .add_traits([ExternalTrait::new()
                            .name("Display")
                            .unwrap()
                            .path(syn::parse_quote!(core::fmt::Display))
                            .build()
                            .unwrap()])
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}
