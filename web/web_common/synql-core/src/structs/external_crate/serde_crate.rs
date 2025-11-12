//! Submodule implementing the method `serde` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `serde` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalTrait};

static SERDE_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `serde`
    /// crate.
    pub fn serde() -> Arc<ExternalCrate> {
        SERDE_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("serde")
                        .unwrap()
                        .version("1.0")
                        .features(["derive", "rc"])
                        .add_traits([
                            ExternalTrait::new()
                                .name("Serialize")
                                .unwrap()
                                .path(syn::parse_quote!(serde::Serialize))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("Deserialize")
                                .unwrap()
                                .path(syn::parse_quote!(serde::Deserialize))
                                .build()
                                .unwrap(),
                        ])
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}
