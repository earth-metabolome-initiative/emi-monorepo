#![doc = include_str!("../README.md")]

use std::str::FromStr;

#[cfg(feature = "diesel")]
pub mod diesel_impls;
#[cfg(feature = "pgrx")]
mod pgrx;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::expression::AsExpression, diesel::deserialize::FromSqlRow)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature="diesel", diesel(sql_type = crate::diesel_impls::Uuid))]
/// A wrapper around the `uuid` crate's `Uuid` type.
pub struct Uuid(uuid::Uuid);

impl Uuid {
    #[must_use]
    /// Creates a new `Uuid` using the `uuid` crate's `new_v4` method.
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl FromStr for Uuid {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::from_str(s)?))
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(uuid: Uuid) -> Self {
        uuid.0
    }
}

impl From<[u8; 16]> for Uuid {
    fn from(bytes: [u8; 16]) -> Self {
        Self(uuid::Uuid::from_bytes(bytes))
    }
}

impl From<Uuid> for [u8; 16] {
    fn from(uuid: Uuid) -> Self {
        *uuid.0.as_bytes()
    }
}

impl<'a> From<&'a [u8; 16]> for Uuid {
    fn from(bytes: &'a [u8; 16]) -> Self {
        Self(uuid::Uuid::from_bytes(*bytes))
    }
}

impl AsRef<uuid::Uuid> for Uuid {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl AsMut<uuid::Uuid> for Uuid {
    fn as_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.0
    }
}

impl AsRef<[u8; 16]> for Uuid {
    fn as_ref(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }
}

impl core::ops::Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::fmt::Display for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
