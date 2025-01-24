//! Submodule defining the rank enumeration for Open Tree of Life taxonomy.

use std::str::FromStr;

use serde::{de::Deserialize, Serialize};
use strum::EnumIter;

use crate::{errors::TaxonEntryBuilderError, traits::Rank};

use super::taxon_entry::CatalogOfLifeTaxonEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
/// Enumeration of the ranks used in the Open Tree of Life taxonomy.
pub enum CatalogOfLifeRank {
    /// No rank.
    NoRank,
    /// Unranked.
    Unranked,
    /// Aberration
    Aberration,
    /// section botany
    SectionBotany,
    /// subsection botany
    SubSectionBotany,
    /// section zoology
    SectionZoology,
    /// subsection zoology
    SubSectionZoology,
    /// Other
    Other,
    /// Species.
    Species,
    /// SubSpecies.
    SubSpecies,
    /// Variety.
    Variety,
    /// SubVariety
    SubVariety,
    /// Genus
    Genus,
    /// SubGenus
    SubGenus,
    /// infrageneric name
    InfragenericName,
    /// Kingdom
    Kingdom,
    /// SubKingdom
    SubKingdom,
    /// Realm
    Realm,
    /// Family
    Family,
    /// EbiFamily
    EbiFamily,
    /// EpiFamily
    EpiFamily,
    /// InfraFamily
    InfraFamily,
    /// SuperFamily
    SuperFamily,
    /// SubFamily
    SubFamily,
    /// Form
    Form,
    /// SubForm
    SubForm,
    /// Forma Specialis
    FormaSpecialis,
    /// Tribe
    Tribe,
    /// SuperTribe
    SuperTribe,
    /// Sub Tribe
    SubTribe,
    /// InfraTribe
    InfraTribe,
    /// Infraspecific Name
    InfraspecificName,
    /// infrasubspecific name
    InfrasubspecificName,
    /// Species Aggregate
    SpeciesAggregate,
    /// Order
    Order,
    /// InfraOrder
    InfraOrder,
    /// Parvorder
    Parvorder,
    /// Super Order
    SuperOrder,
    /// Sub Order
    SubOrder,
    /// NanOrder
    NanOrder,
    /// Proles
    Proles,
    /// Lusus
    Lusus,
    /// Mutatio
    Mutatio,
    /// Phylum
    Phylum,
    /// SubPhylum
    SubPhylum,
    /// parvphylum
    Parvphylum,
    /// InfraPhylum
    InfraPhylum,
    /// Class
    Class,
    /// MegaClass
    MegaClass,
    /// SuperClass
    SuperClass,
    /// InfraClass
    InfraClass,
    /// Sub Class
    SubClass,
    /// SubterClass
    SubterClass,
    /// Natio
    Natio,
}

