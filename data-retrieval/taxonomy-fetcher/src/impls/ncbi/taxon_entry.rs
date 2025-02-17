//! Submodule providing the implementation of the Taxon entry trait for the
//! NCBI.

use super::rank::NCBIRank;
use crate::impls::generic::taxon_entry::GenericTaxonEntry;

/// A taxon entry for the NCBI taxonomy.
pub type NCBITaxonEntry = GenericTaxonEntry<u32, NCBIRank>;
