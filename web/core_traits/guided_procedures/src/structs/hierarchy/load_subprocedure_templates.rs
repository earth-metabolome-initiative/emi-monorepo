//! Submodule defining a function to recursively load the sub-procedure
//! templates of a given procedure template.

use std::rc::Rc;

use core_structures::{
    ParentProcedureTemplate, ProcedureTemplate,
    codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates,
};
use diesel::{
    RunQueryDsl,
    associations::HasTable,
    connection::LoadConnection,
    dsl::Asc,
    expression_methods::EqAll,
    query_dsl::{
        LoadQuery,
        methods::{FilterDsl, OrderDsl},
    },
};

#[allow(clippy::type_complexity)]
/// Recursively loads all sub-procedure templates of the given procedure
/// template, returning a vector of all loaded sub-procedure templates and a
/// vector of edges representing the parent-child relationships between them.
///
/// # Arguments
///
/// * `procedure_template` - The procedure template whose sub-procedure
///   templates are to be loaded.
/// * `conn` - The database connection to use for loading the sub-procedure
///   templates.
pub(super) fn load_subprocedure_templates<C: LoadConnection>(
    procedure_template: &Rc<ProcedureTemplate>,
    conn: &mut C,
) -> Result<
    (Vec<Rc<ProcedureTemplate>>, Vec<(Rc<ProcedureTemplate>, Rc<ProcedureTemplate>)>),
    diesel::result::Error,
>
where
    <ParentProcedureTemplate as HasTable>::Table:
        FilterDsl<<parent_procedure_templates::parent as EqAll<i32>>::Output>,
    <<ParentProcedureTemplate as HasTable>::Table as FilterDsl<
        <parent_procedure_templates::parent as EqAll<i32>>::Output,
    >>::Output:
        OrderDsl<(Asc<parent_procedure_templates::parent>, Asc<parent_procedure_templates::child>)>,
    <<<ParentProcedureTemplate as HasTable>::Table as FilterDsl<
        <parent_procedure_templates::parent as EqAll<i32>>::Output,
    >>::Output as OrderDsl<(
        Asc<parent_procedure_templates::parent>,
        Asc<parent_procedure_templates::child>,
    )>>::Output: RunQueryDsl<C> + for<'a> LoadQuery<'a, C, ParentProcedureTemplate>,
    ProcedureTemplate: web_common_traits::database::Read<C>,
{
    let mut subprocedure_templates = Vec::new();
    let mut edges = Vec::new();

    for parent_child_relation in
        ParentProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?
    {
        let child_procedure = Rc::from(parent_child_relation.child(conn)?);
        let (child_subprocedure_templates, child_edges) =
            load_subprocedure_templates(&child_procedure, conn)?;
        subprocedure_templates.extend(child_subprocedure_templates);
        edges.push((procedure_template.clone(), child_procedure.clone()));
        subprocedure_templates.push(child_procedure);
        edges.extend(child_edges);
    }

    Ok((subprocedure_templates, edges))
}
