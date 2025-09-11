//! Submodule defining the `Hierarchy` struct, which represents the
//! hierarchy of procedure templates rooted at a given procedure template.

use std::rc::Rc;

use algebra::impls::{CSR2D, GenericBiMatrix2D, SquareCSR2D};
use core_structures::ProcedureTemplate;
use graph::prelude::{DiBiGraph, GenericGraph};
use sorted_vec::prelude::SortedVec;

mod load_subprocedure_templates;
mod load_task_graph;

#[derive(Debug, Clone)]
pub(crate) struct Hierarchy {
    /// The hierarchy of procedure templates, rooted at the procedure template
    /// being built, and including all its sub-procedure templates.
    hierarchy: GenericGraph<
        Rc<SortedVec<Rc<ProcedureTemplate>>>,
        GenericBiMatrix2D<SquareCSR2D<CSR2D<u16, u16, u16>>, SquareCSR2D<CSR2D<u16, u16, u16>>>,
    >,
}

impl Hierarchy {
    pub(crate) fn new<C>(procedure_template: &ProcedureTemplate, conn: &mut C) -> Self {
        todo!()
    }
}
