//! Submodule defining the rank enumeration for Open Tree of Life taxonomy.

use std::str::FromStr;

use crate::{errors::TaxonEntryBuilderError, traits::Rank};
use font_awesome::Icon;
use serde::de::Deserialize;
use serde::ser::{Serialize, Serializer};
use strum::EnumIter;

use super::taxon_entry::OpenTreeOfLifeTaxonEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
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
    fn description(&self) -> &'static str {
        match self {
            OpenTreeOfLifeRank::NoRank => "No specific rank assigned.",
            OpenTreeOfLifeRank::NoRankTerminal => "No specific rank, terminal node.",
            OpenTreeOfLifeRank::Domain => "The highest taxonomic rank, grouping all life forms.",
            OpenTreeOfLifeRank::Kingdom => {
                "A major rank, grouping organisms with similar characteristics."
            }
            OpenTreeOfLifeRank::SuperKingdom => "A rank above kingdom, used for large clades.",
            OpenTreeOfLifeRank::SubKingdom => {
                "A rank below kingdom, dividing kingdoms into smaller groups."
            }
            OpenTreeOfLifeRank::Infrakingdom => {
                "A rank below subkingdom, for finer classifications."
            }
            OpenTreeOfLifeRank::Phylum => "A major taxonomic rank grouping classes of organisms.",
            OpenTreeOfLifeRank::SuperPhylum => "A rank above phylum for large clades.",
            OpenTreeOfLifeRank::Infraphylum => "A rank below phylum for finer divisions.",
            OpenTreeOfLifeRank::SubPhylum => "A rank below phylum, dividing it further.",
            OpenTreeOfLifeRank::Class => "A taxonomic rank grouping orders of organisms.",
            OpenTreeOfLifeRank::SuperClass => "A rank above class, for larger groupings.",
            OpenTreeOfLifeRank::SubClass => "A rank below class for finer divisions.",
            OpenTreeOfLifeRank::SubterClass => "A rank below subclass, for additional specificity.",
            OpenTreeOfLifeRank::InfraClass => "A rank below subclass for finer classifications.",
            OpenTreeOfLifeRank::Order => "A taxonomic rank grouping families of organisms.",
            OpenTreeOfLifeRank::InfraOrder => "A rank below order, for smaller groupings.",
            OpenTreeOfLifeRank::ParvOrder => "A minor division below infraorder.",
            OpenTreeOfLifeRank::SuperOrder => "A rank above order, grouping related orders.",
            OpenTreeOfLifeRank::SubOrder => {
                "A rank below order, dividing orders into smaller groups."
            }
            OpenTreeOfLifeRank::Family => "A taxonomic rank grouping genera.",
            OpenTreeOfLifeRank::SuperFamily => "A rank above family, grouping related families.",
            OpenTreeOfLifeRank::Subfamily => "A rank below family for finer distinctions.",
            OpenTreeOfLifeRank::Genus => "A taxonomic rank grouping species with shared traits.",
            OpenTreeOfLifeRank::Subgenus => "A rank below genus, grouping closely related species.",
            OpenTreeOfLifeRank::Forma => "A minor taxonomic rank for variations within a species.",
            OpenTreeOfLifeRank::SubForm => "A rank below forma for specific variations.",
            OpenTreeOfLifeRank::SpeciesGroup => "A grouping of related species.",
            OpenTreeOfLifeRank::SpeciesSubGroup => "A subdivision within a species group.",
            OpenTreeOfLifeRank::Species => {
                "The basic rank, representing individual organisms capable of interbreeding."
            }
            OpenTreeOfLifeRank::Subspecies => {
                "A rank below species, denoting geographic or morphological variation."
            }
            OpenTreeOfLifeRank::InfraSpecifiNname => "A rank for naming infraspecific variations.",
            OpenTreeOfLifeRank::Varietas => "A rank for botanical varieties within a species.",
            OpenTreeOfLifeRank::SubVarietas => "A rank below varietas for finer distinctions.",
            OpenTreeOfLifeRank::Tribe => "A rank grouping related genera within a family.",
            OpenTreeOfLifeRank::SuperTribe => "A rank above tribe, grouping related tribes.",
            OpenTreeOfLifeRank::SubTribe => "A rank below tribe for finer distinctions.",
            OpenTreeOfLifeRank::SubDivision => "A rank below division for smaller groupings.",
            OpenTreeOfLifeRank::Section => "A rank grouping species within a genus.",
            OpenTreeOfLifeRank::SubSection => "A rank below section for finer groupings.",
            OpenTreeOfLifeRank::Cohort => "A taxonomic rank grouping related orders or families.",
            OpenTreeOfLifeRank::SubCohort => "A rank below cohort for additional classification.",
            OpenTreeOfLifeRank::Samples => {
                "Represents samples of organisms without specific taxonomy."
            }
            OpenTreeOfLifeRank::Natio => "An obsolete or informal rank for populations.",
        }
    }
}

