//! Implementation of the taxonomy builder for the Open Tree of Life taxonomy.

use crate::traits::TaxonomyBuilder;

use super::{
    taxon_entry::OpenTreeOfLifeTaxonEntry, taxon_entry_builder::OpenTreeOfLifeTaxonEntryBuilder,
    taxonomy::OpenTreeOfLifeTaxonomy, version::OpenTreeOfLifeVersion,
};

/// Implementation of the taxonomy trait for the Open Tree of Life.
pub struct OpenTreeOfLifeTaxonomyBuilder {
    /// Version of the Open Tree of Life taxonomy to build.
    version: Option<OpenTreeOfLifeVersion>,
    /// Set the directory where the taxonomy is stored.
    directory: Option<std::path::PathBuf>,
    /// Root of the taxonomy.
    root_id: Option<u32>,
    /// Taxon entries.
    taxon_entries: Vec<OpenTreeOfLifeTaxonEntry>,
    /// Hashmap from taxon name to position in taxon entries.
    name_to_position: std::collections::HashMap<String, u32>,
    /// Hashmap from taxon identifier to position in taxon entries.
    id_to_position: std::collections::HashMap<u32, u32>,
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

    fn build(
        self,
    ) -> Result<
        Self::Taxonomy,
        crate::errors::TaxonomyBuilderError<<Self::TaxonEntry as crate::traits::TaxonEntry>::Id>,
    > {
        unimplemented!("OpenTreeOfLifeTaxonomyBuilder::build")
    }
}
