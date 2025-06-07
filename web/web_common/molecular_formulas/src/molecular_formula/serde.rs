#![cfg(feature = "serde")]
//! Submodule implementing serialization and deserialization for the
//! `MolecularFormula` struct using serialization and deserialization traits
//! with `String` as the underlying type.
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::MolecularFormula;
impl Serialize for MolecularFormula {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MolecularFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        MolecularFormula::from_str(&s).map_err(serde::de::Error::custom)
    }
}
