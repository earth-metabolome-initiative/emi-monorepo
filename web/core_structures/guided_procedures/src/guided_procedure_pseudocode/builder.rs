//! Submodule defining a builder for guided procedures.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::{
    GuidedProcedurePseudocode, ProcedureTemplateGraph,
    guided_procedure_pseudocode::{GPPListener, error::GuidedProcedurePseudocodeError},
    ptg_visitor::PTGVisitor,
};

#[derive(Debug)]
/// Struct representing a builder for guided procedures.
#[derive(Default)]
pub struct GuidedProcedurePseudocodeBuilder<'graph> {
    graph: Option<&'graph ProcedureTemplateGraph>,
}

impl<'graph> GuidedProcedurePseudocodeBuilder<'graph> {
    /// Sets the procedure template graph for the guided procedure.
    pub fn graph(
        mut self,
        graph: &'graph ProcedureTemplateGraph,
    ) -> Result<Self, GuidedProcedurePseudocodeError> {
        if !graph.is_simple_path() {
            return Err(GuidedProcedurePseudocodeError::NotASimplePath);
        }
        self.graph = Some(graph);
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Attributes that can be set in the `GuidedProcedurePseudocode`.
pub enum GuidedProcedurePseudocodeAttribute {
    Graph,
}

impl Display for GuidedProcedurePseudocodeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuidedProcedurePseudocodeAttribute::Graph => write!(f, "graph"),
        }
    }
}

impl Attributed for GuidedProcedurePseudocodeBuilder<'_> {
    type Attribute = GuidedProcedurePseudocodeAttribute;
}

impl IsCompleteBuilder for GuidedProcedurePseudocodeBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.graph.is_some()
    }
}

impl<'graph> Builder for GuidedProcedurePseudocodeBuilder<'graph> {
    type Error = BuilderError<GuidedProcedurePseudocodeAttribute>;
    type Object = GuidedProcedurePseudocode<'graph>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let graph = self
            .graph
            .ok_or(BuilderError::IncompleteBuild(GuidedProcedurePseudocodeAttribute::Graph))?;

        Ok(GuidedProcedurePseudocode { visitor: PTGVisitor::new(graph, GPPListener::new(graph)) })
    }
}
