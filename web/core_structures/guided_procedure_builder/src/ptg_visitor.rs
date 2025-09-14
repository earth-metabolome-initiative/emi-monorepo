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

    /// Visit a foreign procedure template.
    fn visit_foreign_procedure_template(
        &mut self,
        foreign_procedure_template: &ProcedureTemplate,
    ) -> Result<(), Self::Error>;

    /// Visit a procedure template.
    fn visit_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error>;

    /// Complete the visit of a procedure template.
    fn leave_procedure_template(
        &mut self,
        parents: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error>;

    /// Visit a task step, characterized by having a parent
    /// `ProcedureTemplate` and a predecessor `ProcedureTemplate`.
    fn continue_task(
        &mut self,
        parents: &[&ProcedureTemplate],
        predecessors: &[&ProcedureTemplate],
        child: &ProcedureTemplate,
    ) -> Result<(), Self::Error>;

    /// Visit the procedure template asset model owned by the current
    /// `ProcedureTemplate` leaf in the task.
    fn visit_leaf_ptam(
        &mut self,
        parents: &[&ProcedureTemplate],
        leaf: &ProcedureTemplate,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> Result<(), Self::Error>;

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

impl ProcedureTemplateGraph {
    /// Traverses the `ProcedureTemplateGraph` using the provided visitor.
    pub fn visit<'graph, V: PTGVisitor + 'graph>(
        &'graph self,
        visitor: &mut V,
    ) -> Result<(), V::Error> {
        for foreign_procedure_template in self.foreign_procedure_templates() {
            visitor.visit_foreign_procedure_template(foreign_procedure_template)?;
        }
        self.visit_recursive(visitor, &mut Vec::new(), self.root_procedure_template())
    }

    fn visit_recursive<'graph, V: PTGVisitor + 'graph>(
        &'graph self,
        visitor: &mut V,
        parents: &mut Vec<&'graph ProcedureTemplate>,
        current_node: &'graph ProcedureTemplate,
    ) -> Result<(), V::Error> {
        visitor.visit_procedure_template(parents.as_slice(), current_node)?;
        if let Some(task_graph) = self.task_graph_of(current_node) {
            parents.push(current_node);
            let current_node = task_graph.root_node();
            let mut nodes_to_visit = vec![(Vec::new(), current_node)];
            let mut nodes_to_visit_tmp = Vec::new();

            while !nodes_to_visit.is_empty() {
                for (mut predecessors, node) in nodes_to_visit.drain(..) {
                    self.visit_recursive(visitor, parents, node)?;

                    if task_graph.has_successors(node) {
                        predecessors.push(node);
                        let successors = visitor.filter_successors(
                            parents.as_slice(),
                            predecessors.as_slice(),
                            task_graph.successors(node),
                        )?;

                        for successor in successors {
                            self.visit_recursive(visitor, parents, successor)?;
                            visitor.continue_task(
                                parents.as_slice(),
                                predecessors.as_slice(),
                                successor,
                            )?;
                            nodes_to_visit_tmp.push((predecessors.clone(), successor));
                        }
                    }
                }

                core::mem::swap(&mut nodes_to_visit, &mut nodes_to_visit_tmp);
                nodes_to_visit_tmp.clear();
            }
            parents.pop();
        } else {
            // Leaf node: visit all owned procedure template asset models.
            for ptam in self.employed_by(current_node) {
                visitor.visit_leaf_ptam(parents.as_slice(), current_node, ptam)?;
            }
        }

        visitor.leave_procedure_template(parents.as_slice(), current_node)?;

        Ok(())
    }
}
