//! Submodule providing the implementation of the taxon trait for the Catalog of Life.

use crate::traits::Taxon;

use super::{taxon_entry::CatalogOfLifeTaxonEntry, taxonomy::CatalogOfLifeTaxonomy};

/// Entry of a taxon in the Catalog of Life taxonomy.
pub struct CatalogOfLifeTaxon<'a> {
    /// Entry of the taxon in the Catalog of Life taxonomy.
    pub taxon_entry: &'a CatalogOfLifeTaxonEntry,
    /// Reference to the parent Taxonomy
    pub taxonomy: &'a CatalogOfLifeTaxonomy,
}

impl<'a> Taxon<'a> for CatalogOfLifeTaxon<'a> {
    type Taxonomy = CatalogOfLifeTaxonomy;

    fn entry(&self) -> &'a <Self::Taxonomy as crate::traits::Taxonomy>::TaxonEntry {
        self.taxon_entry
    }

    fn taxonomy(&self) -> &'a Self::Taxonomy {
        self.taxonomy
    }
}
