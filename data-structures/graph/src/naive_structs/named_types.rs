//! Submodule defining commonly used named types for the generic graph data
//! structures.

use algebra::impls::{SquareCSR2D, SymmetricCSR2D, UpperTriangularCSR2D};
use sorted_vec::prelude::SortedVec;

use super::{GenericDirectedEdgesBuilder, GenericDirectionalGraph, GenericUndirectedEdgesBuilder};

/// Type alias for a generic directed graph.
pub type DiGraph<NodeSymbol> =
    GenericDirectionalGraph<SortedVec<NodeSymbol>, SquareCSR2D<usize, usize>>;
/// Type alias for a generic directed edges list builder.
pub type DiEdgesBuilder<EdgeIterator> =
    GenericDirectedEdgesBuilder<EdgeIterator, SquareCSR2D<usize, usize>>;

/// Type alias for a generic undirected graph.
pub type UndiGraph<NodeSymbol> =
    GenericDirectionalGraph<SortedVec<NodeSymbol>, SymmetricCSR2D<usize, usize>>;
/// Type alias for a generic undirected edges list builder.
pub type UndiEdgesBuilder<EdgeIterator> = GenericUndirectedEdgesBuilder<
    EdgeIterator,
    UpperTriangularCSR2D<usize, usize>,
    SymmetricCSR2D<usize, usize>,
>;
