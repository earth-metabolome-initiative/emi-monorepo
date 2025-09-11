//! Submodule defining the `ProcedureTemplateGraph` structure and its associated
//! methods.

use std::rc::Rc;

use core_structures::{
    NextProcedureTemplate, ParentProcedureTemplate, ProcedureTemplate, ProcedureTemplateAssetModel,
};
use diesel::{associations::HasTable, connection::LoadConnection, helper_types::Asc};
use graph::prelude::DiGraph;

#[derive(Debug, Clone)]
/// Struct providing functionalities to help the user concretely build a
/// procedure.
pub struct ProcedureTemplateGraph {
    /// The procedure template being built.
    procedure_template: ProcedureTemplate,
    /// The task graphs of each procedure template in the hierarchy.
    task_graphs: Vec<Rc<DiGraph<Rc<ProcedureTemplate>>>>,
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

fn load_subprocedure_templates<C: LoadConnection>(
    procedure_template: &ProcedureTemplate,
    conn: &mut C,
) -> Result<
    (Option<Rc<DiGraph<ProcedureTemplate>>>, Vec<(ProcedureTemplate, Option<Rc<DiGraph<ProcedureTemplate>>>)>, Vec<(ProcedureTemplate, ProcedureTemplate)>),
    diesel::result::Error,
> where
<ParentProcedureTemplate as HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
    <core_structures::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent as diesel::expression_methods::EqAll<
        i32,
    >>::Output,
>,
<<ParentProcedureTemplate as HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
    <core_structures::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent as diesel::expression_methods::EqAll<
        i32,
    >>::Output,
>>::Output: diesel::query_dsl::methods::OrderDsl<
    (
        Asc<
            core_structures::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent,
        >,
        Asc<
            core_structures::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child,
        >,
    ),
>,
<<<ParentProcedureTemplate as HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
    <core_structures::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent as diesel::expression_methods::EqAll<
        i32,
    >>::Output,
>>::Output as diesel::query_dsl::methods::OrderDsl<
    (
        Asc<
            core_structures::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent,
        >,
        Asc<
            core_structures::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child,
        >,
    ),
>>::Output: diesel::RunQueryDsl<C>
+ for<'a> diesel::query_dsl::LoadQuery<'a, C, ParentProcedureTemplate>,
ProcedureTemplate: web_common_traits::database::Read<C>,
<NextProcedureTemplate as HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
    <core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent as diesel::expression_methods::EqAll<
        i32,
    >>::Output,
>,
<<NextProcedureTemplate as HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
    <core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent as diesel::expression_methods::EqAll<
        i32,
    >>::Output,
>>::Output: diesel::query_dsl::methods::OrderDsl<
    (
        Asc<
            core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent,
        >,
        Asc<
            core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor,
        >,
        Asc<
            core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor,
        >,
    ),
>,
<<<NextProcedureTemplate as HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
    <core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent as diesel::expression_methods::EqAll<
        i32,
    >>::Output,
>>::Output as diesel::query_dsl::methods::OrderDsl<
    (
        Asc<
            core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent,
        >,
        Asc<
            core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor,
        >,
        Asc<
            core_structures::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor,
        >,
    ),
>>::Output: diesel::RunQueryDsl<C>
    + for<'a> diesel::query_dsl::LoadQuery<'a, C, NextProcedureTemplate>,
{
    
    let mut sub_procedures = Vec::new();
    let mut edges = Vec::new();

    for parent_child_relation in
        ParentProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?
    {
        let child_procedure = parent_child_relation.child(conn)?;
        let (task_graph, child_sub_procedures, child_edges) =
            load_subprocedure_templates(&child_procedure, conn)?;
        sub_procedures.push((child_procedure.clone(), task_graph));
        sub_procedures.extend(child_sub_procedures);
        edges.push((procedure_template.clone(), child_procedure));
        edges.extend(child_edges);
    }

    let task_graph = if sub_procedures.is_empty() {
        None
    } else {
        let mut task_graph_edges = Vec::new();
        for next_procedure_template in
            NextProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?
        {
            let successor = next_procedure_template.successor(conn)?;
            let predecessor = next_procedure_template.predecessor(conn)?;
        }
        todo!()
    };

    Ok((task_graph, sub_procedures, edges))
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
