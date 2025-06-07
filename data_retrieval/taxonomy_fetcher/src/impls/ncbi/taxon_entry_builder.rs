//! Submodule implementing the `TaxonEntryBuilder` trait for the NCBI taxonomy.

/// Implementation of the taxon entry builder for the NCBI taxonomy.
pub type NCBITaxonEntryBuilder =
    crate::impls::generic::taxon_entry_builder::GenericTaxonEntryBuilder<
        u32,
        super::rank::NCBIRank,
    >;
