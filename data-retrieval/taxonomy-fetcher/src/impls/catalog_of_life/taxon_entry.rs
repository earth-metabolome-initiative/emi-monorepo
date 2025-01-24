//! Submodule providing the implementation of the Taxon entry trait for the Catalog of Life.

use std::{fmt::Display, str::FromStr};

use serde::Serialize;

use crate::{impls::generic::taxon_entry::GenericTaxonEntry, TaxonIdentifier};

use super::rank::CatalogOfLifeRank;

#[derive(Debug, Eq, Clone, Copy, PartialEq, Hash)]
/// Represents in an efficient manner IDs such as:
///
/// - Four characters: C9FM
/// - Five characters: 3CP83
pub enum COLId {
	/// One character.
	One(u8),
	/// Two characters.
	Two([u8; 2]),
	/// Three characters.
	Three([u8; 3]),
    /// Four characters.
    Four([u8; 4]),
    /// Five characters.
    Five([u8; 5]),
}

impl TaxonIdentifier for COLId {}

impl Display for COLId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			COLId::One(char) => write!(f, "{}", char),
			COLId::Two(chars) => write!(f, "{}", std::str::from_utf8(chars).unwrap()),
			COLId::Three(chars) => write!(f, "{}", std::str::from_utf8(chars).unwrap()),
			COLId::Four(chars) => write!(f, "{}", std::str::from_utf8(chars).unwrap()),
			COLId::Five(chars) => write!(f, "{}", std::str::from_utf8(chars).unwrap()),
		}
	}
}

impl FromStr for COLId {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.len() {
			1 => Ok(COLId::One(s.as_bytes()[0])),
			2 => {
				let mut chars = [0; 2];
				chars.copy_from_slice(&s.as_bytes()[..2]);
				Ok(COLId::Two(chars))
			}
			3 => {
				let mut chars = [0; 3];
				chars.copy_from_slice(&s.as_bytes()[..3]);
				Ok(COLId::Three(chars))
			}
			4 => {
				let mut chars = [0; 4];
				chars.copy_from_slice(&s.as_bytes()[..4]);
				Ok(COLId::Four(chars))
			}
			5 => {
				let mut chars = [0; 5];
				chars.copy_from_slice(&s.as_bytes()[..5]);
				Ok(COLId::Five(chars))
			}
			length => Err(format!("Invalid length for COLId: {}", length)),
		}
	}
}

impl<'de> serde::Deserialize<'de> for COLId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
		COLId::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for COLId {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

/// A taxon entry for the Catalog of Life taxonomy.
pub type CatalogOfLifeTaxonEntry = GenericTaxonEntry<COLId, CatalogOfLifeRank>;
