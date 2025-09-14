//! Submodule defining a bipartite graph representing procedure templates and
//! the procedure template asset models they reference.

use std::rc::Rc;

use algebra::impls::{CSR2D, GenericBiMatrix2D, SquareCSR2D};
use core_structures::{AssetModel, ProcedureTemplate, ProcedureTemplateAssetModel};
use diesel::connection::LoadConnection;
use graph::{
    prelude::{
        GenericBiGraph, GenericEdgesBuilder, GenericGraph, GenericMonoplexMonopartiteGraphBuilder,
    },
    traits::{
        BipartiteGraph, EdgesBuilder, MonopartiteGraphBuilder, MonoplexGraph, MonoplexGraphBuilder,
    },
};
use sorted_vec::prelude::SortedVec;
use web_common_traits::{database::Read, prelude::Builder};

#[derive(Debug, Clone)]
/// Bipartite graph representing procedure templates and their asset models.
pub struct Ownership {
    /// Bipartite graph.
    graph: GenericBiGraph<
        Rc<SortedVec<Rc<ProcedureTemplate>>>,
        Rc<SortedVec<ProcedureTemplateAssetModel>>,
        CSR2D<usize, usize, usize>,
    >,
    /// Foreign procedure template which were not included in the hierarchy
    /// but are referenced by the procedure templates asset models.
    foreign_procedure_templates: SortedVec<ProcedureTemplate>,
    /// Asset models referenced in the procedure template asset models.
    asset_models: Vec<AssetModel>,
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
    pub(super) fn new<C: LoadConnection>(
        graph: GenericBiGraph<
            Rc<SortedVec<Rc<ProcedureTemplate>>>,
            Rc<SortedVec<ProcedureTemplateAssetModel>>,
            CSR2D<usize, usize, usize>,
        >,
        foreign_procedure_templates: SortedVec<ProcedureTemplate>,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error>
    where
        AssetModel: Read<C>,
    {
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

        edges.sort_unstable();
        edges.dedup();

        let asset_models = ptams
            .iter()
            .map(|ptam| ptam.asset_model(conn))
            .collect::<Result<Vec<AssetModel>, diesel::result::Error>>()?;

        let number_of_nodes = ptams.len();
        let directed: SquareCSR2D<CSR2D<u16, usize, usize>> = GenericEdgesBuilder::default()
            .expected_number_of_edges(edges.len() as u16)
            .expected_shape(number_of_nodes)
            .edges(edges)
            .build()
            .expect("Failed to build ownership graph");
        let derivatives = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(ptams)
            .edges(GenericBiMatrix2D::new(directed))
            .build()
            .expect("Failed to build ownership graph");

        Ok(Self { graph, foreign_procedure_templates, asset_models, derivatives })
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

    /// Returns whether the provided procedure template is a foreign procedure
    /// template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to check.
    fn is_foreign_procedure_template(&self, procedure_template: &ProcedureTemplate) -> bool {
        self.as_ref().foreign_procedure_templates.binary_search(procedure_template).is_ok()
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

    /// Returns whether the given procedure template asset model is owned by a
    /// foreign procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   whose ownership status is to be checked.
    fn foreign_owned_ptam(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> bool {
        self.foreign_procedure_template_of(procedure_template_asset_model).is_some()
    }

    /// Returns the procedure template asset models which are owned by the
    /// provided foreign procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The foreign procedure template whose owned
    ///   procedure template asset models are to be retrieved.
    fn foreign_ptams_of(
        &self,
        procedure_template: &ProcedureTemplate,
    ) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        assert!(self.is_foreign_procedure_template(procedure_template));
        self.procedure_template_asset_models()
            .filter(move |ptam| ptam.procedure_template == procedure_template.procedure_template)
    }

    /// Returns the procedure template asset models which are employed by
    /// the provided procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template leaf whose employed
    ///   procedure template asset models are to be retrieved.
    ///
    /// # Panics
    ///
    /// * If the provided procedure template is not part of the ownership graph.
    fn employed_by(
        &self,
        procedure_template: &ProcedureTemplate,
    ) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        let procedure_template_id = self
            .as_ref()
            .graph
            .left_nodes_vocabulary()
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template.procedure_template)
            })
            .expect("Procedure template not part of ownership graph");

        self.as_ref().graph.successors(procedure_template_id).map(|ptam_id| {
            self.as_ref()
                .graph
                .right_nodes_vocabulary()
                .get(ptam_id)
                .expect("Procedure template asset model id out of bounds")
                .as_ref()
        })
    }

    /// Returns a reference to the asset model owned by the given procedure
    /// template asset model.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   whose owned asset model is to be retrieved.
    ///
    /// # Panics
    ///
    /// * If the provided procedure template asset model is not part of the
    ///   ownership graph.
    fn asset_model_of(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> &AssetModel {
        let ptam_id = self
            .as_ref()
            .graph
            .right_nodes_vocabulary()
            .binary_search(procedure_template_asset_model)
            .expect("Procedure template asset model not part of ownership graph");

        self.as_ref().asset_models.get(ptam_id).expect("Asset model id out of bounds")
    }
}

impl<T: AsRef<Ownership>> OwnershipLike for T {}

impl AsRef<Ownership> for Ownership {
    fn as_ref(&self) -> &Ownership {
        self
    }
}
