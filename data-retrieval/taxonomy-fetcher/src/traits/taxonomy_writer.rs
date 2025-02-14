//! Submodule defining a trait to write a taxonomy.

use super::TaxonEntry;
use super::Taxonomy;
use crate::errors::TaxonomyError;
use crate::traits::taxon::Taxon;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufWriter, Write};

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

    /// Wether the file should be compressed or not.
    fn compressed(self) -> Self;

    /// Gets the defined separator.
    fn get_sep(&self) -> u8;

    /// Returns wether the outputed taxonomy is LTREE
    fn is_ltree(&self) -> bool;

    /// Returns wether the compressing option was used or not.
    fn is_compressed(&self) -> bool;

    /// Writes the Taxonomy out.
    fn write(
        &self,
        taxonomy: &Self::Taxonomy,
        path: &str,
    ) -> Result<
        (),
        TaxonomyError<<<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Id>,
    > {
        // Create a boxed writer that abstracts over the underlying writer type
        let writer: Box<dyn Write> = if self.is_compressed() {
            let file = File::create(path)?;
            let buf_writer = BufWriter::new(file);
            let encoder = GzEncoder::new(buf_writer, Compression::default());
            Box::new(encoder)
        } else {
            let file = File::create(path)?;
            Box::new(file)
        };

        // Create the CSV writer using the boxed writer
        let mut csv_writer = csv::WriterBuilder::new()
            .delimiter(self.get_sep())
            .from_writer(writer);

        for taxon in taxonomy.taxons() {
            let taxon_entry: CSVTaxonEntry<<Self::Taxonomy as super::Taxonomy>::TaxonEntry> =
                CSVTaxonEntry {
                    id: *taxon.id(),
                    name: if self.is_ltree() {
                        taxon.ltree_path()
                    } else {
                        taxon.name().to_string()
                    },
                    parent_id: taxon.parent_id().map(|id| *id),
                    rank: taxon.rank().clone(),
                };
            csv_writer.serialize(taxon_entry)?;
        }

        csv_writer.flush()?;

        Ok(())
    }
}
