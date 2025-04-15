//! Submodule implementing the `TaxonEntryBuilder` trait for the Catalog of Life
//! taxonomy.

/// Implementation of the taxon entry builder for the Catalog of Life taxonomy.
pub type CatalogOfLifeTaxonEntryBuilder =
    crate::impls::generic::taxon_entry_builder::GenericTaxonEntryBuilder<
        super::COLId,
        super::rank::CatalogOfLifeRank,
    >;