impl Rank for CatalogOfLifeRank {
    fn description(&self) -> &'static str {
        match self {
            CatalogOfLifeRank::NoRank => "No specific taxonomic rank assigned.",
            CatalogOfLifeRank::Unranked => "An unranked position within the taxonomy.",
            CatalogOfLifeRank::Aberration => "An unusual deviation or irregularity within a species.",
            CatalogOfLifeRank::SectionBotany => "A grouping of plants within a genus for botany.",
            CatalogOfLifeRank::SubSectionBotany => "A subdivision of a section in botanical classification.",
            CatalogOfLifeRank::SectionZoology => "A grouping of animals within a genus for zoology.",
            CatalogOfLifeRank::SubSectionZoology => "A subdivision of a section in zoological classification.",
            CatalogOfLifeRank::Other => "A taxonomic rank not specified within standard categories.",
            CatalogOfLifeRank::Species => "The basic unit of biological classification, representing interbreeding organisms.",
            CatalogOfLifeRank::SubSpecies => "A taxonomic rank below species, indicating geographic or morphological variation.",
            CatalogOfLifeRank::Variety => "A botanical rank below species, often used for cultivated plants.",
            CatalogOfLifeRank::SubVariety => "A finer subdivision of a variety.",
            CatalogOfLifeRank::Genus => "A taxonomic rank grouping species with similar characteristics.",
            CatalogOfLifeRank::SubGenus => "A rank below genus, grouping closely related species.",
            CatalogOfLifeRank::InfragenericName => "A name assigned below the genus level for further specificity.",
            CatalogOfLifeRank::Kingdom => "A major rank grouping organisms into large categories.",
            CatalogOfLifeRank::SubKingdom => "A rank below kingdom for more specific classification.",
            CatalogOfLifeRank::Realm => "A broad taxonomic rank, often used in zoogeography or microbiology.",
            CatalogOfLifeRank::Family => "A taxonomic rank grouping related genera.",
            CatalogOfLifeRank::EbiFamily => "A specific taxonomic family grouping for organisms.",
            CatalogOfLifeRank::EpiFamily => "A rank denoting families at an extended level.",
            CatalogOfLifeRank::InfraFamily => "A rank below family, grouping closely related genera.",
            CatalogOfLifeRank::SuperFamily => "A rank above family, grouping related families.",
            CatalogOfLifeRank::SubFamily => "A rank below family, grouping genera within a family.",
            CatalogOfLifeRank::Form => "A minor taxonomic rank for variations within a species.",
            CatalogOfLifeRank::SubForm => "A subdivision of form, providing additional specificity.",
            CatalogOfLifeRank::FormaSpecialis => "A form distinguished by specialized pathogenic characteristics.",
            CatalogOfLifeRank::Tribe => "A rank grouping related genera within a family.",
            CatalogOfLifeRank::SuperTribe => "A rank above tribe, grouping related tribes.",
            CatalogOfLifeRank::SubTribe => "A rank below tribe for finer distinctions.",
            CatalogOfLifeRank::InfraTribe => "A rank below subtribe for even finer classification.",
            CatalogOfLifeRank::InfraspecificName => "A name given below species for specific variations.",
            CatalogOfLifeRank::InfrasubspecificName => "A name denoting distinctions below infraspecific level.",
            CatalogOfLifeRank::SpeciesAggregate => "A group of closely related species treated collectively.",
            CatalogOfLifeRank::Order => "A taxonomic rank grouping families.",
            CatalogOfLifeRank::InfraOrder => "A rank below order, grouping closely related families.",
            CatalogOfLifeRank::Parvorder => "A minor division within an infraorder.",
            CatalogOfLifeRank::SuperOrder => "A rank above order, grouping related orders.",
            CatalogOfLifeRank::SubOrder => "A rank below order for finer groupings.",
            CatalogOfLifeRank::NanOrder => "A rare subdivision below suborder.",
            CatalogOfLifeRank::Proles => "A subdivision denoting progeny or offspring.",
            CatalogOfLifeRank::Lusus => "A term representing an unusual form or mutation.",
            CatalogOfLifeRank::Mutatio => "A rank representing a genetic mutation.",
            CatalogOfLifeRank::Phylum => "A major rank grouping classes of organisms.",
            CatalogOfLifeRank::SubPhylum => "A rank below phylum, dividing it further.",
            CatalogOfLifeRank::Parvphylum => "A minor rank below subphylum.",
            CatalogOfLifeRank::InfraPhylum => "A rank below parvphylum for finer classification.",
            CatalogOfLifeRank::Class => "A taxonomic rank grouping orders of organisms.",
            CatalogOfLifeRank::MegaClass => "A rank above class for broader groupings.",
            CatalogOfLifeRank::SuperClass => "A rank above class, grouping related classes.",
            CatalogOfLifeRank::InfraClass => "A rank below class for smaller groupings.",
            CatalogOfLifeRank::SubClass => "A rank below class for finer distinctions.",
            CatalogOfLifeRank::SubterClass => "A further subdivision below subclass.",
            CatalogOfLifeRank::Natio => "A term used to denote a national or geographic variety.",
        }
    }
}

