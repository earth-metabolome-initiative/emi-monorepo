//! Submodule implementing the method `serde` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `serde` crate.

use common_traits::builder::Builder;
use lazy_static::lazy_static;

use crate::structs::{ExternalCrate, ExternalTrait};

lazy_static! {
    pub static ref SERDE_CRATE: ExternalCrate<'static> = ExternalCrate::new()
        .name("serde".to_string())
        .unwrap()
        .version("1.0")
        .features(["derive", "rc"])
        .unwrap()
        .add_traits([
            ExternalTrait::new()
                .name("Serialize")
                .unwrap()
                .path(syn::parse_str("serde::Serialize").unwrap())
                .build()
                .unwrap(),
            ExternalTrait::new()
                .name("Deserialize")
                .unwrap()
                .path(syn::parse_str("serde::Deserialize").unwrap())
                .build()
                .unwrap(),
        ])
        .unwrap()
        .build()
        .unwrap();
}

impl ExternalCrate<'_> {
    /// Initializes a `ExternalCrate` instance describing the `serde` crate.
    pub fn serde() -> &'static ExternalCrate<'static> {
        &SERDE_CRATE
    }
}
