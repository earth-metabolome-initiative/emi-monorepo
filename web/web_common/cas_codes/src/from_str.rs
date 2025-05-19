//! Submodule implementing the `FromStr` trait for the `CAS` number struct.

use std::str::FromStr;

use crate::CAS;

impl FromStr for CAS {
    type Err = crate::errors::Error;

    /// Parses a string into a `CAS` number.
    ///
    /// The input string must be in the format `XXXX-XX-X`, where:
    /// - `XXXX` is the first part of the CAS number,
    /// - `XX` is the second part of the CAS number,
    /// - `X` is the check digit.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iterator = if s.contains('-') {
            s.split('-')
        } else if s.contains('\u{2013}') {
            s.split('\u{2013}')
        } else {
            return Err(crate::errors::Error::InvalidString(s.to_string()));
        };

        let Some(first) = iterator.next() else {
            return Err(crate::errors::Error::InvalidString(s.to_string()));
        };

        let Some(second) = iterator.next() else {
            return Err(crate::errors::Error::InvalidString(s.to_string()));
        };

        let Some(third) = iterator.next() else {
            return Err(crate::errors::Error::InvalidString(s.to_string()));
        };

        if iterator.next().is_some() {
            return Err(crate::errors::Error::InvalidString(s.to_string()));
        }

        let first =
            first.parse::<u32>().map_err(|_| crate::errors::Error::InvalidString(s.to_string()))?;
        let second =
            second.parse::<u8>().map_err(|_| crate::errors::Error::InvalidString(s.to_string()))?;
        let third =
            third.parse::<u8>().map_err(|_| crate::errors::Error::InvalidString(s.to_string()))?;

        CAS::try_from((first, second, third))
    }
}
