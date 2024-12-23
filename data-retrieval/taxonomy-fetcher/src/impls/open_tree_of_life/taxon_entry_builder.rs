//! Submodule implementing the TaxonEntryBuilder trait for the Open Tree of Life taxonomy.

/// Implementation of the taxon entry builder for the Open Tree of Life taxonomy.
pub type OpenTreeOfLifeTaxonEntryBuilder =
    crate::impls::generic::taxon_entry_builder::GenericTaxonEntryBuilder<
        u32,
        crate::impls::open_tree_of_life::rank::OpenTreeOfLifeRank,
    >;
