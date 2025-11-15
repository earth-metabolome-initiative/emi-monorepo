//! Submodule defining the `GuidedProcedureBuilder` structure and its associated
//! methods.

mod builder;
mod error;
mod iterator;
mod listener;
pub use builder::GuidedProcedureBuilder;
use listener::GPBListener;

use crate::{procedure_template_graph::ProcedureTemplateGraph, ptg_visitor::PTGVisitor};
#[derive(Debug)]
/// Struct providing functionalities to help the user concretely build a
/// procedure.
pub struct GuidedProcedure<'graph, C> {
    visitor: PTGVisitor<'graph, ProcedureTemplateGraph, GPBListener<'graph, C>>,
}

impl<'graph, C> GuidedProcedure<'graph, C> {
    /// Creates a new `GuidedProcedureBuilder` instance.
    #[must_use]
    pub fn new() -> GuidedProcedureBuilder<'graph, C> {
        GuidedProcedureBuilder::default()
    }
}
