//! Submodule defining a function to recursively load the sub-procedure
//! templates of a given procedure template.

use core_structures::{
    NextProcedureTemplate, ProcedureTemplate,
    codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates,
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
use graph::{
    prelude::{GenericEdgesBuilder, MonopartiteGraph},
    traits::EdgesBuilder,
};
use web_common_traits::prelude::Builder;

use crate::structs::{Hierarchy, TaskGraph};

impl Hierarchy {
    /// Returns the task graph for the given procedure template, if it has
    /// sub-procedure templates; otherwise, returns `None`.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template for which to load the
    ///   task graph.
    /// * `conn` - The database connection to use for loading the task graph.
    ///
    /// # Errors
    ///
    /// * Returns a `diesel::result::Error` if there is an error loading the
    ///   task graph from the database.
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

        let nodes = self.hierarchy.nodes_vocabulary().clone();
        let number_of_nodes = nodes.len();
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for next_relation in
            NextProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?
        {
            let successor = next_relation.successor(conn)?;
            let predecessor = next_relation.predecessor(conn)?;

            let source_index = nodes
                .binary_search_by(|pt| pt.as_ref().cmp(&successor))
                .expect("Successor not found in nodes vocabulary");
            let destination_index = nodes
                .binary_search_by(|pt| pt.as_ref().cmp(&predecessor))
                .expect("Predecessor not found in nodes vocabulary");
            edges.push((source_index, destination_index));
        }

        Ok(Some(TaskGraph::new(
            nodes,
            GenericEdgesBuilder::default()
                .expected_number_of_edges(edges.len() as u8)
                .expected_shape(number_of_nodes)
                .edges(edges)
                .build()
                .expect("Failed to build task graph"),
        )))
    }

    /// Returns the task graphs for all procedure templates in the hierarchy.
    /// Each task graph corresponds to a procedure template in the hierarchy,
    /// in the same order as the procedure templates in the hierarchy's nodes
    /// vocabulary.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection to use for loading the task graphs.
    ///
    /// # Errors
    ///
    /// * Returns a `diesel::result::Error` if there is an error loading the
    ///   task graphs from the database.
    fn task_graphs<C: LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Option<TaskGraph>>, diesel::result::Error>
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
        let mut task_graphs = Vec::with_capacity(self.hierarchy.number_of_nodes());
        for procedure_template in self.hierarchy.nodes_vocabulary().iter() {
            task_graphs.push(self.task_graph(procedure_template, conn)?);
        }
        Ok(task_graphs)
    }
}
