//! Submodule defining a function to recursively load the sub-procedure
//! templates of a given procedure template.

use std::rc::Rc;

use core_structures::{
    NextProcedureTemplate, ParentProcedureTemplate, ProcedureTemplate,
    codegen::diesel_codegen::tables::{
        next_procedure_templates::next_procedure_templates,
        parent_procedure_templates::parent_procedure_templates,
    },
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

use crate::structs::{Hierarchy, TaskGraph};

impl Hierarchy {
    fn task_graph<C: LoadConnection>(
        &self,
        procedure_template: &ProcedureTemplate,
        conn: &mut C,
    ) -> Result<Option<TaskGraph>, diesel::result::Error>
    where
        <NextProcedureTemplate as HasTable>::Table:
            FilterDsl<<next_procedure_templates::parent as EqAll<i32>>::Output>,
        <<NextProcedureTemplate as HasTable>::Table as FilterDsl<
            <next_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output: OrderDsl<(
            Asc<next_procedure_templates::parent>,
            Asc<next_procedure_templates::predecessor>,
            Asc<next_procedure_templates::successor>,
        )>,
        <<<NextProcedureTemplate as HasTable>::Table as FilterDsl<
            <next_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output as OrderDsl<(
            Asc<next_procedure_templates::parent>,
            Asc<next_procedure_templates::predecessor>,
            Asc<next_procedure_templates::successor>,
        )>>::Output: RunQueryDsl<C> + for<'a> LoadQuery<'a, C, NextProcedureTemplate>,
        ProcedureTemplate: web_common_traits::database::Read<C>,
    {
        if procedure_template.number_of_subprocedure_templates == 0 {
            return Ok(None);
        }

        let nodes = self.hierarchy.node_vocabulary().clone();
        let edges = Vec::new();
        for next_relation in
            NextProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?
        {
            let successor = next_relation.successor(conn)?;
            let predecessor = next_relation.predecessor(conn)?;

            // We find the procedure template from the task graph nodes which
            // is equal to the successor
        }
        todo!()
    }
}
