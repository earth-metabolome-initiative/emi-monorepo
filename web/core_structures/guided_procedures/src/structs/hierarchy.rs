//! Submodule defining the `Hierarchy` struct, which represents the
//! hierarchy of procedure templates rooted at a given procedure template.

use std::rc::Rc;

use algebra::impls::{CSR2D, GenericBiMatrix2D, SquareCSR2D};
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
use graph::{
    prelude::{GenericEdgesBuilder, GenericGraph, GenericMonoplexMonopartiteGraphBuilder},
    traits::{
        EdgesBuilder, MonopartiteGraph, MonopartiteGraphBuilder, MonoplexGraph,
        MonoplexGraphBuilder,
    },
};
use sorted_vec::prelude::SortedVec;
use web_common_traits::prelude::Builder;
mod load_ownership;
mod load_subprocedure_templates;
mod load_task_graph;
use load_subprocedure_templates::load_subprocedure_templates;

#[derive(Debug, Clone)]
#[allow(clippy::type_complexity)]
/// Represents the hierarchy of procedure templates rooted at a given procedure
/// template, including all its sub-procedure templates.
pub struct Hierarchy {
    /// The hierarchy of procedure templates, rooted at the procedure template
    /// being built, and including all its sub-procedure templates.
    hierarchy: GenericGraph<
        Rc<SortedVec<Rc<ProcedureTemplate>>>,
        GenericBiMatrix2D<
            SquareCSR2D<CSR2D<usize, usize, usize>>,
            SquareCSR2D<CSR2D<usize, usize, usize>>,
        >,
    >,
}

impl Hierarchy {
    pub(crate) fn new<C>(
        procedure_template: &ProcedureTemplate,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error>
    where
        C: LoadConnection,
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
        let procedure_template = Rc::new(procedure_template.clone());
        let (mut procedure_nodes, edges) = load_subprocedure_templates(&procedure_template, conn)?;
        procedure_nodes.push(procedure_template_id);
        procedure_nodes.sort_unstable_by(|a, b| a.procedure_template.cmp(&b.procedure_template_id));
        procedure_nodes.dedup();
        let procedure_nodes = SortedVec::try_from(procedure_nodes).unwrap();
        let mut numerical_edges = edges
            .into_iter()
            .map(|(source, destination)| {
                (
                    procedure_nodes
                        .binary_search(&source)
                        .unwrap_or_else(|_| panic!("Source node not found: `{}`", source.name)),
                    procedure_nodes.binary_search(&destination).unwrap_or_else(|_| {
                        panic!("Destination node not found: `{}`", destination.name)
                    }),
                )
            })
            .collect::<Vec<(usize, usize)>>();
        numerical_edges.sort_unstable();
        numerical_edges.dedup();
        let number_of_nodes = procedure_nodes.len();
        let directed: SquareCSR2D<CSR2D<usize, usize, usize>> = GenericEdgesBuilder::default()
            .expected_number_of_edges(numerical_edges.len())
            .expected_shape(number_of_nodes)
            .edges(numerical_edges)
            .build()
            .expect("Failed to build hierarchy graph");
        let bimatrix = GenericBiMatrix2D::new(directed);
        Ok(Self {
            hierarchy: GenericMonoplexMonopartiteGraphBuilder::default()
                .nodes(Rc::new(procedure_nodes))
                .edges(bimatrix)
                .build()
                .expect("Failed to build hierarchy graph"),
        })
    }
}

impl AsRef<Hierarchy> for Hierarchy {
    fn as_ref(&self) -> &Hierarchy {
        self
    }
}

/// A trait for types that can provide access to a `Hierarchy`.
pub trait HierarchyLike: AsRef<Hierarchy> {
    /// Returns a reference to the root procedure template of the hierarchy.
    fn root_procedure_template(&self) -> &ProcedureTemplate {
        self.as_ref().hierarchy.nodes_vocabulary().first().expect("Hierarchy is empty").as_ref()
    }

    /// Returns a reference to the root procedure template name.
    fn root_procedure_template_name(&self) -> &str {
        &self.root_procedure_template().name
    }

    /// Returns whether the provided procedure template is a leaf in the
    /// hierarchy (i.e., it has no sub-procedure templates).
    ///
    /// # Panics
    ///
    /// * Panics if the provided procedure template is not part of the
    ///   hierarchy.
    fn is_leaf(&self, procedure_template: &ProcedureTemplate) -> bool {
        let procedure_template_id = self
            .as_ref()
            .hierarchy
            .nodes_vocabulary()
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template.procedure_template_id)
            })
            .expect("Procedure template not part of hierarchy graph");

        !self.as_ref().hierarchy.has_successors(procedure_template_id)
    }

    /// Returns the internal node identifier for the given procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to get the node
    ///   identifier for.
    ///
    /// # Panics
    ///
    /// * Panics if the provided procedure template is not part of the
    ///   hierarchy.
    fn procedure_node_id(&self, procedure_template: &ProcedureTemplate) -> usize {
        self.as_ref()
            .hierarchy
            .nodes_vocabulary()
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template.procedure_template_id)
            })
            .expect("Procedure template not part of hierarchy graph")
    }
}

impl<T: AsRef<Hierarchy>> HierarchyLike for T {}
