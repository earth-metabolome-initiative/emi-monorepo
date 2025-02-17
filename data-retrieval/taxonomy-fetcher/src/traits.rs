//! Submodule providing the traits used across the taxonomy-fetcher crate.

pub mod rank;
pub mod taxon;
pub mod taxon_entry;
pub mod taxon_entry_builder;
pub mod taxon_identifier;
pub mod taxon_topological_iterator;
pub mod taxon_version;
pub mod taxonomy;
pub mod taxonomy_builder;

pub use rank::Rank;
pub use taxon::Taxon;
pub use taxon_entry::TaxonEntry;
pub use taxon_entry_builder::TaxonEntryBuilder;
pub use taxon_identifier::TaxonIdentifier;
pub use taxon_topological_iterator::TaxonTopologicalIterator;
pub use taxon_version::TaxonVersion;
pub use taxonomy::Taxonomy;
pub use taxonomy_builder::TaxonomyBuilder;
