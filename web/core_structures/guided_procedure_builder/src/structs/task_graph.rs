//! Submodule defining the `TaskGraph` for a given procedure template.

use std::rc::Rc;

use algebra::{
    impls::{CSR2D, GenericBiMatrix2D, SquareCSR2D},
    prelude::Kahn,
};
use core_structures::ProcedureTemplate;
use graph::{
    prelude::{GenericGraph, GenericMonoplexMonopartiteGraphBuilder},
    traits::{MonopartiteGraphBuilder, MonoplexGraphBuilder},
};
use sorted_vec::prelude::SortedVec;
use web_common_traits::prelude::Builder;
#[derive(Debug, Clone)]
pub(crate) struct TaskGraph {
    /// The task graph of the procedure template being built.
    task_graph: GenericGraph<
        SortedVec<Rc<ProcedureTemplate>>,
        GenericBiMatrix2D<
            SquareCSR2D<CSR2D<u8, usize, usize>>,
            SquareCSR2D<CSR2D<u8, usize, usize>>,
        >,
    >,
    /// A Kahn topological ordering of the procedure templates in the task
    /// graph.
    topological_ordering: Vec<usize>,
}

impl TaskGraph {
    pub(super) fn new(
        nodes: SortedVec<Rc<ProcedureTemplate>>,
        edges: GenericBiMatrix2D<
            SquareCSR2D<CSR2D<u8, usize, usize>>,
            SquareCSR2D<CSR2D<u8, usize, usize>>,
        >,
    ) -> Self {
        let topological_ordering = edges.kahn().expect("The task graph has cycles");

        let task_graph: GenericGraph<
            SortedVec<Rc<ProcedureTemplate>>,
            GenericBiMatrix2D<
                SquareCSR2D<CSR2D<u8, usize, usize>>,
                SquareCSR2D<CSR2D<u8, usize, usize>>,
            >,
        > = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(nodes)
            .edges(edges)
            .build()
            .expect("Failed to build task graph");

        Self { task_graph, topological_ordering }
    }
}