impl std::fmt::Display for OpenTreeOfLifeRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpenTreeOfLifeRank::NoRank => write!(f, "No rank"),
            OpenTreeOfLifeRank::NoRankTerminal => write!(f, "No rank - terminal"),
            OpenTreeOfLifeRank::Domain => write!(f, "Domain"),
            OpenTreeOfLifeRank::Kingdom => write!(f, "Kingdom"),
            OpenTreeOfLifeRank::SuperKingdom => write!(f, "Superkingdom"),
            OpenTreeOfLifeRank::SubKingdom => write!(f, "Subkingdom"),
            OpenTreeOfLifeRank::Infrakingdom => write!(f, "Infrakingdom"),
            OpenTreeOfLifeRank::Phylum => write!(f, "Phylum"),
            OpenTreeOfLifeRank::SuperPhylum => write!(f, "Superphylum"),
            OpenTreeOfLifeRank::Infraphylum => write!(f, "Infraphylum"),
            OpenTreeOfLifeRank::SubPhylum => write!(f, "Subphylum"),
            OpenTreeOfLifeRank::Class => write!(f, "Class"),
            OpenTreeOfLifeRank::SuperClass => write!(f, "Superclass"),
            OpenTreeOfLifeRank::SubClass => write!(f, "Subclass"),
            OpenTreeOfLifeRank::SubterClass => write!(f, "Subterclass"),
            OpenTreeOfLifeRank::InfraClass => write!(f, "Infraclass"),
            OpenTreeOfLifeRank::Order => write!(f, "Order"),
            OpenTreeOfLifeRank::InfraOrder => write!(f, "Infraorder"),
            OpenTreeOfLifeRank::ParvOrder => write!(f, "Parvorder"),
            OpenTreeOfLifeRank::SuperOrder => write!(f, "Superorder"),
            OpenTreeOfLifeRank::SubOrder => write!(f, "Suborder"),
            OpenTreeOfLifeRank::Family => write!(f, "Family"),
            OpenTreeOfLifeRank::SuperFamily => write!(f, "Superfamily"),
            OpenTreeOfLifeRank::Subfamily => write!(f, "Subfamily"),
            OpenTreeOfLifeRank::Genus => write!(f, "Genus"),
            OpenTreeOfLifeRank::Forma => write!(f, "Forma"),
            OpenTreeOfLifeRank::SubForm => write!(f, "Subform"),
            OpenTreeOfLifeRank::Subgenus => write!(f, "Subgenus"),
            OpenTreeOfLifeRank::SpeciesGroup => write!(f, "Species group"),
            OpenTreeOfLifeRank::SpeciesSubGroup => write!(f, "Species subgroup"),
            OpenTreeOfLifeRank::Species => write!(f, "Species"),
            OpenTreeOfLifeRank::Subspecies => write!(f, "Subspecies"),
            OpenTreeOfLifeRank::InfraSpecifiNname => write!(f, "Infraspecificname"),
            OpenTreeOfLifeRank::Varietas => write!(f, "Varietas"),
            OpenTreeOfLifeRank::SubVarietas => write!(f, "Subvarietas"),
            OpenTreeOfLifeRank::Tribe => write!(f, "Tribe"),
            OpenTreeOfLifeRank::SuperTribe => write!(f, "Supertribe"),
            OpenTreeOfLifeRank::SubTribe => write!(f, "Subtribe"),
            OpenTreeOfLifeRank::SubDivision => write!(f, "Subdivision"),
            OpenTreeOfLifeRank::Section => write!(f, "Section"),
            OpenTreeOfLifeRank::SubSection => write!(f, "Subsection"),
            OpenTreeOfLifeRank::Cohort => write!(f, "Cohort"),
            OpenTreeOfLifeRank::SubCohort => write!(f, "Subcohort"),
            OpenTreeOfLifeRank::Samples => write!(f, "Samples"),
            OpenTreeOfLifeRank::Natio => write!(f, "Natio"),
        }
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

impl Serialize for OpenTreeOfLifeRank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
