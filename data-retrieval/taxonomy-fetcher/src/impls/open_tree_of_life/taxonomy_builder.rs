//! Implementation of the taxonomy builder for the Open Tree of Life taxonomy.

use super::OpenTreeOfLifeRank;
use super::{
    taxon_entry::OpenTreeOfLifeTaxonEntry, taxon_entry_builder::OpenTreeOfLifeTaxonEntryBuilder,
    taxonomy::OpenTreeOfLifeTaxonomy, version::OpenTreeOfLifeVersion,
};
use crate::traits::TaxonomyBuilder;
use crate::TaxonEntryBuilder;
use csv::ReaderBuilder;
use downloader::Downloader;
use reqwest::Url;
use serde::Deserialize;
use std::io::BufReader;

#[derive(Default)]
/// Implementation of the taxonomy trait for the Open Tree of Life.
pub struct OpenTreeOfLifeTaxonomyBuilder {
    /// Version of the Open Tree of Life taxonomy to build.
    version: Option<OpenTreeOfLifeVersion>,
    /// Set the directory where the taxonomy is stored.
    directory: Option<std::path::PathBuf>,
    /// Root of the taxonomy.
    root_position: Option<u32>,
    /// Taxon entries.
    taxon_entries: Vec<OpenTreeOfLifeTaxonEntry>,
    /// Hashmap from taxon name to position in taxon entries.
    name_to_position: std::collections::HashMap<String, u32>,
    /// Hashmap from taxon identifier to position in taxon entries.
    id_to_position: std::collections::HashMap<u32, u32>,
}

#[derive(Debug, PartialEq, Eq)]
/// Enum defining the source information of a taxon.

enum SourceInfo {
    Silva(String),
    NCBI(u32),
    Worms(u32),
    GBIF(u32),
    IRMNG(u32),
    Additions(u32, u32, u32),
    Study713(u32),
    H2007,
    IF(u32),
    URL(Url),
}

impl<'de> Deserialize<'de> for SourceInfo {
    fn deserialize<D>(deserializer: D) -> Result<SourceInfo, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        if let Ok(url) = Url::parse(&s) {
            return Ok(SourceInfo::URL(url));
        }

        if !s.contains(':') {
            return Err(serde::de::Error::custom(format!(
                "Invalid source information: '{}'",
                s
            )));
        }

        let mut parts = s.split(':');
        let source = parts.next().unwrap();
        let id = parts.next().unwrap();

        if source.starts_with("additions") {
            let mut parts = source.split('-');
            let _source = parts.next().unwrap();
            let primary_id = u32::from_str_radix(&parts.next().unwrap(), 10)
                .map_err(serde::de::Error::custom)?;
            let secondary_id = u32::from_str_radix(&parts.next().unwrap(), 10)
                .map_err(serde::de::Error::custom)?;
            let tertiary_id = u32::from_str_radix(id, 10).map_err(serde::de::Error::custom)?;
            return Ok(SourceInfo::Additions(primary_id, secondary_id, tertiary_id));
        }

        match source {
            "silva" => Ok(SourceInfo::Silva(id.to_owned())),
            "h2007" => Ok(SourceInfo::H2007),
            "ncbi" | "worms" | "gbif" | "irmng" | "study713" | "if" => {
                let numeric_id: u32 =
                    u32::from_str_radix(&id, 10).map_err(serde::de::Error::custom)?;
                match source {
                    "ncbi" => Ok(SourceInfo::NCBI(numeric_id)),
                    "worms" => Ok(SourceInfo::Worms(numeric_id)),
                    "gbif" => Ok(SourceInfo::GBIF(numeric_id)),
                    "irmng" => Ok(SourceInfo::IRMNG(numeric_id)),
                    "study713" => Ok(SourceInfo::Study713(numeric_id)),
                    "if" => Ok(SourceInfo::IF(numeric_id)),
                    _ => unreachable!(),
                }
            }
            unknown => Err(serde::de::Error::custom(format!(
                "Unknown source: '{}'",
                unknown
            ))),
        }
    }
}

/// Returns a vector of source information from a comma-separated sources.
fn deserialize_comma_separated_sources<'de, D>(deserializer: D) -> Result<Vec<SourceInfo>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s == "null" {
        return Ok(Vec::new());
    }

    s.split(',')
        .map(|source| Deserialize::deserialize(serde::de::value::StrDeserializer::new(source)))
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum OTOLFlag {
    SiblingHigher,
    SiblingLower,
    Infraspecific,
    WasContainer,
    NotOTU,
    IncertaeSedis,
    IncertaeSedisInherited,
    IncertaeSedisDirect,
    Environmental,
    EnvironmentalInherited,
    Barren,
    Merged,
    Extinct,
    ExtinctInherited,
    ExtinctDirect,
    Hidden,
    HiddenInherited,
    Hybrid,
    Inconsistent,
    MajorRankConflict,
    MajorRankConflictInherited,
    MajorRankConflictDirect,
    Unclassified,
    UnclassifiedInherited,
    UnclassifiedDirect,
    Unplaced,
    UnplacedInherited,
    Viral,
    Tattered,
    TatteredInherited,
    Edited,
    Flagged,
    ForcedVisible,
}

