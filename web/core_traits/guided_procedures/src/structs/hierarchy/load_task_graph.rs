//! Submodule defining a function to recursively load the sub-procedure
//! templates of a given procedure template.

use std::rc::Rc;

use algebra::impls::GenericBiMatrix2D;
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
use graph::{
    prelude::{GenericEdgesBuilder, MonopartiteGraph},
    traits::EdgesBuilder,
};
use sorted_vec::prelude::SortedVec;
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
        <ParentProcedureTemplate as HasTable>::Table:
            FilterDsl<<parent_procedure_templates::parent as EqAll<i32>>::Output>,
        <<ParentProcedureTemplate as HasTable>::Table as FilterDsl<
            <parent_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output: OrderDsl<(
            Asc<parent_procedure_templates::parent>,
            Asc<parent_procedure_templates::child>,
        )>,
        <<<ParentProcedureTemplate as HasTable>::Table as FilterDsl<
            <parent_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output as OrderDsl<(
            Asc<parent_procedure_templates::parent>,
            Asc<parent_procedure_templates::child>,
        )>>::Output: RunQueryDsl<C> + for<'a> LoadQuery<'a, C, ParentProcedureTemplate>,
        ProcedureTemplate: web_common_traits::database::Read<C>,
    {
        let parent_child_relations =
            ParentProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?;

        if parent_child_relations.is_empty() {
            return Ok(None);
        }

        let mut nodes = parent_child_relations
            .iter()
            .map(|relation| {
                let child = relation.child(conn)?;

                // We find the curresponding Rc<ProcedureTemplate> in the hierarchy's
                // nodes vocabulary so to avoid duplicating memory allocation in the
                // finalized data structure.
                let rc_child = self
                    .hierarchy
                    .nodes_vocabulary()
                    .binary_search_by(|pt| pt.as_ref().cmp(&child))
                    .map(|index| self.hierarchy.nodes_vocabulary()[index].clone())
                    .expect("Child procedure template not found in hierarchy's vocabulary");

                Ok(rc_child)
            })
            .collect::<Result<Vec<Rc<ProcedureTemplate>>, diesel::result::Error>>()?;

        nodes.sort_unstable();

        let sorted_nodes = SortedVec::try_from(nodes).unwrap();

        let next_relations =
            NextProcedureTemplate::from_parent(procedure_template.procedure_template, conn)?;

        let mut edges: Vec<(usize, usize)> = Vec::new();
        for next_relation in next_relations {
            let predecessor = next_relation.predecessor(conn)?;
            let successor = next_relation.successor(conn)?;

            let source_index = sorted_nodes
                .binary_search_by(|pt| pt.as_ref().cmp(&predecessor))
                .expect("Predecessor not found in nodes vocabulary");
            let destination_index = sorted_nodes
                .binary_search_by(|pt| pt.as_ref().cmp(&successor))
                .expect("Successor not found in nodes vocabulary");
            edges.push((source_index, destination_index));
        }

        edges.sort_unstable();

        let number_of_nodes = sorted_nodes.len();
        let edges = GenericEdgesBuilder::default()
            .expected_number_of_edges(edges.len())
            .expected_shape(number_of_nodes)
            .edges(edges)
            .build()
            .expect("Failed to build task graph");

        let biedges = GenericBiMatrix2D::new(edges);

        Ok(Some(TaskGraph::new(sorted_nodes, biedges)))
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
    pub(crate) fn task_graphs<C: LoadConnection>(
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
        <ParentProcedureTemplate as HasTable>::Table:
            FilterDsl<<parent_procedure_templates::parent as EqAll<i32>>::Output>,
        <<ParentProcedureTemplate as HasTable>::Table as FilterDsl<
            <parent_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output: OrderDsl<(
            Asc<parent_procedure_templates::parent>,
            Asc<parent_procedure_templates::child>,
        )>,
        <<<ParentProcedureTemplate as HasTable>::Table as FilterDsl<
            <parent_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output as OrderDsl<(
            Asc<parent_procedure_templates::parent>,
            Asc<parent_procedure_templates::child>,
        )>>::Output: RunQueryDsl<C> + for<'a> LoadQuery<'a, C, ParentProcedureTemplate>,
        ProcedureTemplate: web_common_traits::database::Read<C>,
    {
        let mut task_graphs = Vec::with_capacity(self.hierarchy.number_of_nodes());
        for procedure_template in self.hierarchy.nodes_vocabulary().iter() {
            task_graphs.push(self.task_graph(procedure_template, conn)?);
        }
        Ok(task_graphs)
    }
}
