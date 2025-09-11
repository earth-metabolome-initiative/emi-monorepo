//! Submodule defining the `ProcedureTemplateGraph` structure and its associated
//! methods.

use std::rc::Rc;

use core_structures::{
    NextProcedureTemplate, ParentProcedureTemplate, ProcedureTemplate, ProcedureTemplateAssetModel,
};
use diesel::{associations::HasTable, connection::LoadConnection, helper_types::Asc};
use graph::prelude::DiGraph;

use crate::structs::{Hierarchy, TaskGraph};

#[derive(Debug, Clone)]
/// Struct providing functionalities to help the user concretely build a
/// procedure.
pub struct ProcedureTemplateGraph {
    /// The procedure template being built.
    procedure_template: ProcedureTemplate,
    /// The task graphs of each procedure template in the hierarchy.
    /// Leaves of the hierarchy have no task graph, hence the `Option`.
    task_graphs: Vec<Option<TaskGraph>>,
    /// The hierarchy of procedure templates, rooted at the procedure template
    /// being built, and including all its sub-procedure templates.
    hierarchy: Hierarchy,
    /// The foreign procedures referenced by some procedure template asset model
    /// which are not among either the root procedure template or any of its
    /// sub-procedure templates.
    foreign_procedure_templates: Vec<ProcedureTemplate>,
    /// The procedure template asset models associated to the root procedure
    /// template or any of its sub-procedure templates.
    procedure_template_asset_models: Vec<ProcedureTemplateAssetModel>,
}

impl ProcedureTemplateGraph {
    /// Creates a new `ProcedureTemplateGraph` rooted at the given procedure
    /// template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The root procedure template of the graph.
    /// * `conn` - A mutable reference to a database connection.
    ///
    /// # Errors
    ///
    /// * Returns a `diesel::result::Error` if there is an issue querying the
    ///   database.
    pub fn new<C>(
        procedure_template: ProcedureTemplate,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error> {
        // Ok(Self {
        //     procedure_template,
        //     hierarchy: DiGraph::new(),
        //     foreign_procedure_templates: Vec::new(),
        //     procedure_template_asset_models: Vec::new(),
        // })
        todo!()
    }
}
