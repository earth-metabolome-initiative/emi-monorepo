//! Submodule implementing the traits for the Open Tree of Life taxonomy.

pub mod rank;
pub mod taxon;
pub mod taxon_entry;
pub mod taxon_entry_builder;
pub mod taxonomy;
pub mod taxonomy_builder;
pub mod version;

pub use rank::OpenTreeOfLifeRank;
pub use taxon::OpenTreeOfLifeTaxon;
pub use taxon_entry::OpenTreeOfLifeTaxonEntry;
pub use taxon_entry_builder::OpenTreeOfLifeTaxonEntryBuilder;
pub use taxonomy::OpenTreeOfLifeTaxonomy;
pub use taxonomy_builder::OpenTreeOfLifeTaxonomyBuilder;
pub use version::OpenTreeOfLifeVersion;

