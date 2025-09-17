//! Submodule defining commonly used named types for the generic graph data
//! structures.

use algebra::impls::{
    CSR2D, GenericBiMatrix2D, SquareCSR2D, SymmetricCSR2D, UpperTriangularCSR2D, ValuedCSR2D,
};
use sorted_vec::prelude::SortedVec;

use super::{
    GenericBiGraph, GenericEdgesBuilder, GenericGraph, GenericUndirectedMonopartiteEdgesBuilder,
};

/// Type alias for a generic directed graph.
pub type DiGraph<NodeSymbol> =
    GenericGraph<SortedVec<NodeSymbol>, SquareCSR2D<CSR2D<usize, usize, usize>>>;
/// Type alias for a generic directed bipartite graph with bidirectional edges.
pub type DiBiGraph<NodeSymbol> = GenericGraph<
    SortedVec<NodeSymbol>,
    GenericBiMatrix2D<
        SquareCSR2D<CSR2D<usize, usize, usize>>,
        SquareCSR2D<CSR2D<usize, usize, usize>>,
    >,
>;
/// Type alias for a generic directed edges list builder.
pub type DiEdgesBuilder<EdgeIterator> =
    GenericEdgesBuilder<EdgeIterator, SquareCSR2D<CSR2D<usize, usize, usize>>>;
/// Type alias for a generic directed bipartite edges list builder with
/// bidirectional edges.
pub type DiBiEdgesBuilder<EdgeIterator> = GenericEdgesBuilder<
    EdgeIterator,
    GenericBiMatrix2D<
        SquareCSR2D<CSR2D<usize, usize, usize>>,
        SquareCSR2D<CSR2D<usize, usize, usize>>,
    >,
>;

/// Type alias for a generic directed bipartite graph.
pub type BiGraph<LeftNodeSymbol, RightNodeSymbol> = GenericBiGraph<
    SortedVec<LeftNodeSymbol>,
    SortedVec<RightNodeSymbol>,
    CSR2D<usize, usize, usize>,
>;
/// Type alias for a generic directed bipartite edges list builder.
pub type BiEdgesBuilder<EdgeIterator> =
    GenericEdgesBuilder<EdgeIterator, UpperTriangularCSR2D<CSR2D<usize, usize, usize>>>;

/// Type alias for a generic weighted directed bipartite graph.
pub type WeightedBiGraph<LeftNodeSymbol, RightNodeSymbol> = GenericBiGraph<
    SortedVec<LeftNodeSymbol>,
    SortedVec<RightNodeSymbol>,
    ValuedCSR2D<usize, usize, usize, f64>,
>;

/// Type alias for a generic undirected graph.
pub type UndiGraph<NodeSymbol> =
    GenericGraph<SortedVec<NodeSymbol>, SymmetricCSR2D<CSR2D<usize, usize, usize>>>;
/// Type alias for a generic undirected edges list builder.
pub type UndiEdgesBuilder<EdgeIterator> = GenericUndirectedMonopartiteEdgesBuilder<
    EdgeIterator,
    UpperTriangularCSR2D<CSR2D<usize, usize, usize>>,
    SymmetricCSR2D<CSR2D<usize, usize, usize>>,
>;
