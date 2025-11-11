//! Submodule implementing the method `diesel` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel` crate.

use std::sync::Arc;

use common_traits::builder::Builder;

use crate::{
    structs::{ExternalCrate, ExternalTrait},
    utils::generic_type,
};

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `diesel-queries`
    /// crate.
    pub fn diesel_queries() -> Arc<ExternalCrate> {
        Arc::new(
            ExternalCrate::new()
                .name("diesel-queries".to_string())
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
    }
}
