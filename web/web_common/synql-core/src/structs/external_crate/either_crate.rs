//! Submodule implementing the method `either` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `either` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::{
    structs::{DataVariantRef, ExternalCrate, ExternalType},
    utils::generic_type,
};

static EITHER_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `either`
    /// crate.
    pub fn either() -> Arc<ExternalCrate> {
        EITHER_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("either")
                        .unwrap()
                        .version("1")
                        .features(["serde"])
                        .add_type(Arc::new(
                            ExternalType::new()
                                .rust_type(syn::parse_quote!(either::Either))
                                .supports_clone()
                                .supports_debug()
                                .supports_eq()
                                .supports_hash()
                                .supports_serde()
                                .generics([generic_type("L"), generic_type("R")])
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

    /// Returns an `Either` `ExternalType` instance parametrized with the
    /// provided left and right types.
    pub fn either_of(left: DataVariantRef, right: DataVariantRef) -> DataVariantRef {
        let either_crate = Self::either();
        let either_type = either_crate.external_type(&syn::parse_quote!(either::Either)).unwrap();
        either_type
            .set_generic_field(&generic_type("L"), left)
            .unwrap()
            .set_generic_field(&generic_type("R"), right)
            .unwrap()
            .into()
    }
}
