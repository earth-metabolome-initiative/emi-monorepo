//! Submodule implementing the method `diesel` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel` crate.

use crate::structs::{ExternalCrate, ExternalType};

/// Enum to specify the maximal number of columns flags to
/// be enabled in the `diesel` crate. Since the compile times
/// increase precipitously with the number of columns, it is
/// advisable to keep this number as low as possible.
pub enum MaximalNumberOfColumns {
    /// Up to 16 columns support.
    Columns16,
    /// Up to 32 columns support.
    Columns32,
    /// Up to 64 columns support.
    Columns64,
    /// Up to 128 columns support.
    Columns128,
}

impl TryFrom<usize> for MaximalNumberOfColumns {
    type Error = crate::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0..=16 => Ok(MaximalNumberOfColumns::Columns16),
            17..=32 => Ok(MaximalNumberOfColumns::Columns32),
            33..=64 => Ok(MaximalNumberOfColumns::Columns64),
            65..=128 => Ok(MaximalNumberOfColumns::Columns128),
            _ => Err(crate::Error::TooManyColumns(value)),
        }
    }
}

impl MaximalNumberOfColumns {
    /// Returns the feature string corresponding to the maximal number of
    /// columns.
    fn as_feature_str(&self) -> Option<&str> {
        match self {
            MaximalNumberOfColumns::Columns16 => None,
            MaximalNumberOfColumns::Columns32 => Some("32-column-tables"),
            MaximalNumberOfColumns::Columns64 => Some("64-column-tables"),
            MaximalNumberOfColumns::Columns128 => Some("128-column-tables"),
        }
    }
}

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the `diesel`
    /// crate.
    pub fn diesel(number_of_columns: MaximalNumberOfColumns) -> ExternalCrate {
        ExternalCrate::new("diesel")
            .unwrap()
            .feature("extras")
            .features(number_of_columns.as_feature_str())
            .git("https://github.com/LucaCappelletti94/diesel", "future3")
            .types([
                ExternalType::new(
                    syn::parse_quote!(::diesel::sql_types::Interval),
                    syn::parse_quote!(::diesel::pg::data_types::PgInterval),
                )
                .postgres_type("interval")
                .unwrap()
                .supports_copy()
                .supports_eq()
                .into(),
                ExternalType::new(
                    syn::parse_quote!(::diesel::result::Error),
                    syn::parse_quote!(::diesel::result::Error),
                )
                .into(),
            ])
            .unwrap()
            .into()
    }
}
