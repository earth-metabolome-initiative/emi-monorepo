//! Submodule defining the rank enumeration for NCBI taxonomy.

use std::str::FromStr;

use serde::de::Deserialize;

use crate::{errors::TaxonEntryBuilderError, traits::Rank};

use super::taxon_entry::NCBITaxonEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl std::fmt::Display for NCBIRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
