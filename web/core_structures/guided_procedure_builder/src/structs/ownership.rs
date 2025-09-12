//! Submodule defining a bipartite graph representing procedure templates and
//! the procedure template asset models they reference.

use std::rc::Rc;

use algebra::impls::{CSR2D, GenericBiMatrix2D, SquareCSR2D, UpperTriangularCSR2D};
use core_structures::{ProcedureTemplate, ProcedureTemplateAssetModel};
use graph::{
    prelude::{
        GenericBiGraph, GenericEdgesBuilder, GenericGraph, GenericMonoplexMonopartiteGraphBuilder,
    },
    traits::{BipartiteGraph, EdgesBuilder, MonopartiteGraphBuilder, MonoplexGraphBuilder},
};
use sorted_vec::prelude::SortedVec;
use web_common_traits::prelude::Builder;

#[derive(Debug, Clone)]
/// Bipartite graph representing procedure templates and their asset models.
pub struct Ownership {
    /// Bipartite graph.
    graph: GenericBiGraph<
        Rc<SortedVec<Rc<ProcedureTemplate>>>,
        Rc<SortedVec<ProcedureTemplateAssetModel>>,
        UpperTriangularCSR2D<CSR2D<usize, usize, usize>>,
    >,
    /// Foreign procedure template which were not included in the hierarchy
    /// but are referenced by the procedure templates asset models.
    foreign_procedure_templates: SortedVec<ProcedureTemplate>,
    /// Graph describing the procedure template asset models which are
    /// based on other procedure template asset models.
    derivatives: GenericGraph<
        Rc<SortedVec<ProcedureTemplateAssetModel>>,
        GenericBiMatrix2D<
            SquareCSR2D<CSR2D<u16, usize, usize>>,
            SquareCSR2D<CSR2D<u16, usize, usize>>,
        >,
    >,
}

impl Ownership {
    pub(super) fn new(
        graph: GenericBiGraph<
            Rc<SortedVec<Rc<ProcedureTemplate>>>,
            Rc<SortedVec<ProcedureTemplateAssetModel>>,
            UpperTriangularCSR2D<CSR2D<usize, usize, usize>>,
        >,
        foreign_procedure_templates: SortedVec<ProcedureTemplate>,
    ) -> Self {
        let mut edges = Vec::new();
        let ptams = graph.right_nodes_vocabulary().clone();
        for (i, asset_model) in ptams.iter().enumerate() {
            if let Some(original_ptam) = asset_model.based_on {
                let original_index = ptams
                    .binary_search_by(|am| am.id.cmp(&original_ptam))
                    .expect("Based on asset model not found in vocabulary");
                edges.push((original_index, i));
            }
        }
        let number_of_nodes = ptams.len();
        let directed: SquareCSR2D<CSR2D<u16, usize, usize>> = GenericEdgesBuilder::default()
            .expected_number_of_edges(edges.len() as u16)
            .expected_shape(number_of_nodes)
            .edges(edges)
            .build()
            .expect("Failed to build hierarchy graph");
        let bimatrix = GenericBiMatrix2D::new(directed);
        let derivatives = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(ptams)
            .edges(bimatrix)
            .build()
            .expect("Failed to build hierarchy graph");

        Self { graph, foreign_procedure_templates, derivatives }
    }
}

/// Trait providing read-only access to an `Ownership` structure.
pub trait OwnershipLike: AsRef<Ownership> {
    /// Returns the number of procedure template asset models in the ownership
    /// graph.
    fn number_of_procedure_template_asset_models(&self) -> usize {
        self.as_ref().graph.number_of_right_nodes()
    }

    /// Returns the number of foreign procedure templates in the ownership
    /// graph.
    fn number_of_foreign_procedure_templates(&self) -> usize {
        self.as_ref().foreign_procedure_templates.len()
    }

    /// Returns an iterator over all procedure template asset models in the
    /// ownership graph.
    fn procedure_template_asset_models(
        &self,
    ) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        self.as_ref().graph.right_nodes_vocabulary().iter().map(|am| am.as_ref())
    }

    /// Returns an iterator over all foreign procedure templates in the
    /// ownership graph.
    fn foreign_procedure_templates(&self) -> impl Iterator<Item = &ProcedureTemplate> {
        self.as_ref().foreign_procedure_templates.iter()
    }

    /// Returns the optional reference associated with the foreign procedure
    /// owning the given procedure template asset model, if any.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   whose foreign owning procedure template is to be retrieved.
    fn foreign_procedure_template_of(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> Option<&ProcedureTemplate> {
        self.as_ref()
            .foreign_procedure_templates
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template_asset_model.procedure_template)
            })
            .ok()
            .and_then(|index| self.as_ref().foreign_procedure_templates.get(index))
    }
}

impl<T: AsRef<Ownership>> OwnershipLike for T {}

impl AsRef<Ownership> for Ownership {
    fn as_ref(&self) -> &Ownership {
        self
    }
}
