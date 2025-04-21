//! Submodule providing the implementation of the taxonomy trait for the Open
//! Tree of Life.

use super::{
    taxon::OpenTreeOfLifeTaxon, taxon_entry::OpenTreeOfLifeTaxonEntry,
    version::OpenTreeOfLifeVersion,
};
use crate::traits::Taxonomy;

/// Version of the Open Tree of Life taxonomy.
pub struct OpenTreeOfLifeTaxonomy {
    /// Version of the Open Tree of Life taxonomy.
    pub version: OpenTreeOfLifeVersion,
    /// Root of the taxonomy.
    pub root_position: usize,
    /// Taxon entries.
    pub taxon_entries: Vec<OpenTreeOfLifeTaxonEntry>,
}

impl Taxonomy for OpenTreeOfLifeTaxonomy {
    type TaxonEntry = OpenTreeOfLifeTaxonEntry;
    type Version = OpenTreeOfLifeVersion;
    type Taxon<'a> = OpenTreeOfLifeTaxon<'a>;

    fn name(&self) -> &'static str {
        "Open Tree of Life"
    }

    fn version(&self) -> Self::Version {
        self.version
    }

    fn taxon_by_id(
        &self,
        id: &<Self::TaxonEntry as crate::traits::TaxonEntry>::Id,
    ) -> Result<
        Self::Taxon<'_>,
        crate::errors::TaxonomyError<<Self::TaxonEntry as crate::traits::TaxonEntry>::Id>,
    > {
        self.taxon_entries
            .iter()
            .find(|entry| &entry.id == id)
            .map(|entry| OpenTreeOfLifeTaxon { taxon_entry: entry, taxonomy: self })
            .ok_or(crate::errors::TaxonomyError::TaxonNotFound(*id))
    }

    fn root(&self) -> Self::Taxon<'_> {
        OpenTreeOfLifeTaxon {
            taxon_entry: &self.taxon_entries[self.root_position],
            taxonomy: self,
        }
    }

    fn taxons(&self) -> impl Iterator<Item = Self::Taxon<'_>> + '_ {
        self.taxon_entries
            .iter()
            .map(move |entry| OpenTreeOfLifeTaxon { taxon_entry: entry, taxonomy: self })
    }
}
