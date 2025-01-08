//! Submodule defining the rank enumeration for Open Tree of Life taxonomy.

use std::str::FromStr;

use serde::de::Deserialize;

use crate::{errors::TaxonEntryBuilderError, traits::Rank};

use super::taxon_entry::OpenTreeOfLifeTaxonEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Enumeration of the ranks used in the Open Tree of Life taxonomy.
pub enum OpenTreeOfLifeRank {
    /// No rank.
    NoRank,
    /// No Rank - Terminal
    NoRankTerminal,
    /// Domain rank.
    Domain,
    /// Kingdom rank.
    Kingdom,
    /// Phylum rank.
    Phylum,
    /// Class rank.
    Class,
    /// Order rank.
    Order,
    /// Family rank.
    Family,
    /// Subfamily rank.
    Subfamily,
    /// Genus rank.
    Genus,
    /// Species group rank.
    SpeciesGroup,
    /// Species sub group rank.
    SpeciesSubGroup,
    /// Species rank.
    Species,
    /// Subspecies rank.
    Subspecies,
}

impl OpenTreeOfLifeRank {
    /// Returns true if the rank is a no-rank terminal.
    pub fn is_no_rank_terminal(&self) -> bool {
        matches!(self, OpenTreeOfLifeRank::NoRankTerminal)
    }
}

impl Rank for OpenTreeOfLifeRank {
}

impl std::fmt::Display for OpenTreeOfLifeRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for OpenTreeOfLifeRank {
    type Err = TaxonEntryBuilderError<OpenTreeOfLifeTaxonEntry>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no rank" => Ok(OpenTreeOfLifeRank::NoRank),
            "no rank - terminal" => Ok(OpenTreeOfLifeRank::NoRankTerminal),
            "domain" => Ok(OpenTreeOfLifeRank::Domain),
            "kingdom" => Ok(OpenTreeOfLifeRank::Kingdom),
            "phylum" => Ok(OpenTreeOfLifeRank::Phylum),
            "class" => Ok(OpenTreeOfLifeRank::Class),
            "order" => Ok(OpenTreeOfLifeRank::Order),
            "family" => Ok(OpenTreeOfLifeRank::Family),
            "subfamily" => Ok(OpenTreeOfLifeRank::Subfamily),
            "genus" => Ok(OpenTreeOfLifeRank::Genus),
            "species group" => Ok(OpenTreeOfLifeRank::SpeciesGroup),
            "species subgroup" => Ok(OpenTreeOfLifeRank::SpeciesSubGroup),
            "species" => Ok(OpenTreeOfLifeRank::Species),
            "subspecies" => Ok(OpenTreeOfLifeRank::Subspecies),
            _ => Err(TaxonEntryBuilderError::UnknownRank(s.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for OpenTreeOfLifeRank {
    fn deserialize<D>(deserializer: D) -> Result<OpenTreeOfLifeRank, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OpenTreeOfLifeRank::from_str(&s).map_err(serde::de::Error::custom)
    }
}
