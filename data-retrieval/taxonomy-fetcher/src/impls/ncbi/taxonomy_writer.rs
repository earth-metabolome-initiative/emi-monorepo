//! Implementation of the taxonomy builder for the NCBI taxonomy.

use super::taxonomy::NCBITaxonomy;
use crate::traits::TaxonomyWriter;

#[derive(Default)]
/// Implementation of the taxonomy trait for the NCBI.
pub struct NCBITaxonomyWriter {
    /// Separator for the tabular export of the NCBI taxonomy
    separator: Option<u8>,
    /// Wether the Taxonomy should be written as LTREE or not.
    ltree: bool,
    /// Wether the output file should be compressed or not.
    compressed: bool,
}

impl TaxonomyWriter for NCBITaxonomyWriter {
    type Taxonomy = NCBITaxonomy;

    fn separator(self, separator: u8) -> Self {
        Self {
            separator: Some(separator),
            ..self
        }
    }

    fn ltree(self) -> Self {
        Self {
            ltree: true,
            ..self
        }
    }

    fn compressed(self) -> Self {
        Self {
            compressed: true,
            ..self
        }
    }

    fn get_sep(&self) -> u8 {
        self.separator.unwrap_or(b'\t')
    }

    fn is_ltree(&self) -> bool {
        self.ltree
    }

    fn is_compressed(&self) -> bool {
        self.compressed
    }
}
