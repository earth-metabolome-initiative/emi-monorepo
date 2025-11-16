//! Submodule implementing the method `diesel_queries` for the [`ExternalCrate`]
//! struct which initializes a `ExternalCrate` instance describing the
//! `diesel-queries` crate.

use std::sync::{Arc, OnceLock};

use common_traits::builder::Builder;

use crate::{
    structs::{ExternalCrate, ExternalTrait},
    utils::generic_type,
};

static DIESEL_QUERIES_CRATE: OnceLock<Arc<ExternalCrate>> = OnceLock::new();

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `diesel-queries` crate.
    pub fn diesel_queries() -> Arc<ExternalCrate> {
        DIESEL_QUERIES_CRATE
            .get_or_init(|| {
                Arc::new(
                    ExternalCrate::new()
                        .name("diesel-queries")
                        .unwrap()
                        .add_traits([
                            ExternalTrait::new()
                                .name("Read")
                                .unwrap()
                                .path(syn::parse_quote!(diesel_queries::prelude::Read))
                                .generic(generic_type("C"))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("ExtensionOf")
                                .unwrap()
                                .path(syn::parse_quote!(diesel_queries::prelude::ExtensionOf))
                                .generic(generic_type("Extended"))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("TableIsExtensionOf")
                                .unwrap()
                                .path(syn::parse_quote!(
                                    diesel_queries::prelude::TableIsExtensionOf
                                ))
                                .generic(generic_type("Extended"))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("VerticalSameAs")
                                .unwrap()
                                .path(syn::parse_quote!(diesel_queries::prelude::VerticalSameAs))
                                .generic(generic_type("Referenced"))
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("HorizontalSameAs")
                                .unwrap()
                                .path(syn::parse_quote!(diesel_queries::prelude::HorizontalSameAs))
                                .generics([generic_type("Referenced"), generic_type("Key")])
                                .build()
                                .unwrap(),
                            ExternalTrait::new()
                                .name("Ancestor")
                                .unwrap()
                                .path(syn::parse_quote!(diesel_queries::prelude::Ancestor))
                                .generics([generic_type("Extended"), generic_type("C")])
                                .build()
                                .unwrap(),
                        ])
                        .unwrap()
                        .git(
                            "https://github.com/earth-metabolome-initiative/emi-monorepo",
                            "postgres-crate",
                        )
                        .build()
                        .unwrap(),
                )
            })
            .clone()
    }
}