impl<'de> Deserialize<'de> for OTOLFlag {
    fn deserialize<D>(deserializer: D) -> Result<OTOLFlag, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "major_rank_conflict_direct" => Ok(OTOLFlag::MajorRankConflictDirect),
            "sibling_higher" => Ok(OTOLFlag::SiblingHigher),
            "sibling_lower" => Ok(OTOLFlag::SiblingLower),
            "infraspecific" => Ok(OTOLFlag::Infraspecific),
            "was_container" => Ok(OTOLFlag::WasContainer),
            "not_otu" => Ok(OTOLFlag::NotOTU),
            "incertae_sedis" => Ok(OTOLFlag::IncertaeSedis),
            "incertae_sedis_inherited" => Ok(OTOLFlag::IncertaeSedisInherited),
            "incertae_sedis_direct" => Ok(OTOLFlag::IncertaeSedisDirect),
            "environmental" => Ok(OTOLFlag::Environmental),
            "environmental_inherited" => Ok(OTOLFlag::EnvironmentalInherited),
            "barren" => Ok(OTOLFlag::Barren),
            "merged" => Ok(OTOLFlag::Merged),
            "extinct" => Ok(OTOLFlag::Extinct),
            "extinct_inherited" => Ok(OTOLFlag::ExtinctInherited),
            "extinct_direct" => Ok(OTOLFlag::ExtinctDirect),
            "hidden" => Ok(OTOLFlag::Hidden),
            "hidden_inherited" => Ok(OTOLFlag::HiddenInherited),
            "hybrid" => Ok(OTOLFlag::Hybrid),
            "inconsistent" => Ok(OTOLFlag::Inconsistent),
            "major_rank_conflict" => Ok(OTOLFlag::MajorRankConflict),
            "major_rank_conflict_inherited" => Ok(OTOLFlag::MajorRankConflictInherited),
            "unclassified" => Ok(OTOLFlag::Unclassified),
            "unclassified_inherited" => Ok(OTOLFlag::UnclassifiedInherited),
            "unclassified_direct" => Ok(OTOLFlag::UnclassifiedDirect),
            "unplaced" => Ok(OTOLFlag::Unplaced),
            "unplaced_inherited" => Ok(OTOLFlag::UnplacedInherited),
            "viral" => Ok(OTOLFlag::Viral),
            "tattered" => Ok(OTOLFlag::Tattered),
            "tattered_inherited" => Ok(OTOLFlag::TatteredInherited),
            "edited" => Ok(OTOLFlag::Edited),
            "flagged" => Ok(OTOLFlag::Flagged),
            "forced_visible" => Ok(OTOLFlag::ForcedVisible),
            _ => Err(serde::de::Error::custom(format!("Unknown flag: '{}'", s))),
        }
    }
}

/// Returns a vector of flags from a comma-separated flags.
fn deserialize_comma_separated_flags<'de, D>(deserializer: D) -> Result<Vec<OTOLFlag>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|flag| Deserialize::deserialize(serde::de::value::StrDeserializer::new(flag)))
        .collect()
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
/// Row in the Open Tree of Life taxonomy.
struct TaxonomyRow {
    /// Unique identifier of the taxon.
    uid: u32,
    /// Unique identifier of the parent taxon.
    parent_uid: Option<u32>,
    /// Name of the taxon.
    name: String,
    /// Rank of the taxon.
    rank: OpenTreeOfLifeRank,
    /// Source information.
    #[serde(deserialize_with = "deserialize_comma_separated_sources")]
    sourceinfo: Vec<SourceInfo>,
    /// Unique name of the taxon.
    uniqname: String,
    /// Flags.
    #[serde(deserialize_with = "deserialize_comma_separated_flags")]
    flags: Vec<OTOLFlag>,
}

impl TaxonomyRow {
    /// Returns whether the current taxon should be skipped.
    fn should_skip(&self) -> bool {
        false
    }
}

impl TaxonomyBuilder for OpenTreeOfLifeTaxonomyBuilder {
    type TaxonEntry = OpenTreeOfLifeTaxonEntry;
    type Taxonomy = OpenTreeOfLifeTaxonomy;
    type TaxonEntryBuilder = OpenTreeOfLifeTaxonEntryBuilder;

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

    fn is_id_in_use(&self, id: &u32) -> bool {
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
            .has_headers(true)
            .from_reader(BufReader::new(std::fs::File::open(version.taxonomy_file())?));

        // We iterate over the records.
        for record in csv_reader.deserialize() {
            let record: TaxonomyRow = record?;

            if record.should_skip() {
                continue;
            }

            let taxon_entry = OpenTreeOfLifeTaxonEntryBuilder::default()
                .set_id(record.uid)?
                .set_name(record.name)?
                .set_parent_id(record.parent_uid)?
                .set_rank(record.rank)?
                .build(&self)?;

            self.id_to_position
                .insert(taxon_entry.id, self.taxon_entries.len() as u32);
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
            if let Some(parent_id) = taxon_entry.parent_id {
                if !self.id_to_position.contains_key(&parent_id) {
                    return Err(crate::errors::TaxonEntryBuilderError::ParentNotFound(
                        taxon_entry.clone(),
                    )
                    .into());
                }
            }

            // TODO! Check that parent has a compatible rank!
        }

        Ok(OpenTreeOfLifeTaxonomy {
            version,
            root_position: self.root_position.unwrap(),
            taxon_entries: self.taxon_entries,
        })
    }
}