impl std::fmt::Display for CatalogOfLifeRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CatalogOfLifeRank::NoRank => write!(f, "No rank"),
            CatalogOfLifeRank::Unranked => write!(f, "Unranked"),
            CatalogOfLifeRank::Aberration => write!(f, "Aberration"),
            CatalogOfLifeRank::SectionBotany => write!(f, "Section Botany"),
            CatalogOfLifeRank::SubSectionBotany => write!(f, "Subsection Botany"),
            CatalogOfLifeRank::SectionZoology => write!(f, "Section Zoology"),
            CatalogOfLifeRank::SubSectionZoology => write!(f, "Subsection Zoology"),
            CatalogOfLifeRank::Other => write!(f, "Other"),
            CatalogOfLifeRank::Species => write!(f, "Species"),
            CatalogOfLifeRank::SubSpecies => write!(f, "Subspecies"),
            CatalogOfLifeRank::SpeciesAggregate => write!(f, "Species Aggregate"),
            CatalogOfLifeRank::Variety => write!(f, "Variety"),
            CatalogOfLifeRank::SubVariety => write!(f, "Subvariety"),
            CatalogOfLifeRank::Genus => write!(f, "Genus"),
            CatalogOfLifeRank::SubGenus => write!(f, "Subgenus"),
            CatalogOfLifeRank::InfragenericName => write!(f, "Infrageneric Name"),
            CatalogOfLifeRank::Kingdom => write!(f, "Kingdom"),
            CatalogOfLifeRank::SubKingdom => write!(f, "Subkingdom"),
            CatalogOfLifeRank::Realm => write!(f, "Realm"),
            CatalogOfLifeRank::Family => write!(f, "Family"),
            CatalogOfLifeRank::EbiFamily => write!(f, "EbiFamily"),
            CatalogOfLifeRank::EpiFamily => write!(f, "EpiFamily"),
            CatalogOfLifeRank::InfraFamily => write!(f, "InfraFamily"),
            CatalogOfLifeRank::SuperFamily => write!(f, "SuperFamily"),
            CatalogOfLifeRank::SubFamily => write!(f, "SubFamily"),
            CatalogOfLifeRank::Form => write!(f, "Form"),
            CatalogOfLifeRank::SubForm => write!(f, "SubForm"),
            CatalogOfLifeRank::FormaSpecialis => write!(f, "Forma Specialis"),
            CatalogOfLifeRank::Tribe => write!(f, "Tribe"),
            CatalogOfLifeRank::SuperTribe => write!(f, "SuperTribe"),
            CatalogOfLifeRank::SubTribe => write!(f, "SubTribe"),
            CatalogOfLifeRank::InfraTribe => write!(f, "InfraTribe"),
            CatalogOfLifeRank::InfraspecificName => write!(f, "Infraspecific Name"),
            CatalogOfLifeRank::InfrasubspecificName => write!(f, "Infrasubspecific Name"),
            CatalogOfLifeRank::Order => write!(f, "Order"),
            CatalogOfLifeRank::NanOrder => write!(f, "NanOrder"),
            CatalogOfLifeRank::InfraOrder => write!(f, "InfraOrder"),
            CatalogOfLifeRank::Parvorder => write!(f, "Parvorder"),
            CatalogOfLifeRank::SuperOrder => write!(f, "SuperOrder"),
            CatalogOfLifeRank::SubOrder => write!(f, "SubOrder"),
            CatalogOfLifeRank::Proles => write!(f, "Proles"),
            CatalogOfLifeRank::Lusus => write!(f, "Lusus"),
            CatalogOfLifeRank::Mutatio => write!(f, "Mutatio"),
            CatalogOfLifeRank::Phylum => write!(f, "Phylum"),
            CatalogOfLifeRank::SubPhylum => write!(f, "SubPhylum"),
            CatalogOfLifeRank::Parvphylum => write!(f, "Parvphylum"),
            CatalogOfLifeRank::InfraPhylum => write!(f, "InfraPhylum"),
            CatalogOfLifeRank::Class => write!(f, "Class"),
            CatalogOfLifeRank::MegaClass => write!(f, "MegaClass"),
            CatalogOfLifeRank::SuperClass => write!(f, "SuperClass"),
            CatalogOfLifeRank::InfraClass => write!(f, "InfraClass"),
            CatalogOfLifeRank::SubClass => write!(f, "SubClass"),
            CatalogOfLifeRank::SubterClass => write!(f, "SubterClass"),
            CatalogOfLifeRank::Natio => write!(f, "Natio"),
        }
    }
}

