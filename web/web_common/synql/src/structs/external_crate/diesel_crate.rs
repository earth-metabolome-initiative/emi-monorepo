//! Submodule implementing the method `diesel` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel` crate.

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `diesel`
    /// crate.
    pub fn diesel() -> ExternalCrate {
        ExternalCrate::new("diesel")
            .unwrap()
            .git("https://github.com/diesel-rs/diesel", "main")
            .types([
                ExternalType::new(
                    syn::parse_quote!(diesel::sql_types::Interval),
                    syn::parse_quote!(diesel::pg::data_types::PgInterval),
                )
                .postgres_type("interval")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .into(),
                ExternalType::new(
                    syn::parse_quote!(diesel::result::Error),
                    syn::parse_quote!(diesel::result::Error),
                )
                .into(),
            ])
            .unwrap()
            .into()
    }
}
