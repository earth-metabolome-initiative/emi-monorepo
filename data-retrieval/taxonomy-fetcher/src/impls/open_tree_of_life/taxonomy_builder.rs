//! Implementation of the taxonomy builder for the Open Tree of Life taxonomy.

use super::OpenTreeOfLifeRank;
use super::{
    taxon_entry::OpenTreeOfLifeTaxonEntry, taxon_entry_builder::OpenTreeOfLifeTaxonEntryBuilder,
    taxonomy::OpenTreeOfLifeTaxonomy, version::OpenTreeOfLifeVersion,
};
use crate::traits::TaxonomyBuilder;
use crate::utils::separator_fixed_reader::SeparatorFixedReader;
use crate::TaxonEntryBuilder;
use csv::ReaderBuilder;
use downloader::Downloader;
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
    root_position: Option<usize>,
    /// Taxon entries.
    taxon_entries: Vec<OpenTreeOfLifeTaxonEntry>,
    /// Hashmap from taxon name to position in taxon entries.
    name_to_position: std::collections::HashMap<String, u32>,
    /// Hashmap from taxon identifier to position in taxon entries.
    id_to_position: std::collections::HashMap<u32, u32>,
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
    sourceinfo: String,
    /// Unique name of the taxon.
    uniqname: String,
    /// Flags.
    flags: String,
}

impl TaxonomyRow {
    /// Returns true if the rank is a no-rank terminal.
    fn is_no_rank_terminal(&self) -> bool {
        self.rank.is_no_rank_terminal()
    }

    /// Returns true if the taxon is uncultured.
    fn is_uncultured(&self) -> bool {
        self.name == "uncultured"
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

        // We create a reader where we replace the odd delimiter
        // that is used in the Open Tree of Life taxonomy, i.e. '\t|\t',
        // by a comma, in stream mode.
        let reader = SeparatorFixedReader::new(
            BufReader::new(std::fs::File::open(version.taxonomy_file())?),
            "\t",
            "\t|\t",
        );

        // We read the taxonomy file.
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(true)
            .from_reader(reader);

        // We iterate over the records.
        for record in csv_reader.deserialize() {
            let record: TaxonomyRow = record?;

            if record.is_no_rank_terminal() || record.is_uncultured() {
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
                self.root_position = Some(self.taxon_entries.len());
            }
            self.taxon_entries.push(taxon_entry);
        }

        // We check for each taxon entry that has a parent that the parent exists.
        for taxon_entry in &self.taxon_entries {
            if let Some(parent_id) = taxon_entry.parent_id {
                if !self.id_to_position.contains_key(&parent_id) {
                    return Err(
                        crate::errors::TaxonEntryBuilderError::ParentNotFound(taxon_entry.clone()).into(),
                    );
                }
            }

            // TODO! Check that parent has a compatible rank!
        }

        unimplemented!("OpenTreeOfLifeTaxonomyBuilder::build")
    }
}
