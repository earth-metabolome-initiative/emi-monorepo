//! Submodule defining the rank enumeration for Open Tree of Life taxonomy.

use std::str::FromStr;

use crate::{errors::TaxonEntryBuilderError, traits::Rank};

use super::taxon_entry::OpenTreeOfLifeTaxonEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Enumeration of the ranks used in the Open Tree of Life taxonomy.
pub enum OpenTreeOfLifeRank {
    /// No rank.
    NoRank,
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
    /// Genus rank.
    Genus,
    /// Species rank.
    Species,
    /// Subspecies rank.
    Subspecies,
}

impl Rank for OpenTreeOfLifeRank {}

impl std::fmt::Display for OpenTreeOfLifeRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for OpenTreeOfLifeRank {
    type Err = TaxonEntryBuilderError<OpenTreeOfLifeTaxonEntry>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "domain" => Ok(OpenTreeOfLifeRank::Domain),
            "kingdom" => Ok(OpenTreeOfLifeRank::Kingdom),
            "phylum" => Ok(OpenTreeOfLifeRank::Phylum),
            "class" => Ok(OpenTreeOfLifeRank::Class),
            "order" => Ok(OpenTreeOfLifeRank::Order),
            "family" => Ok(OpenTreeOfLifeRank::Family),
            "genus" => Ok(OpenTreeOfLifeRank::Genus),
            "species" => Ok(OpenTreeOfLifeRank::Species),
            "subspecies" => Ok(OpenTreeOfLifeRank::Subspecies),
            _ => Err(TaxonEntryBuilderError::UnknownRank(s.to_string())),
        }
    }
}
