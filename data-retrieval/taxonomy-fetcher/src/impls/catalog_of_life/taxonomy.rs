//! Submodule providing the implementation of the taxonomy trait for the Open
//! Tree of Life.

use super::{
    taxon::CatalogOfLifeTaxon, taxon_entry::CatalogOfLifeTaxonEntry, version::CatalogOfLifeVersion,
};
use crate::traits::Taxonomy;

#[derive(Debug)]
/// Version of the Open Tree of Life taxonomy.
pub struct CatalogOfLifeTaxonomy {
    /// Version of the Open Tree of Life taxonomy.
    pub version: CatalogOfLifeVersion,
    /// Root of the taxonomy.
    pub root_position: u32,
    /// Taxon entries.
    pub taxon_entries: Vec<CatalogOfLifeTaxonEntry>,
}

impl Taxonomy for CatalogOfLifeTaxonomy {
    type TaxonEntry = CatalogOfLifeTaxonEntry;
    type Version = CatalogOfLifeVersion;
    type Taxon<'a> = CatalogOfLifeTaxon<'a>;

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
            .map(|entry| CatalogOfLifeTaxon { taxon_entry: entry, taxonomy: self })
            .ok_or(crate::errors::TaxonomyError::TaxonNotFound(*id))
    }

    fn root(&self) -> Self::Taxon<'_> {
        CatalogOfLifeTaxon {
            taxon_entry: &self.taxon_entries[self.root_position as usize],
            taxonomy: self,
        }
    }

    fn taxons(&self) -> impl Iterator<Item = Self::Taxon<'_>> + '_ {
        self.taxon_entries
            .iter()
            .map(move |entry| CatalogOfLifeTaxon { taxon_entry: entry, taxonomy: self })
    }
}
