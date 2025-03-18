//! Submodule defining the trait for building a graph.

pub mod bipartite_graph_builder;
pub mod monopartite_graph_builder;
pub mod monoplex_bipartite_graph_builder;
pub mod monoplex_graph_builder;
pub mod monoplex_monopartite_graph_builder;

pub use bipartite_graph_builder::BipartiteGraphBuilder;
pub use monopartite_graph_builder::MonopartiteGraphBuilder;
pub use monoplex_bipartite_graph_builder::MonoplexBipartiteGraphBuilder;
pub use monoplex_graph_builder::MonoplexGraphBuilder;
pub use monoplex_monopartite_graph_builder::MonoplexMonopartiteGraphBuilder;
