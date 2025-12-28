//! Submodule providing a visitor pattern trait to traverse a
//! `ProcedureTemplateGraph`.

use core_structures::{ProcedureTemplate, ProcedureTemplateAssetModel};

/// Visitor pattern trait for traversing a `ProcedureTemplateGraph`.
pub trait PTGListener<'graph> {
    /// Error enumeration which may occur during the visit.
    type Error: core::error::Error;
    /// The type returned when choosing a successor in a task.
    type FilteredSuccessors<I>: IntoIterator<Item = &'graph ProcedureTemplate>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>;
    /// The object being produced by the visits.
    type Output;

    /// Visit a foreign procedure template.
    ///
    /// # Arguments
    ///
    /// * `foreign_procedure_template` - The foreign `ProcedureTemplate` to
    ///   visit.
    ///
    /// # Errors
    ///
    /// * Returns a `Self::Error` if there is an issue processing the foreign
    ///   procedure template.
    fn enter_foreign_procedure_template(
        &mut self,
        foreign_procedure_template: &'graph ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error>;

    /// Visit a procedure template.
    ///
    /// # Arguments
    ///
    /// * `parents` - The parents of the current `ProcedureTemplate`.
    /// * `child` - The current `ProcedureTemplate` being visited.
    ///
    /// # Errors
    ///
    /// * Returns a `Self::Error` if there is an issue processing the procedure
    ///   template.
    fn enter_procedure_template(
        &mut self,
        parents: &[&'graph ProcedureTemplate],
        child: &'graph ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error>;

    /// Complete the visit of a procedure template.
    ///
    /// # Arguments
    ///
    /// * `parents` - The parents of the current `ProcedureTemplate`.
    /// * `child` - The current `ProcedureTemplate` being exited.
    ///
    /// # Errors
    ///
    /// * Returns a `Self::Error` if there is an issue processing the procedure
    ///   template.
    fn leave_procedure_template(
        &mut self,
        parents: &[&'graph ProcedureTemplate],
        child: &'graph ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error>;

    /// Visit a task step, characterized by having a parent
    /// `ProcedureTemplate` and a predecessor `ProcedureTemplate`.
    ///
    /// # Arguments
    ///
    /// * `parents` - The parents of the current `ProcedureTemplate`.
    /// * `predecessors` - The predecessors of the current `ProcedureTemplate`.
    /// * `child` - The current `ProcedureTemplate` being visited.
    ///
    /// # Errors
    ///
    /// * Returns a `Self::Error` if there is an issue processing the task step.
    fn continue_task(
        &mut self,
        parents: &[&'graph ProcedureTemplate],
        predecessors: &[&'graph ProcedureTemplate],
        child: &'graph ProcedureTemplate,
    ) -> Result<(), Self::Error>;

    /// Visit the procedure template asset model owned by the current
    /// `ProcedureTemplate` leaf in the task.
    ///
    /// # Arguments
    ///
    /// * `parents` - The parents of the current `ProcedureTemplate`.
    /// * `leaf` - The current `ProcedureTemplate` leaf in the task.
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   owned by the current `ProcedureTemplate` leaf in the task.
    ///
    /// # Errors
    ///
    /// * Returns a `Self::Error` if there is an issue processing the procedure
    ///   template asset model.
    fn enter_leaf_ptam(
        &mut self,
        parents: &[&'graph ProcedureTemplate],
        leaf: &'graph ProcedureTemplate,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> Result<Self::Output, Self::Error>;

    /// Requires the visitor to choose which of the possible successors
    /// of the current `ProcedureTemplate` to visit next in the task.
    ///
    /// This is only called if the current `ProcedureTemplate` has more
    /// than one successor in the task.
    ///
    /// # Arguments
    ///
    /// * `successors` - An iterator over the possible successors of the current
    ///   `ProcedureTemplate`.
    ///
    /// # Errors
    ///
    /// * Returns a `Self::Error` if there is an issue choosing the successors.
    fn filter_successors<I>(
        &mut self,
        successors: I,
    ) -> Result<Self::FilteredSuccessors<I>, Self::Error>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>;
}
