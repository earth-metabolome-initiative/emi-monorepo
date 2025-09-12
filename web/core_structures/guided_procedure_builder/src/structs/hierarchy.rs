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
    traits::{EdgesBuilder, MonopartiteGraph, MonopartiteGraphBuilder, MonoplexGraphBuilder},
};
use sorted_vec::prelude::SortedVec;
use web_common_traits::prelude::Builder;
mod load_ownership;
mod load_subprocedure_templates;
mod load_task_graph;
use load_subprocedure_templates::load_subprocedure_templates;

#[derive(Debug, Clone)]
pub(crate) struct Hierarchy {
    /// The hierarchy of procedure templates, rooted at the procedure template
    /// being built, and including all its sub-procedure templates.
    hierarchy: GenericGraph<
        Rc<SortedVec<Rc<ProcedureTemplate>>>,
        GenericBiMatrix2D<
            SquareCSR2D<CSR2D<u16, usize, usize>>,
            SquareCSR2D<CSR2D<u16, usize, usize>>,
        >,
    >,
}

impl Hierarchy {
    pub(crate) fn new<C: LoadConnection>(
        procedure_template: &ProcedureTemplate,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error>
    where
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
        let (mut procedure_nodes, edges) = load_subprocedure_templates(procedure_template, conn)?;
        procedure_nodes.sort_unstable();
        procedure_nodes.dedup();
        let procedure_nodes = SortedVec::try_from(procedure_nodes).unwrap();
        let numerical_edges = edges
            .into_iter()
            .map(|(source, destination)| {
                (
                    procedure_nodes.binary_search(&source).expect("Source node not found"),
                    procedure_nodes
                        .binary_search(&destination)
                        .expect("Destination node not found"),
                )
            })
            .collect::<Vec<(usize, usize)>>();
        let number_of_nodes = procedure_nodes.len();
        let directed: SquareCSR2D<CSR2D<u16, usize, usize>> = GenericEdgesBuilder::default()
            .expected_number_of_edges(numerical_edges.len() as u16)
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

pub trait HierarchyLike: AsRef<Hierarchy> {
    /// Returns a reference to the root procedure template of the hierarchy.
    fn root_procedure_template(&self) -> &ProcedureTemplate {
        self.as_ref().hierarchy.nodes_vocabulary().first().expect("Hierarchy is empty").as_ref()
    }
}
