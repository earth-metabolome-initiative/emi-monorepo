//! Submodule implementing the `TryFrom` trait for the `CAS` number struct.

use std::fmt::Display;

use crate::{CAS, utils::checksum};

impl TryFrom<&str> for CAS {
    type Error = crate::errors::Error;

    /// Parses a string into a `CAS` number.
    ///
    /// The input string must be in the format `XXXX-XX-X`, where:
    /// - `XXXX` is the first part of the CAS number,
    /// - `XX` is the second part of the CAS number,
    /// - `X` is the check digit.
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for CAS {
    type Error = crate::errors::Error;

    /// Parses a string into a `CAS` number.
    ///
    /// The input string must be in the format `XXXX-XX-X`, where:
    /// - `XXXX` is the first part of the CAS number,
    /// - `XX` is the second part of the CAS number,
    /// - `X` is the check digit.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
    }
}

impl TryFrom<&String> for CAS {
    type Error = crate::errors::Error;

    /// Parses a string into a `CAS` number.
    ///
    /// The input string must be in the format `XXXX-XX-X`, where:
    /// - `XXXX` is the first part of the CAS number,
    /// - `XX` is the second part of the CAS number,
    /// - `X` is the check digit.
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
    }
}

impl<A, B, C> TryFrom<(A, B, C)> for CAS
where
    A: TryInto<u32> + Copy + Display,
    B: TryInto<u8> + Copy + Display,
    C: TryInto<u8> + Copy + Display,
{
    type Error = crate::errors::Error;

    /// Parses a tuple into a `CAS` number.
    fn try_from(t: (A, B, C)) -> Result<Self, Self::Error> {
        let (a, b, c) = t;
        let first: u32 = a
            .try_into()
            .map_err(|_| crate::errors::Error::InvalidString(format!("{a}-{b}-{c}")))?;
        let second: u8 = b
            .try_into()
            .map_err(|_| crate::errors::Error::InvalidString(format!("{a}-{b}-{c}")))?;
        let third: u8 = c
            .try_into()
            .map_err(|_| crate::errors::Error::InvalidString(format!("{a}-{b}-{c}")))?;

        let cas = CAS((first * 1000) + (u32::from(second) * 10) + u32::from(third));

        let expected = checksum(cas);
        if cas.check_digit() != expected {
            return Err(crate::errors::Error::InvalidChecksum {
                cas: format!("{a}-{b}-{c}"),
                expected,
                actual: cas.check_digit(),
            });
        }

        Ok(cas)
    }
}

impl TryFrom<u32> for CAS {
    type Error = crate::errors::Error;

    /// Parses a `u32` into a `CAS` number.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let first = value / 1000;
        let second = (value % 1000) / 10;
        let third = value % 10;

        CAS::try_from((first, second, third))
    }
}
