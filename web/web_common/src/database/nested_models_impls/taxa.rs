//! Method to build the Taxon name and rank for a Taxon

use crate::database::Taxa;
use std::fmt::Display;

impl Taxa {
    pub fn taxon_display(&self) -> String {
        let taxon_display = self.name.clone();
        // if let Some(ncbi_taxon_id) = &self.ncbi_taxon_id {
        //     taxon_display.push_str(&format!("NCBI ID: {}", ncbi_taxon_id));
        // }
        taxon_display
    }
}

impl Display for Taxa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.taxon_display())
    }
}