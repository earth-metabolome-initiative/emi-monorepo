//! Submodule defining the `TaskGraph` for a given procedure template.

use std::rc::Rc;

use algebra::impls::{CSR2D, GenericBiMatrix2D, SquareCSR2D};
use core_structures::ProcedureTemplate;
use graph::{
    prelude::{GenericGraph, GenericMonoplexMonopartiteGraphBuilder, RootNodes, SinkNodes},
    traits::{
        MonopartiteGraph, MonopartiteGraphBuilder, MonoplexGraph, MonoplexGraphBuilder, SimplePath,
        TransposedMonoplexGraph,
    },
};
use sorted_vec::prelude::SortedVec;
use web_common_traits::prelude::Builder;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the task graph of a procedure template, including its nodes,
/// edges, and a Kahn topological ordering of the nodes.
pub struct TaskGraph {
    /// The task graph of the procedure template being built.
    task_graph: GenericGraph<
        SortedVec<Rc<ProcedureTemplate>>,
        GenericBiMatrix2D<
            SquareCSR2D<CSR2D<u8, usize, usize>>,
            SquareCSR2D<CSR2D<u8, usize, usize>>,
        >,
    >,
    /// The root node ID of the task graph.
    root_node_id: usize,
    /// The sink node IDs of the task graph.
    sink_node_ids: Vec<usize>,
}

impl TaskGraph {
    pub(super) fn new(
        nodes: SortedVec<Rc<ProcedureTemplate>>,
        edges: GenericBiMatrix2D<
            SquareCSR2D<CSR2D<u8, usize, usize>>,
            SquareCSR2D<CSR2D<u8, usize, usize>>,
        >,
    ) -> Self {
        assert!(!nodes.is_empty(), "The task graph must have at least one node");

        let task_graph: GenericGraph<
            SortedVec<Rc<ProcedureTemplate>>,
            GenericBiMatrix2D<
                SquareCSR2D<CSR2D<u8, usize, usize>>,
                SquareCSR2D<CSR2D<u8, usize, usize>>,
            >,
        > = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(nodes)
            .edges(edges)
            .build()
            .expect("Failed to build task graph");

        let root_nodes = task_graph.root_nodes();
        let sink_nodes = task_graph.sink_nodes();

        assert_eq!(root_nodes.len(), 1, "The task graph must have exactly one root node");

        Self { task_graph, root_node_id: root_nodes[0], sink_node_ids: sink_nodes }
    }

    /// Returns whether the graph is a simple path (i.e., a linear sequence of
    /// nodes).
    pub fn is_simple_path(&self) -> bool {
        self.task_graph.is_simple_path()
    }

    /// Returns the root node of the task graph.
    pub fn root_node(&self) -> &ProcedureTemplate {
        &self.task_graph.nodes_vocabulary()[self.root_node_id]
    }

    /// Returns the sink nodes of the task graph.
    pub fn sink_nodes(&self) -> impl Iterator<Item = &ProcedureTemplate> {
        self.sink_node_ids.iter().map(|&id| self.task_graph.nodes_vocabulary()[id].as_ref())
    }

    /// Returns whether the provided procedure template has successors in the
    /// task graph.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to check for successors.
    ///
    /// # Panics
    ///
    /// * Panics if the provided procedure template is not part of the task
    ///   graph.
    pub fn has_successors(&self, procedure_template: &ProcedureTemplate) -> bool {
        let procedure_template_id = self
            .task_graph
            .nodes_vocabulary()
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template.procedure_template)
            })
            .expect("Procedure template not part of task graph");
        self.task_graph.has_successors(procedure_template_id)
    }

    /// Returns an iterator over the successors of the given procedure
    /// template in the task graph.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to get the successors
    ///   for.
    ///
    /// # Panics
    ///
    /// * Panics if the provided procedure template is not part of the task
    ///   graph.
    pub fn successors(
        &self,
        procedure_template: &ProcedureTemplate,
    ) -> impl Iterator<Item = &ProcedureTemplate> {
        let procedure_template_id = self
            .task_graph
            .nodes_vocabulary()
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template.procedure_template)
            })
            .expect("Procedure template not part of task graph");
        self.task_graph
            .successors(procedure_template_id)
            .map(|id| self.task_graph.nodes_vocabulary()[id].as_ref())
    }

    /// Returns whether the provided procedure template has predecessors in the
    /// task graph.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to check for
    ///   predecessors.
    ///
    /// # Panics
    ///
    /// * Panics if the provided procedure template is not part of the task
    ///   graph.
    pub fn has_predecessors(&self, procedure_template: &ProcedureTemplate) -> bool {
        let procedure_template_id = self
            .task_graph
            .nodes_vocabulary()
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template.procedure_template)
            })
            .expect("Procedure template not part of task graph");
        self.task_graph.has_predecessors(procedure_template_id)
    }

    /// Returns an iterator over the predecessors of the given procedure
    /// template in the task graph.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to get the predecessors
    ///   for.
    ///
    /// # Panics
    ///
    /// * Panics if the provided procedure template is not part of the task
    ///   graph.
    pub fn predecessors(
        &self,
        procedure_template: &ProcedureTemplate,
    ) -> impl Iterator<Item = &ProcedureTemplate> {
        let procedure_template_id = self
            .task_graph
            .nodes_vocabulary()
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template.procedure_template)
            })
            .expect("Procedure template not part of task graph");
        self.task_graph
            .predecessors(procedure_template_id)
            .map(|id| self.task_graph.nodes_vocabulary()[id].as_ref())
    }
}
