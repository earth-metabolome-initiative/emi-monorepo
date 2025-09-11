//! Submodule defining the `TaskGraph` for a given procedure template.

use std::rc::Rc;

use algebra::impls::{CSR2D, SquareCSR2D};
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
    task_graph: GenericGraph<Rc<SortedVec<Rc<ProcedureTemplate>>>, SquareCSR2D<CSR2D<u8, usize, usize>>>,
}

impl TaskGraph {
    pub(super) fn new(
        nodes: Rc<SortedVec<Rc<ProcedureTemplate>>>,
        edges: SquareCSR2D<CSR2D<u8, usize, usize>>,
    ) -> Self {
        Self {
            task_graph: GenericMonoplexMonopartiteGraphBuilder::default()
                .nodes(nodes)
                .edges(edges)
                .build()
                .expect("Failed to build task graph"),
        }
    }
}
