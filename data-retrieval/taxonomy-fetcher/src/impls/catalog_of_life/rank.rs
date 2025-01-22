//! Submodule defining the rank enumeration for Open Tree of Life taxonomy.

use std::str::FromStr;

use serde::de::Deserialize;

use crate::{errors::TaxonEntryBuilderError, traits::Rank};

use super::taxon_entry::CatalogOfLifeTaxonEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Epifamily
    Epifamily,
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
}

impl std::fmt::Display for CatalogOfLifeRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
