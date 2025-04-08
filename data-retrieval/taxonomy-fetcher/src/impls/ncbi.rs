//! Submodule for the NCBI taxonomy fetcher implementation.

pub mod rank;
pub mod taxon;
pub mod taxon_entry;
pub mod taxon_entry_builder;
pub mod taxonomy;
pub mod taxonomy_builder;
pub mod version;

pub use rank::NCBIRank;
pub use taxon::NCBITaxon;
pub use taxon_entry::NCBITaxonEntry;
pub use taxon_entry_builder::NCBITaxonEntryBuilder;
pub use taxonomy::NCBITaxonomy;
pub use taxonomy_builder::NCBITaxonomyBuilder;
pub use version::NCBIVersion;
