//! Submodule implementing the method `serde_json` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `serde_json` crate.

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `serde_json` crate.
    pub fn serde_json() -> ExternalCrate {
        ExternalCrate::new("serde_json")
            .unwrap()
            .version("1.0")
            .types([
                ExternalType::new(
                    syn::parse_quote!(::diesel::sql_types::Json),
                    syn::parse_quote!(::serde_json::Value),
                )
                .postgres_types(["json"])
                .unwrap()
                .supports_clone()
                .supports_eq()
                .into(),
                ExternalType::new(
                    syn::parse_quote!(::diesel::sql_types::Jsonb),
                    syn::parse_quote!(::serde_json::Value),
                )
                .postgres_types(["jsonb"])
                .unwrap()
                .supports_clone()
                .supports_eq()
                .into(),
            ])
            .unwrap()
            .into()
    }
}
