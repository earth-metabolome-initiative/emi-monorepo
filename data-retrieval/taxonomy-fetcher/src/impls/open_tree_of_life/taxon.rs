//! Submodule providing the implementation of the taxon trait for the Open Tree of Life.

use crate::traits::Taxon;

use super::{taxon_entry::OpenTreeOfLifeTaxonEntry, taxonomy::OpenTreeOfLifeTaxonomy};

/// Entry of a taxon in the Open Tree of Life taxonomy.
pub struct OpenTreeOfLifeTaxon<'a> {
    /// Entry of the taxon in the Open Tree of Life taxonomy.
    pub taxon_entry: &'a OpenTreeOfLifeTaxonEntry,
    /// Reference to the parent Taxonomy
    pub taxonomy: &'a OpenTreeOfLifeTaxonomy,
}

impl<'a> Taxon<'a> for OpenTreeOfLifeTaxon<'a> {
    type Taxonomy = OpenTreeOfLifeTaxonomy;

    fn entry(&self) -> &'a <Self::Taxonomy as crate::traits::Taxonomy>::TaxonEntry {
        self.taxon_entry
    }

    fn taxonomy(&self) -> &'a Self::Taxonomy {
        self.taxonomy
    }
}
