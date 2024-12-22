//! Submodule providing the implementation of the Taxon entry trait for the Open Tree of Life.

use crate::impls::generic::taxon_entry::GenericTaxonEntry;

use super::rank::OpenTreeOfLifeRank;

/// A taxon entry for the Open Tree of Life taxonomy.
pub type OpenTreeOfLifeTaxonEntry = GenericTaxonEntry<u32, OpenTreeOfLifeRank>;
