//! Submodule defining the listener used for the `GuidedProcedureBuilder`.

use std::{convert::Infallible, fmt::Debug};

use core_structures::ProcedureTemplate;

use crate::{PTGListener, ProcedureTemplateGraph};
#[derive(Debug, Clone, Copy)]
pub(super) struct GPPListener<'listener> {
    graph: &'listener ProcedureTemplateGraph,
}

impl<'listener> GPPListener<'listener> {
    pub(super) fn new(graph: &'listener ProcedureTemplateGraph) -> Self {
        Self { graph }
    }
}

pub enum GPPListenerOutput<'graph> {
    NoOp,
    Template(&'graph ProcedureTemplate),
}

impl<'graph> PTGListener<'graph> for GPPListener<'graph> {
    type Output = GPPListenerOutput<'graph>;
    type FilteredSuccessors<I>
        = I
    where
        I: Iterator<Item = &'graph core_structures::ProcedureTemplate>;
    type Error = Infallible;

    fn enter_foreign_procedure_template(
        &mut self,
        _foreign_procedure_template: &core_structures::ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        Ok(GPPListenerOutput::NoOp)
    }

    fn continue_task(
        &mut self,
        _parents: &[&core_structures::ProcedureTemplate],
        _predecessors: &[&core_structures::ProcedureTemplate],
        _child: &core_structures::ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn enter_procedure_template(
        &mut self,
        _parents: &[&'graph core_structures::ProcedureTemplate],
        child: &'graph core_structures::ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        if child.most_concrete_table == "procedure_templates" {
            Ok(GPPListenerOutput::NoOp)
        } else {
            Ok(GPPListenerOutput::Template(child))
        }
    }

    fn leave_procedure_template(
        &mut self,
        _parents: &[&core_structures::ProcedureTemplate],
        _child: &core_structures::ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        Ok(GPPListenerOutput::NoOp)
    }

    fn enter_leaf_ptam(
        &mut self,
        _parents: &[&core_structures::ProcedureTemplate],
        _leaf: &core_structures::ProcedureTemplate,
        _procedure_template_asset_model: &core_structures::ProcedureTemplateAssetModel,
    ) -> Result<Self::Output, Self::Error> {
        Ok(GPPListenerOutput::NoOp)
    }

    fn filter_successors<I>(
        &mut self,
        successors: I,
    ) -> Result<Self::FilteredSuccessors<I>, Self::Error>
    where
        I: Iterator<Item = &'graph core_structures::ProcedureTemplate>,
    {
        Ok(successors)
    }
}