impl FromStr for CatalogOfLifeRank {
    type Err = TaxonEntryBuilderError<CatalogOfLifeTaxonEntry>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no rank" => Ok(CatalogOfLifeRank::NoRank),
            "unranked" => Ok(CatalogOfLifeRank::Unranked),
            "aberration" => Ok(CatalogOfLifeRank::Aberration),
            "section botany" => Ok(CatalogOfLifeRank::SectionBotany),
            "subsection botany" => Ok(CatalogOfLifeRank::SubSectionBotany),
            "section zoology" => Ok(CatalogOfLifeRank::SectionZoology),
            "subsection zoology" => Ok(CatalogOfLifeRank::SubSectionZoology),
            "other" => Ok(CatalogOfLifeRank::Other),
            "species" => Ok(CatalogOfLifeRank::Species),
            "subspecies" => Ok(CatalogOfLifeRank::SubSpecies),
            "species aggregate" => Ok(CatalogOfLifeRank::SpeciesAggregate),
            "variety" => Ok(CatalogOfLifeRank::Variety),
            "subvariety" => Ok(CatalogOfLifeRank::SubVariety),
            "genus" => Ok(CatalogOfLifeRank::Genus),
            "subgenus" => Ok(CatalogOfLifeRank::SubGenus),
            "infrageneric name" => Ok(CatalogOfLifeRank::InfragenericName),
            "kingdom" => Ok(CatalogOfLifeRank::Kingdom),
            "subkingdom" => Ok(CatalogOfLifeRank::SubKingdom),
            "realm" => Ok(CatalogOfLifeRank::Realm),
            "family" => Ok(CatalogOfLifeRank::Family),
            "ebifamily" => Ok(CatalogOfLifeRank::EbiFamily),
            "epifamily" => Ok(CatalogOfLifeRank::EbiFamily),
            "infrafamily" => Ok(CatalogOfLifeRank::InfraFamily),
            "superfamily" => Ok(CatalogOfLifeRank::SuperFamily),
            "subfamily" => Ok(CatalogOfLifeRank::SubFamily),
            "form" => Ok(CatalogOfLifeRank::Form),
            "subform" => Ok(CatalogOfLifeRank::SubForm),
            "forma specialis" => Ok(CatalogOfLifeRank::FormaSpecialis),
            "tribe" => Ok(CatalogOfLifeRank::Tribe),
            "supertribe" => Ok(CatalogOfLifeRank::SuperTribe),
            "subtribe" => Ok(CatalogOfLifeRank::SubTribe),
            "infratribe" => Ok(CatalogOfLifeRank::InfraTribe),
            "infraspecific name" => Ok(CatalogOfLifeRank::InfraspecificName),
            "infrasubspecific name" => Ok(CatalogOfLifeRank::InfrasubspecificName),
            "order" => Ok(CatalogOfLifeRank::Order),
            "infraorder" => Ok(CatalogOfLifeRank::InfraOrder),
            "parvorder" => Ok(CatalogOfLifeRank::Parvorder),
            "superorder" => Ok(CatalogOfLifeRank::SuperOrder),
            "suborder" => Ok(CatalogOfLifeRank::SubOrder),
            "proles" => Ok(CatalogOfLifeRank::Proles),
            "lusus" => Ok(CatalogOfLifeRank::Lusus),
            "mutatio" => Ok(CatalogOfLifeRank::Mutatio),
            "phylum" => Ok(CatalogOfLifeRank::Phylum),
            "subphylum" => Ok(CatalogOfLifeRank::SubPhylum),
            "parvphylum" => Ok(CatalogOfLifeRank::Parvphylum),
            "infraphylum" => Ok(CatalogOfLifeRank::InfraPhylum),
            "class" => Ok(CatalogOfLifeRank::Class),
            "megaclass" => Ok(CatalogOfLifeRank::MegaClass),
            "superclass" => Ok(CatalogOfLifeRank::SuperClass),
            "infraclass" => Ok(CatalogOfLifeRank::InfraClass),
            "subclass" => Ok(CatalogOfLifeRank::SubClass),
            "subterclass" => Ok(CatalogOfLifeRank::SubterClass),
            "natio" => Ok(CatalogOfLifeRank::Natio),
            "nanorder" => Ok(CatalogOfLifeRank::NanOrder),
            _ => Err(TaxonEntryBuilderError::UnknownRank(s.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for CatalogOfLifeRank {
    fn deserialize<D>(deserializer: D) -> Result<CatalogOfLifeRank, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        CatalogOfLifeRank::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for CatalogOfLifeRank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}