//! Submodule providing the implementation of the taxon trait for NCBI.

use super::{taxon_entry::NCBITaxonEntry, taxonomy::NCBITaxonomy};
use crate::traits::Taxon;

/// Entry of a taxon in the Open Tree of Life taxonomy.
pub struct NCBITaxon<'a> {
    /// Entry of the taxon in the Open Tree of Life taxonomy.
    pub taxon_entry: &'a NCBITaxonEntry,
    /// Reference to the parent Taxonomy
    pub taxonomy: &'a NCBITaxonomy,
}

impl<'a> Taxon<'a> for NCBITaxon<'a> {
    type Taxonomy = NCBITaxonomy;

    fn entry(&self) -> &'a <Self::Taxonomy as crate::traits::Taxonomy>::TaxonEntry {
        self.taxon_entry
    }

    fn taxonomy(&self) -> &'a Self::Taxonomy {
        self.taxonomy
    }
}
