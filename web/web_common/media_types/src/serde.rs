#![cfg(feature = "serde")]
//! Submodule implementing the `serde` serialization and deserialization
//! for the `MediaType` struct, by using the `ToString` and `FromStr`
//! traits.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::MediaType;

/// Serializes a `MediaType` instance into a string representation.
impl Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Deserializes a string representation into a `MediaType` instance.
impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
