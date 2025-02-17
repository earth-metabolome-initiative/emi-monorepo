//! Submodule providing the implementation of the taxonomy trait for the NCBI.

use super::{taxon::NCBITaxon, taxon_entry::NCBITaxonEntry, version::NCBIVersion};
use crate::traits::Taxonomy;

/// Version of the NCBI taxonomy.
pub struct NCBITaxonomy {
    /// Version of the NCBI taxonomy.
    pub version: NCBIVersion,
    /// Root of the taxonomy.
    pub root_position: u32,
    /// Taxon entries.
    pub taxon_entries: Vec<NCBITaxonEntry>,
}

impl Taxonomy for NCBITaxonomy {
    type TaxonEntry = NCBITaxonEntry;
    type Version = NCBIVersion;
    type Taxon<'a> = NCBITaxon<'a>;

    fn name(&self) -> &str {
        "NCBI"
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
            .map(|entry| NCBITaxon { taxon_entry: entry, taxonomy: self })
            .ok_or(crate::errors::TaxonomyError::TaxonNotFound(id.clone()))
    }

    fn root(&self) -> Self::Taxon<'_> {
        NCBITaxon { taxon_entry: &self.taxon_entries[self.root_position as usize], taxonomy: self }
    }

    fn taxons(&self) -> impl Iterator<Item = Self::Taxon<'_>> + '_ {
        self.taxon_entries.iter().map(move |entry| NCBITaxon { taxon_entry: entry, taxonomy: self })
    }
}
