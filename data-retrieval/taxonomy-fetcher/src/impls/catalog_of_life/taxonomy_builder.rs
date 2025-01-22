//! Implementation of the taxonomy builder for the Open Tree of Life taxonomy.

use super::rank::CatalogOfLifeRank;
use super::COLId;
use super::{
    taxon_entry::CatalogOfLifeTaxonEntry, taxon_entry_builder::CatalogOfLifeTaxonEntryBuilder,
    taxonomy::CatalogOfLifeTaxonomy, version::CatalogOfLifeVersion,
};
use crate::traits::TaxonomyBuilder;
use crate::utils::separator_fixed_reader::SeparatorFixedReader;
use crate::TaxonEntryBuilder;
use csv::ReaderBuilder;
use downloader::Downloader;
use reqwest::Url;
use serde::Deserialize;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Default)]
/// Implementation of the taxonomy trait for the Open Tree of Life.
pub struct CatalogOfLifeTaxonomyBuilder {
    /// Version of the Open Tree of Life taxonomy to build.
    version: Option<CatalogOfLifeVersion>,
    /// Set the directory where the taxonomy is stored.
    directory: Option<std::path::PathBuf>,
    /// Root of the taxonomy.
    root_position: Option<u32>,
    /// Taxon entries.
    taxon_entries: Vec<CatalogOfLifeTaxonEntry>,
    /// Hashmap from taxon name to position in taxon entries.
    name_to_position: std::collections::HashMap<String, u32>,
    /// Hashmap from taxon identifier to position in taxon entries.
    id_to_position: std::collections::HashMap<COLId, u32>,
}

#[derive(Debug, PartialEq, Eq)]
/// Represents the taxonomical status of a taxon.
enum TaxonomicalStatus {
    Synonym,
    Accepted,
    ProvisionallyAccepted,
    AmbiguosSynonym,
    Misapplied
}

