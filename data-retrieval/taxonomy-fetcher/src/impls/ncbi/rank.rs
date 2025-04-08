//! Submodule defining the rank enumeration for NCBI taxonomy.

use std::{fmt::Display, str::FromStr};

use serde::{de::Deserialize, Serialize};
use strum::EnumIter;

use super::taxon_entry::NCBITaxonEntry;
use crate::{errors::TaxonEntryBuilderError, traits::Rank};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
/// Enumeration of the ranks used in the NCBI taxonomy.
pub enum NCBIRank {
    /// BioType
    BioType,
    /// Class
    Class,
    /// Clade
    Clade,
    /// Cohort
    Cohort,
    /// Family
    Family,
    /// Forma
    Forma,
    /// Forma Specialis
    FormaSpecialis,
    /// Genotype
    Genotype,
    /// Genus
    Genus,
    /// InfraClass
    InfraClass,
    /// InfraOrder
    InfraOrder,
    /// Isolate
    Isolate,
    /// Kingdom
    Kingdom,
    /// Morph
    Morph,
    /// No rank.
    NoRank,
    /// Order
    Order,
    /// ParvOrder
    ParvOrder,
    /// PathoGroup
    PathoGroup,
    /// Phylum
    Phylum,
    /// Section
    Section,
    /// SeroGroup
    SeroGroup,
    /// Serotype
    Serotype,
    /// Series
    Series,
    /// Species
    Species,
    /// Species Group
    SpeciesGroup,
    /// Species Subgroup
    SpeciesSubgroup,
    /// Strain
    Strain,
    /// SubClass
    SubClass,
    /// SubCohort
    SubCohort,
    /// SubFamily
    SubFamily,
    /// SubGenus
    SubGenus,
    /// SubKingdom
    SubKingdom,
    /// SubOrder
    SubOrder,
    /// SubPhylum
    SubPhylum,
    /// SubSection
    SubSection,
    /// SubSpecies
    Subspecies,
    /// SubTribe
    SubTribe,
    /// SubVariety
    SubVariety,
    /// SuperClass
    SuperClass,
    /// SuperFamily
    SuperFamily,
    /// SuperKingdom
    SuperKingdom,
    /// SuperOrder
    SuperOrder,
    /// SuperPhylum
    SuperPhylum,
    /// Tribe
    Tribe,
    /// Varietas
    Varietas,
}

