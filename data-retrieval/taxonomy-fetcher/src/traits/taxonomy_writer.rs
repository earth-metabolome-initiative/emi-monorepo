//! Submodule defining a trait to write a taxonomy.

use serde::Serialize;
use super::TaxonEntry;
use super::Taxonomy;
use crate::traits::taxon::Taxon;
use crate::errors::TaxonomyError;

#[derive(Debug, Serialize)]
struct CSVTaxonEntry<TE: TaxonEntry> {
    id: TE::Id,
    name: String,
    parent_id: Option<TE::Id>,
    #[serde(rename = "ranks.name")]
    rank: TE::Rank,
}

/// The TaxonomyWriter should be able to write out a file with the content of a Taxonomy

pub trait TaxonomyWriter: Default {
    /// Type of the taxonomy to be written
    type Taxonomy: super::Taxonomy;

    /// Sets the separator to be used for the output document
    fn separator(self, separator: u8) -> Self;

    /// Wether the Taxonomy should be written as LTREE or not.
    fn ltree(self) -> Self;

    /// Gets the defined separator.
    fn get_sep(&self) -> u8;

    /// Returns wether the outputed taxonomy is LTREE
    fn is_ltree(&self) -> bool;

    /// Writes the Taxonomy out.
    fn write(
        &self,
        taxonomy: &Self::Taxonomy,
        path: &str,
    ) -> Result<(), TaxonomyError<<<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Id>> {

        /// First we create a writer_buffer from the path
        /// Next we create a compressed writer_buffer using crate::flate2 (BuffWriter)
        /// Now we provide the from_writer method of WriterBuilder 
    
        let mut writer = csv::WriterBuilder::new()
        .delimiter(self.get_sep())
        .from_path(path)?;

        


        for taxon in taxonomy.taxons() {
            let taxon_entry: CSVTaxonEntry<<Self::Taxonomy as super::Taxonomy>::TaxonEntry> = CSVTaxonEntry {
                id: *taxon.id(),
                name: if self.is_ltree() { 
                    taxon.ltree_path()
                } else {
                    taxon.name().to_string()
                },
                parent_id: taxon.parent_id().map(|id| *id),
                rank: taxon.rank().clone(),
            };
            writer.serialize(taxon_entry)?;
        }

        writer.flush()?;

        Ok(())

    }

}