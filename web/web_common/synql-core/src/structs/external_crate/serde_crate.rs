//! Submodule implementing the method `serde` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `serde` crate.

use std::sync::Arc;

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalTrait};

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `serde` crate.
    pub fn serde() -> Arc<ExternalCrate> {
        Arc::new(
            ExternalCrate::new()
                .name("serde".to_string())
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
    }
}
