//! Submodule providing naively implemented data structures for graphs.
//!
//! These data structures are not optimized for performance, but are useful for
//! some general use cases where performance is not critical.

pub mod generic_vocabulary_builder;
pub mod generic_edges_builder;
pub mod directed_edges_builder;
pub mod undirected_edges_builder;

pub use generic_vocabulary_builder::GenericVocabularyBuilder;
pub use generic_edges_builder::GenericEdgesBuilder;
pub use directed_edges_builder::GenericDirectedEdgesBuilder;
pub use undirected_edges_builder::GenericUndirectedEdgesBuilder;
