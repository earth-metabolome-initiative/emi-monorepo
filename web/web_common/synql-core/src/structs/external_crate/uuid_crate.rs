//! Submodule implementing the method `uuid` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `uuid` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalType};

static UUID_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `uuid` crate.
    pub fn uuid() -> Arc<ExternalCrate> {
        UUID_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("uuid".to_string())
                        .unwrap()
                        .version("1.0")
                        .features(["v4", "serde"])
                        .unwrap()
                        .add_type(Arc::new(
                            ExternalType::new()
                                .diesel_type(syn::parse_quote!(diesel::sql_types::Uuid))
                                .rust_type(syn::parse_quote!(uuid::Uuid))
                                .postgres_type("uuid")
                                .unwrap()
                                .supports_copy()
                                .supports_eq()
                                .supports_serde()
                                .unwrap()
                                .build()
                                .unwrap(),
                        ))
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}
