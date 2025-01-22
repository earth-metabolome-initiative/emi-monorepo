//! Submodule defining the rank enumeration for Open Tree of Life taxonomy.

use std::str::FromStr;

use serde::de::Deserialize;

use crate::{errors::TaxonEntryBuilderError, traits::Rank};

use super::taxon_entry::OpenTreeOfLifeTaxonEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Super kingdom rank.
    SuperKingdom,
    /// Sub kingdom rank.
    SubKingdom,
    /// Infrakingdom rank.
    Infrakingdom,
    /// Phylum rank.
    Phylum,
    /// Superphylum rank.
    SuperPhylum,
    /// Infraphylum rank.
    Infraphylum,
    /// SubPhylum rank.
    SubPhylum,
    /// Class rank.
    Class,
    /// Superclass rank
    SuperClass,
    /// Subclass rank.
    SubClass,
    /// Subterclass rank.
    SubterClass,
    /// Infraclass rank.
    InfraClass,
    /// Order rank.
    Order,
    /// Infraorder rank.
    InfraOrder,
    /// Parvorder rank.
    ParvOrder,
    /// Superorder rank.
    SuperOrder,
    /// Suborder rank.
    SubOrder,
    /// Family rank.
    Family,
    /// Superfamily rank.
    SuperFamily,
    /// Subfamily rank.
    Subfamily,
    /// Genus rank.
    Genus,
    /// Subgenus rank.
    Subgenus,
    /// Forma
    Forma,
    /// SubForm
    SubForm,
    /// Species group rank.
    SpeciesGroup,
    /// Species sub group rank.
    SpeciesSubGroup,
    /// Species rank.
    Species,
    /// Subspecies rank.
    Subspecies,
    /// Infraspecificname rank
    InfraSpecifiNname,
    /// Varietas
    Varietas,
    /// SubVarietas
    SubVarietas,
    /// Tribe
    Tribe,
    /// SuperTribe
    SuperTribe,
    /// SubTribe
    SubTribe,
    /// SubDivision rank
    SubDivision,
    /// Section rank.
    Section,
    /// Subsection rank.
    SubSection,
    /// Cohort rank.
    Cohort,
    /// SubCohort rank.
    SubCohort,
    /// Samples
    Samples,
    /// Natio
    Natio,
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
            "superkingdom" => Ok(OpenTreeOfLifeRank::SuperKingdom),
            "subkingdom" => Ok(OpenTreeOfLifeRank::SubKingdom),
            "infrakingdom" => Ok(OpenTreeOfLifeRank::Infrakingdom),
            "phylum" => Ok(OpenTreeOfLifeRank::Phylum),
            "superphylum" => Ok(OpenTreeOfLifeRank::SuperPhylum),
            "infraphylum" => Ok(OpenTreeOfLifeRank::Infraphylum),
            "subphylum" => Ok(OpenTreeOfLifeRank::SubPhylum),
            "class" => Ok(OpenTreeOfLifeRank::Class),
            "superclass" => Ok(OpenTreeOfLifeRank::SuperClass),
            "subclass" => Ok(OpenTreeOfLifeRank::SubClass),
            "subterclass" => Ok(OpenTreeOfLifeRank::SubterClass),
            "infraclass" => Ok(OpenTreeOfLifeRank::InfraClass),
            "order" => Ok(OpenTreeOfLifeRank::Order),
            "infraorder" => Ok(OpenTreeOfLifeRank::InfraOrder),
            "parvorder" => Ok(OpenTreeOfLifeRank::ParvOrder),
            "superorder" => Ok(OpenTreeOfLifeRank::SuperOrder),
            "suborder" => Ok(OpenTreeOfLifeRank::SubOrder),
            "family" => Ok(OpenTreeOfLifeRank::Family),
            "superfamily" => Ok(OpenTreeOfLifeRank::SuperFamily),
            "subfamily" => Ok(OpenTreeOfLifeRank::Subfamily),
            "genus" => Ok(OpenTreeOfLifeRank::Genus),
            "forma" => Ok(OpenTreeOfLifeRank::Forma),
            "subform" => Ok(OpenTreeOfLifeRank::SubForm),
            "subgenus" => Ok(OpenTreeOfLifeRank::Subgenus),
            "species group" => Ok(OpenTreeOfLifeRank::SpeciesGroup),
            "species subgroup" => Ok(OpenTreeOfLifeRank::SpeciesSubGroup),
            "species" => Ok(OpenTreeOfLifeRank::Species),
            "subspecies" => Ok(OpenTreeOfLifeRank::Subspecies),
            "infraspecificname" => Ok(OpenTreeOfLifeRank::InfraSpecifiNname),
            "varietas" | "variety" => Ok(OpenTreeOfLifeRank::Varietas),
            "subvarietas" | "subvariety" => Ok(OpenTreeOfLifeRank::SubVarietas),
            "tribe" => Ok(OpenTreeOfLifeRank::Tribe),
            "supertribe" => Ok(OpenTreeOfLifeRank::SuperTribe),
            "subtribe" => Ok(OpenTreeOfLifeRank::SubTribe),
            "subdivision" => Ok(OpenTreeOfLifeRank::SubDivision),
            "section" => Ok(OpenTreeOfLifeRank::Section),
            "subsection" => Ok(OpenTreeOfLifeRank::SubSection),
            "cohort" => Ok(OpenTreeOfLifeRank::Cohort),
            "subcohort" => Ok(OpenTreeOfLifeRank::SubCohort),
            "samples" => Ok(OpenTreeOfLifeRank::Samples),
            "natio" => Ok(OpenTreeOfLifeRank::Natio),
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
