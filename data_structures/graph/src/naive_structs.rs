//! Submodule providing naively implemented data structures for graphs.
//!
//! These data structures are not optimized for performance, but are useful for
//! some general use cases where performance is not critical.

pub mod directory_tree;
pub mod generic_bigraph;
pub mod generic_edges_builder;
pub mod generic_graph;
pub mod generic_monoplex_bipartite_graph_builder;
pub mod generic_vocabulary_builder;
pub mod named_types;
pub mod undirected_edges_builder;

pub use directory_tree::DirectoryTree;
pub use generic_bigraph::GenericBiGraph;
pub use generic_edges_builder::GenericEdgesBuilder;
pub use generic_graph::GenericGraph;
pub use generic_monoplex_bipartite_graph_builder::GenericMonoplexBipartiteGraphBuilder;
pub use generic_vocabulary_builder::GenericVocabularyBuilder;
pub use named_types::*;
pub use undirected_edges_builder::GenericUndirectedMonopartiteEdgesBuilder;