impl FromStr for TaxonomicalStatus {
    type Err = crate::errors::TaxonEntryBuilderError<super::taxon_entry::CatalogOfLifeTaxonEntry>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "synonym" => Ok(TaxonomicalStatus::Synonym),
            "accepted" => Ok(TaxonomicalStatus::Accepted),
            "ambiguous synonym" => Ok(TaxonomicalStatus::AmbiguosSynonym),
            "provisionally accepted" => Ok(TaxonomicalStatus::ProvisionallyAccepted),
            "misapplied" => Ok(TaxonomicalStatus::Misapplied),
            _ => Err(crate::errors::TaxonEntryBuilderError::UnknownTaxonomicalStatus(s.to_owned())),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TaxonomicalStatus {
    fn deserialize<D>(deserializer: D) -> Result<TaxonomicalStatus, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        TaxonomicalStatus::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
/// Row in the Open Tree of Life taxonomy.
struct TaxonomyRow {
    /// Unique identifier of the taxon.
    /// dwc:taxonID     dwc:parentNameUsageID   dwc:acceptedNameUsageID dwc:originalNameUsageID dwc:scientificNameID    dwc:datasetID   dwc:taxonomicStatus     dwc:taxonRank      dwc:scientificName      dwc:scientificNameAuthorship    col:notho       dwc:genericName dwc:infragenericEpithet dwc:specificEpithet     dwc:infraspecificEpithet   dwc:cultivarEpithet     dwc:nameAccordingTo     dwc:namePublishedIn     dwc:nomenclaturalCode   dwc:nomenclaturalStatus dwc:taxonRemarks        dcterms:references
    // 673FW           3CP83   7YDkrov8jzu-BK-5Yf51k   ----D8DjzaNRExOS-pw7t1  2232    synonym species Anisophyllum hyssopifolium (L.) Haw.    (L.) Haw.               Anisophyllum               hyssopifolium                           Syn. Pl. Succ.: 161 (1812)      ICN
    #[serde(rename = "dwc:taxonID")]
    uid: COLId,
    #[serde(rename = "dwc:parentNameUsageID")]
    parent_uid: Option<COLId>,
    #[serde(rename = "dwc:acceptedNameUsageID")]
    accepted_name_usage_id: Option<COLId>,
    #[serde(rename = "dwc:originalNameUsageID")]
    original_name_usage_id: Option<String>,
    #[serde(rename = "dwc:scientificNameID")]
    scientific_name_id: String,
    #[serde(rename = "dwc:datasetID")]
    dataset_id: Option<u32>,
    #[serde(rename = "dwc:taxonomicStatus")]
    taxonomic_status: TaxonomicalStatus,
    #[serde(rename = "dwc:taxonRank")]
    rank: CatalogOfLifeRank,
    #[serde(rename = "dwc:scientificName")]
    name: String,
    #[serde(rename = "dwc:scientificNameAuthorship")]
    authorship: Option<String>,
    #[serde(rename = "col:notho")]
    notho: Option<String>,
    #[serde(rename = "dwc:genericName")]
    generic_name: Option<String>,
    #[serde(rename = "dwc:infragenericEpithet")]
    infrageneric_epithet: Option<String>,
    #[serde(rename = "dwc:specificEpithet")]
    specific_epithet: Option<String>,
    #[serde(rename = "dwc:infraspecificEpithet")]
    infraspecific_epithet: Option<String>,
    #[serde(rename = "dwc:cultivarEpithet")]
    cultivar_epithet: Option<String>,
    #[serde(rename = "dwc:nameAccordingTo")]
    name_according_to: Option<String>,
    #[serde(rename = "dwc:namePublishedIn")]
    name_published_in: Option<String>,
    #[serde(rename = "dwc:nomenclaturalCode")]
    nomenclatural_code: Option<String>,
    #[serde(rename = "dwc:nomenclaturalStatus")]
    nomenclatural_status: Option<String>,
    #[serde(rename = "dwc:taxonRemarks")]
    taxon_remarks: Option<String>,
    #[serde(rename = "dcterms:references")]
    references: Option<String>,
}

impl TaxonomyBuilder for CatalogOfLifeTaxonomyBuilder {
    type TaxonEntry = CatalogOfLifeTaxonEntry;
    type Taxonomy = CatalogOfLifeTaxonomy;
    type TaxonEntryBuilder = CatalogOfLifeTaxonEntryBuilder;

    fn version(self, version: <Self::Taxonomy as crate::traits::Taxonomy>::Version) -> Self {
        Self {
            version: Some(version),
            ..self
        }
    }

    fn directory(self, directory: std::path::PathBuf) -> Self {
        Self {
            directory: Some(directory),
            ..self
        }
    }

    fn is_id_in_use(&self, id: &<Self::TaxonEntry as crate::traits::TaxonEntry>::Id) -> bool {
        self.id_to_position.contains_key(id)
    }

    fn is_name_in_use(&self, name: &str) -> bool {
        self.name_to_position.contains_key(name)
    }

    fn get_taxon_entry(
        &self,
        id: &<Self::TaxonEntry as crate::traits::TaxonEntry>::Id,
    ) -> Option<&Self::TaxonEntry> {
        self.id_to_position
            .get(id)
            .map(|&pos| &self.taxon_entries[pos as usize])
    }

    async fn build(
        mut self,
    ) -> Result<Self::Taxonomy, crate::errors::TaxonomyBuilderError<Self::TaxonEntry>> {
        let version = self
            .version
            .ok_or(crate::errors::TaxonomyBuilderError::MissingVersion)?;
        let _reports = Downloader::default()
            .task(version.url())?
            .extract()
            .cache()
            .show_loading_bar()
            .execute()
            .await?;

        // We read the taxonomy file.
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(b'\t')
            .flexible(true)
            .has_headers(true)
            .from_reader(BufReader::new(std::fs::File::open(version.taxonomy_file())?));

        // We iterate over the records.
        for record in csv_reader.deserialize() {
            let record: TaxonomyRow = record?;

            let taxon_entry = CatalogOfLifeTaxonEntryBuilder::default()
                .set_id(record.uid)?
                .set_name(record.name)?
                .set_parent_id(record.parent_uid)?
                .set_rank(record.rank)?
                .build(&self)?;

            self.id_to_position
                .insert(taxon_entry.id.clone(), self.taxon_entries.len() as u32);
            self.name_to_position
                .insert(taxon_entry.name.clone(), self.taxon_entries.len() as u32);
            if record.parent_uid.is_none() {
                if self.root_position.is_some() {
                    return Err(crate::errors::TaxonomyBuilderError::MultipleRoots);
                }
                self.root_position = Some(self.taxon_entries.len() as u32);
            }
            self.taxon_entries.push(taxon_entry);
        }

        // We check that the root position has been set.
        if self.root_position.is_none() {
            return Err(crate::errors::TaxonomyBuilderError::NoRoot);
        }

        // We check for each taxon entry that has a parent that the parent exists.
        for taxon_entry in &self.taxon_entries {
            if let Some(parent_id) = &taxon_entry.parent_id {
                if !self.id_to_position.contains_key(&parent_id) {
                    return Err(crate::errors::TaxonEntryBuilderError::ParentNotFound(
                        taxon_entry.clone(),
                    )
                    .into());
                }
            }

            // TODO! Check that parent has a compatible rank!
        }

        Ok(CatalogOfLifeTaxonomy {
            version,
            root_position: self.root_position.unwrap(),
            taxon_entries: self.taxon_entries,
        })
    }
}
