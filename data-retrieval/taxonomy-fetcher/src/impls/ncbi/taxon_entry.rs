//! Submodule providing the implementation of the Taxon entry trait for the NCBI.

use crate::impls::generic::taxon_entry::GenericTaxonEntry;

use super::rank::NCBIRank;

/// A taxon entry for the NCBI taxonomy.
pub type NCBITaxonEntry = GenericTaxonEntry<u32, NCBIRank>;
