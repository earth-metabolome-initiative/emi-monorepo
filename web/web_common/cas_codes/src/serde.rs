#![cfg(feature = "serde")]
//! Submodule implementing the `serde::Serialize` and `serde::Deserialize`
//! traits for the `CAS` number struct.

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::CAS;

impl Serialize for CAS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for CAS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<CAS>().map_err(D::Error::custom)
    }
}
