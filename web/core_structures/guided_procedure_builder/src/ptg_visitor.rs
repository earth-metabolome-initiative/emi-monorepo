//! Submodule providing a visitor pattern trait to traverse a
//! `ProcedureTemplateGraph`.

use core_structures::{ProcedureTemplate, ProcedureTemplateAssetModel};

use crate::{
    ProcedureTemplateGraph,
    structs::{HierarchyLike, OwnershipLike},
};

/// Visitor pattern trait for traversing a `ProcedureTemplateGraph`.
pub trait PTGVisitor {
    /// Error enumeration which may occur during the visit.
    type Error: core::error::Error;
    /// The type returned when choosing a successor in a task.
    type FilteredSuccessors<'graph, I>: IntoIterator<Item = &'graph ProcedureTemplate>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>;
    /// The object being produced by the visits.
    type Output;

    /// Visit a foreign procedure template.
    fn visit_foreign_procedure_template(
        &mut self,
        foreign_procedure_template: &ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error>;

    /// Visit a procedure template.
    fn visit_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error>;

    /// Complete the visit of a procedure template.
    fn leave_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error>;

    /// Visit a task step, characterized by having a parent
    /// `ProcedureTemplate` and a predecessor `ProcedureTemplate`.
    fn continue_task(
        &mut self,
        parents: &[&ProcedureTemplate],
        predecessors: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error>;

    /// Visit the procedure template asset model owned by the current
    /// `ProcedureTemplate` leaf in the task.
    fn visit_leaf_ptam(
        &mut self,
        parents: &[&ProcedureTemplate],
        leaf: &ProcedureTemplate,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> Result<Self::Output, Self::Error>;

    /// Requires the visitor to choose which of the possible successors
    /// of the current `ProcedureTemplate` to visit next in the task.
    ///
    /// This is only called if the current `ProcedureTemplate` has more
    /// than one successor in the task.
    fn filter_successors<'graph, I>(
        &mut self,
        parents: &[&ProcedureTemplate],
        predecessors: &[&ProcedureTemplate],
        successors: I,
    ) -> Result<Self::FilteredSuccessors<'graph, I>, Self::Error>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>;
}