impl Rank for NCBIRank {
    fn description(&self) -> &'static str {
        match self {
            NCBIRank::BioType => {
                "A biological type, often representing a specific characteristic or group."
            }
            NCBIRank::Class => "A taxonomic rank grouping orders of organisms.",
            NCBIRank::Clade => {
                "A group of organisms believed to have evolved from a common ancestor."
            }
            NCBIRank::Cohort => "A taxonomic rank grouping related orders or families.",
            NCBIRank::Family => "A taxonomic rank grouping genera.",
            NCBIRank::Forma => "A minor taxonomic rank for variations within a species.",
            NCBIRank::FormaSpecialis => {
                "A forma distinguished by specific pathogenic characteristics."
            }
            NCBIRank::Genotype => "A group of organisms with the same genetic makeup.",
            NCBIRank::Genus => "A taxonomic rank grouping species with shared traits.",
            NCBIRank::InfraClass => "A rank below subclass for finer classifications.",
            NCBIRank::InfraOrder => "A rank below order, for smaller groupings.",
            NCBIRank::Isolate => "A population of microorganisms obtained from a single source.",
            NCBIRank::Kingdom => "A major rank grouping organisms with similar characteristics.",
            NCBIRank::Morph => "A rank representing a morphological variant.",
            NCBIRank::NoRank => "No specific rank assigned.",
            NCBIRank::Order => "A taxonomic rank grouping families of organisms.",
            NCBIRank::ParvOrder => "A minor division below infraorder.",
            NCBIRank::PathoGroup => "A grouping based on shared pathogenic traits.",
            NCBIRank::Phylum => "A major taxonomic rank grouping classes of organisms.",
            NCBIRank::Section => "A rank grouping species within a genus.",
            NCBIRank::SeroGroup => "A group based on shared serological properties.",
            NCBIRank::Serotype => "A classification based on antigenic properties.",
            NCBIRank::Series => "A taxonomic rank grouping related species or varieties.",
            NCBIRank::Species => {
                "The basic rank, representing individual organisms capable of interbreeding."
            }
            NCBIRank::SpeciesGroup => "A grouping of related species.",
            NCBIRank::SpeciesSubgroup => "A subdivision within a species group.",
            NCBIRank::Strain => "A genetic variant or subtype within a species.",
            NCBIRank::SubClass => "A rank below class for finer divisions.",
            NCBIRank::SubCohort => "A rank below cohort for additional classification.",
            NCBIRank::SubFamily => "A rank below family for finer distinctions.",
            NCBIRank::SubGenus => "A rank below genus, grouping closely related species.",
            NCBIRank::SubKingdom => "A rank below kingdom, dividing kingdoms into smaller groups.",
            NCBIRank::SubOrder => "A rank below order, dividing orders into smaller groups.",
            NCBIRank::SubPhylum => "A rank below phylum, dividing it further.",
            NCBIRank::SubSection => "A rank below section for finer groupings.",
            NCBIRank::Subspecies => {
                "A rank below species, denoting geographic or morphological variation."
            }
            NCBIRank::SubTribe => "A rank below tribe for finer distinctions.",
            NCBIRank::SubVariety => "A rank below variety for additional specificity.",
            NCBIRank::SuperClass => "A rank above class, for larger groupings.",
            NCBIRank::SuperFamily => "A rank above family, grouping related families.",
            NCBIRank::SuperKingdom => "A rank above kingdom, used for large clades.",
            NCBIRank::SuperOrder => "A rank above order, grouping related orders.",
            NCBIRank::SuperPhylum => "A rank above phylum for large clades.",
            NCBIRank::Tribe => "A rank grouping related genera within a family.",
            NCBIRank::Varietas => "A rank for botanical varieties within a species.",
        }
    }
}

