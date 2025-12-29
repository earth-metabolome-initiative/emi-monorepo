//! Submodule implementing the method `diesel_builders` for the
//! [`ExternalCrate`] struct which initializes a `ExternalCrate` instance
//! describing the `diesel-builders` crate.

use crate::structs::{ExternalCrate, external_crate::MaximalNumberOfColumns};

impl MaximalNumberOfColumns {
    /// Returns the feature string corresponding to the maximal number of
    /// columns.
    fn as_diesel_builders_feature(&self) -> Option<&'static str> {
        match self {
            Self::Columns16 | Self::Columns32 => None,
            Self::Columns64 => Some("size-64"),
            Self::Columns128 => Some("size-128"),
        }
    }
}

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `diesel-builders` crate.
    pub fn diesel_builders(number_of_columns: MaximalNumberOfColumns) -> ExternalCrate {
        ExternalCrate::new("diesel-builders")
            .unwrap()
            .features(number_of_columns.as_diesel_builders_feature())
            .git("https://github.com/LucaCappelletti94/diesel-builders", "main")
            .into()
    }
}