impl FromStr for NCBIRank {
    type Err = TaxonEntryBuilderError<NCBITaxonEntry>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no rank" => Ok(NCBIRank::NoRank),
            "superkingdom" => Ok(NCBIRank::SuperKingdom),
            "kingdom" => Ok(NCBIRank::Kingdom),
            "subkingdom" => Ok(NCBIRank::SubKingdom),
            "genus" => Ok(NCBIRank::Genus),
            "subgenus" => Ok(NCBIRank::SubGenus),
            "species group" => Ok(NCBIRank::SpeciesGroup),
            "species subgroup" => Ok(NCBIRank::SpeciesSubgroup),
            "species" => Ok(NCBIRank::Species),
            "subspecies" => Ok(NCBIRank::Subspecies),
            "order" => Ok(NCBIRank::Order),
            "parvorder" => Ok(NCBIRank::ParvOrder),
            "superorder" => Ok(NCBIRank::SuperOrder),
            "infraorder" => Ok(NCBIRank::InfraOrder),
            "suborder" => Ok(NCBIRank::SubOrder),
            "family" => Ok(NCBIRank::Family),
            "superfamily" => Ok(NCBIRank::SuperFamily),
            "subfamily" => Ok(NCBIRank::SubFamily),
            "tribe" => Ok(NCBIRank::Tribe),
            "subtribe" => Ok(NCBIRank::SubTribe),
            "phylum" => Ok(NCBIRank::Phylum),
            "superphylum" => Ok(NCBIRank::SuperPhylum),
            "subphylum" => Ok(NCBIRank::SubPhylum),
            "class" => Ok(NCBIRank::Class),
            "superclass" => Ok(NCBIRank::SuperClass),
            "infraclass" => Ok(NCBIRank::InfraClass),
            "subclass" => Ok(NCBIRank::SubClass),
            "forma" => Ok(NCBIRank::Forma),
            "forma specialis" => Ok(NCBIRank::FormaSpecialis),
            "varietas" => Ok(NCBIRank::Varietas),
            "subvariety" => Ok(NCBIRank::SubVariety),
            "cohort" => Ok(NCBIRank::Cohort),
            "subcohort" => Ok(NCBIRank::SubCohort),
            "section" => Ok(NCBIRank::Section),
            "subsection" => Ok(NCBIRank::SubSection),
            "series" => Ok(NCBIRank::Series),
            "strain" => Ok(NCBIRank::Strain),
            "serogroup" => Ok(NCBIRank::SeroGroup),
            "pathogroup" => Ok(NCBIRank::PathoGroup),
            "biotype" => Ok(NCBIRank::BioType),
            "clade" => Ok(NCBIRank::Clade),
            "isolate" => Ok(NCBIRank::Isolate),
            "serotype" => Ok(NCBIRank::Serotype),
            "genotype" => Ok(NCBIRank::Genotype),
            "morph" => Ok(NCBIRank::Morph),
            _ => Err(TaxonEntryBuilderError::UnknownRank(s.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for NCBIRank {
    fn deserialize<D>(deserializer: D) -> Result<NCBIRank, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NCBIRank::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Display for NCBIRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NCBIRank::BioType => write!(f, "BioType"),
            NCBIRank::Class => write!(f, "Class"),
            NCBIRank::Clade => write!(f, "Clade"),
            NCBIRank::Cohort => write!(f, "Cohort"),
            NCBIRank::Family => write!(f, "Family"),
            NCBIRank::Forma => write!(f, "Forma"),
            NCBIRank::FormaSpecialis => write!(f, "Forma Specialis"),
            NCBIRank::Genotype => write!(f, "Genotype"),
            NCBIRank::Genus => write!(f, "Genus"),
            NCBIRank::InfraClass => write!(f, "InfraClass"),
            NCBIRank::InfraOrder => write!(f, "InfraOrder"),
            NCBIRank::Isolate => write!(f, "Isolate"),
            NCBIRank::Kingdom => write!(f, "Kingdom"),
            NCBIRank::Morph => write!(f, "Morph"),
            NCBIRank::NoRank => write!(f, "No rank"),
            NCBIRank::Order => write!(f, "Order"),
            NCBIRank::ParvOrder => write!(f, "ParvOrder"),
            NCBIRank::PathoGroup => write!(f, "PathoGroup"),
            NCBIRank::Phylum => write!(f, "Phylum"),
            NCBIRank::Section => write!(f, "Section"),
            NCBIRank::SeroGroup => write!(f, "SeroGroup"),
            NCBIRank::Serotype => write!(f, "Serotype"),
            NCBIRank::Series => write!(f, "Series"),
            NCBIRank::Species => write!(f, "Species"),
            NCBIRank::SpeciesGroup => write!(f, "Species Group"),
            NCBIRank::SpeciesSubgroup => write!(f, "Species Subgroup"),
            NCBIRank::Strain => write!(f, "Strain"),
            NCBIRank::SubClass => write!(f, "SubClass"),
            NCBIRank::SubCohort => write!(f, "SubCohort"),
            NCBIRank::SubFamily => write!(f, "SubFamily"),
            NCBIRank::SubGenus => write!(f, "SubGenus"),
            NCBIRank::SubKingdom => write!(f, "SubKingdom"),
            NCBIRank::SubOrder => write!(f, "SubOrder"),
            NCBIRank::SubPhylum => write!(f, "SubPhylum"),
            NCBIRank::SubSection => write!(f, "SubSection"),
            NCBIRank::Subspecies => write!(f, "Subspecies"),
            NCBIRank::SubTribe => write!(f, "SubTribe"),
            NCBIRank::SubVariety => write!(f, "SubVariety"),
            NCBIRank::SuperClass => write!(f, "SuperClass"),
            NCBIRank::SuperFamily => write!(f, "SuperFamily"),
            NCBIRank::SuperKingdom => write!(f, "SuperKingdom"),
            NCBIRank::SuperOrder => write!(f, "SuperOrder"),
            NCBIRank::SuperPhylum => write!(f, "SuperPhylum"),
            NCBIRank::Tribe => write!(f, "Tribe"),
            NCBIRank::Varietas => write!(f, "Varietas"),
        }
    }
}

impl Serialize for NCBIRank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
